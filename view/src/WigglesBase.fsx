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
open Socket

type ConnectionState =
    | Waiting
    | Open
    | Closed

type SavesAvailable = {saves: string list; autosaves: string list}

type BaseModel<'m, 'msg> = {
    /// The name of the currently-running show.
    name: string
    /// Saved states available for this show.
    savesAvailable: SavesAvailable
    /// Saved shows available for this console.
    showsAvailable: string list
    /// Pop-over modal dialog.  Shared among everything that needs it.
    modalDialog: Modal.Model
    /// App navigation bar.
    navbar: Navbar.Model
    /// The function we can call to get a fresh, empty version of this console.
    showModelInit: unit -> 'm
    /// The collection of messages to emit to initialize the show.
    showInitMessages: Cmd<'msg>
}

let initBaseModel showModelInit showInitMessages = {
    name = ""
    savesAvailable = {saves = []; autosaves = []}

type Model<'m, 'msg> = {
    /// State of the connection to the console server.
    connection: ConnectionState
    /// The basic model pieces that every console uses.
    baseModel: BaseModel<'m, 'msg>
    /// The specific model used by this console.
    showModel: 'm
}

[<RequireQualifiedAccess>]
/// Outer wrapper for console command. Generic over the top-level command type used by an
/// implementation.
type ServerCommand<'msg> =
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
    | Console of 'msg

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
type Message<'cmd, 'rsp, 'msg> =
    /// Message to the server, sent over the socket connection.
    | Command of ServerCommand<'cmd>
    /// Message from the server.
    | Response of ServerResponse<'rsp>
    /// Navbar actions
    | Navbar of Navbar.Message
    /// Modal dialog actions
    | Modal of Modal.Message
    /// Message for the internal operation of this console view.
    | Console of 'msg

// Launch the websocket we'll use to talk to the server.
let (subscription, send) = openSocket Message.Response

/// Update the state of this application based on an incoming server message.
let updateFromResponse message model =
    match message with
    | ServerResponse.ShowName(name) ->
        {model with baseModel = {model.baseModel with name = name}}, Cmd.none
    | ServerResponse.SavesAvailable(s) ->
        {model withbaseModel = {model.baseModel with savesAvailable = s}}, Cmd.none
    | ServerResponse.Loaded(name) ->
        // we loaded a new show; we need to issue whatever commands are necessary to completely
        // refresh the state of this application
        // We do this by refreshing to the initial model state and issuing the messages it requires
        // to initialize itself.

let update message model =
    match message with
    | Message.Command(cmd) = send cmd
    | Message.Response(rsp) = updateFromResponse rsp model



/// Outer view wrapper to show splashes if we're waiting on a server connection or if the connection
/// has gone away.
let viewWithConnection model dispatch =
    match model.connection with
    | Waiting -> Modal.viewSplash "Waiting for console server connection to be established."
    | Open -> view model dispatch
    | Closed -> Modal.viewSplash "The console server disconnected."