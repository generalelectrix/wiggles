/// Re-usable pieces for consoles built around the Wiggles console server.
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Types.fsx"
#load "Bootstrap.fsx"
#load "Modal.fsx"
#load "Socket.fsx"
#load "Navbar.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Types
open Bootstrap

type ConnectionState =
    | Waiting
    | Open
    | Closed

type SavesAvailable = {saves: string list; autosaves: string list}

type BaseModel<'msg> = {
    /// The name of the currently-running show.
    name: string
    /// Saved states available for this show.
    savesAvailable: SavesAvailable
    /// Saved shows available for this console.
    showsAvailable: string list
    /// Pop-over modal dialog.  Shared among everything that needs it.
    modalDialog: Modal.Model
    /// App navigation bar.
    navbar: Navbar.Model<'msg>
}

let private initBaseModel navbar = {
    name = ""
    savesAvailable = {saves = []; autosaves = []}
    showsAvailable = []
    modalDialog = Modal.initialModel()
    navbar = navbar
}

type Model<'m, 'msg> = {
    /// State of the connection to the console server.
    connection: ConnectionState
    /// The basic model pieces that every console uses.
    baseModel: BaseModel<'msg>
    /// The specific model used by this console.
    showModel: 'm
}

let initModel navbar showModel = {
    connection = Waiting
    baseModel = initBaseModel navbar
    showModel = showModel
}

type LoadSpec =
    /// Load the latest saved state.
    | Latest
    /// Load this particular saved state.  Should consist only of a timestamp with no extension.
    | Exact of string
    /// Load the latest autosave.
    | LatestAutosave
    /// Load this particular autosave.  Should consist only of a timestamp with no extension.
    | ExactAutosave of string

type LoadShow = {
    name: string
    spec: LoadSpec
}

[<RequireQualifiedAccess>]
/// Outer wrapper for console command. Generic over the top-level command type used by an
/// implementation.
type ServerCommand<'m> =
    /// Get the name of the running show.
    | ShowName
    /// Create a new, empty show.
    | NewShow of string
    /// List all available shows.
    | SavedShows
    /// List all available saves and autosaves.
    | AvailableSaves
    /// Load a show using this spec.
    | Load of LoadShow
    /// Save the current state of the show.
    | Save
    /// Save the current show as a new show with a different name.  This show will become the one
    /// running in the reactor.
    | SaveAs of string
    /// Change the name of the currently-running show.  This will move all of the files in the
    /// saved show library.
    | Rename of string
    /// Quit the console, cleanly closing down every running thread.
    | Quit
    /// A message to be passed into the console logic running in the reactor.
    | Console of 'm

[<RequireQualifiedAccess>]
/// Outer wrapper for console response.  Generic over the top-level response type used by an
/// implementation.
type ServerResponse<'msg> =
    /// The name of the running show.
    | ShowName of string
    /// A listing of all available save and autosave files for the running show.
    | SavesAvailable of SavesAvailable
    /// Listing of all saved shows for this console.
    | ShowsAvailable of string list
    /// A new show was loaded, with this name.
    | Loaded of string
    /// The running show's name changed.
    | Renamed of string
    /// The show was saved successfully.
    | Saved
    /// A show library error occurred.
    | ShowLibErr of string
    /// The console is going to quit.
    | Quit
    /// A response emanting from the console itself.
    | Console of 'msg


[<RequireQualifiedAccess>]
/// Outer message type.
type Message<'cmd, 'rsp, 'msg> =
    /// The connection state of the application has changed.
    | Socket of Socket.SocketMessage
    /// Message to the server, sent over the socket connection.
    | Command of ServerCommand<'cmd>
    /// Message from the server.
    | Response of ServerResponse<'rsp>
    /// Navbar actions
    | Navbar of Navbar.Message
    /// Modal dialog actions
    | Modal of Modal.Message
    /// Message for the internal operation of this console view.
    | Inner of 'msg
   
/// The initial commands to fire to initialize a base Wiggles console.
let initCommands = [ServerCommand.ShowName]

/// Return a command that opens a modal dialog with a message and a button to dismiss it.
let private prompt msg = msg |> Modal.prompt |> Modal.Open |> Message.Modal |> Cmd.ofMsg

/// Update the state of this application based on an incoming server message.
let private updateFromResponse wrapShowResponse updateShow message model =
    match message with
    | ServerResponse.ShowName(name) ->
        {model with baseModel = {model.baseModel with name = name}}, Cmd.none
    | ServerResponse.SavesAvailable(s) ->
        {model with baseModel = {model.baseModel with savesAvailable = s}}, Cmd.none
    | ServerResponse.Loaded(name) ->
        // For the moment blow up on show load
        failwith "A new show was loaded but view reloading is not implemented yet."
    | ServerResponse.Renamed(name) ->
        {model with baseModel = {model.baseModel with name = name}}, Cmd.none
    | ServerResponse.Saved ->
        model, prompt "The show has been saved."
    | ServerResponse.ShowLibErr(msg) ->
        model, prompt (sprintf "A show library error occurred: %s" msg)
    | ServerResponse.Quit ->
        // Don't do anything in response to it, allow the socket connection closure to trigger
        // the next action.
        printf "The server sent the quit message.  The socket connection should close."
        model, Cmd.none
    | ServerResponse.Console(m) ->
        let showModel, showMessages = updateShow (wrapShowResponse m) model.showModel
        {model with showModel = showModel}, showMessages |> Cmd.map Message.Inner

/// Update the whole view by processing a message.
let update initCommands socketSend wrapShowResponse updateShow message model =
    match message with
    | Message.Socket(Connected) ->
        {model with connection = Open}, initCommands |> Cmd.map Message.Command
    | Message.Socket(Disconnected) ->
        {model with connection = Closed}, Cmd.none
    | Message.Command(msg) ->
        socketSend msg
        model, Cmd.none
    | Message.Response(msg) -> updateFromResponse wrapShowResponse updateShow msg model
    | Message.Navbar(msg) ->
        {model with
            baseModel = {model.baseModel with
                navbar = Navbar.update msg model.baseModel.navbar}}, Cmd.none
    | Message.Modal(msg) ->
        {model with
            baseModel = {model.baseModel with
                modalDialog = Modal.update msg model.baseModel.modalDialog}}, Cmd.none
    | Message.Inner(msg) ->
        let showModel, showMessages = updateShow msg model.showModel
        {model with showModel = showModel}, showMessages |> Cmd.map Message.Inner

/// View the basic page structure including the navbar and modal if its open.
/// Delegate the rest of the view to the console.
let private viewInner viewShow model dispatch =
    let openModal req = req |> Modal.Message.Open |> Message.Modal |> dispatch
    R.div [] [
        Navbar.view model.baseModel.navbar dispatch (Message.Navbar >> dispatch)
        R.div [Container.Fluid] [
            viewShow
                openModal
                model.showModel
                (Message.Inner >> dispatch) // show dispatches a message to itself
                (ServerCommand.Console >> Message.Command >> dispatch) // show dispatches a message to the server
            Modal.view model.baseModel.modalDialog (Message.Modal >> dispatch)
        ]
    ]

/// Outer view wrapper to show splashes if we're waiting on a server connection or if the connection
/// has gone away.
let view viewShow model dispatch =
    match model.connection with
    | Waiting -> Modal.viewSplash "Waiting for console server connection to be established."
    | Open -> viewInner viewShow model dispatch
    | Closed -> Modal.viewSplash "The console server disconnected."