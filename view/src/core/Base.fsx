/// Re-usable core view logic and host communication for consoles built around the Wiggles console server.
module Base
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Bootstrap.fsx"
#load "Modal.fsx"
#load "Socket.fsx"
#load "Navbar.fsx"
#load "Types.fsx"
#load "Slider.fsx"
#load "LoadShow.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Bootstrap
open Types

type UtilPage =
    | ShowLoader

let commandsForUtilPageChange page =
    match page with
    | ShowLoader ->
        // emit a command to update our collection of available shows
        [(Exclusive, ServerCommand.SavedShows)]


/// Basic model pieces used by a view application.
type BaseModel<'msg> = {
    /// The name of the currently-running show.
    name: string
    /// Saved states available for this show.
    savesAvailable: SavesAvailable
    /// Saved shows available for this console.
    showsAvailable: string list
    /// Which utility page is open, if any.
    utilPage: UtilPage option
    /// Tool to load shows.
    showLoader: LoadShow.Model
    /// Pop-over modal dialog.  Shared among everything that needs it.
    modalDialog: Modal.Model
    /// App navigation bar.
    navbar: Navbar.Model<'msg>
}

/// Top-level model type for a wiggles view application.
type Model<'m, 'msg> = {
    /// State of the connection to the console server.
    connection: ConnectionState
    /// The basic model pieces that every console uses.
    baseModel: BaseModel<'msg>
    /// The specific model used by this console.
    showModel: 'm
}

/// Helper function to lift a tuple of filter and message up the message hierarchy.
let liftResponseAndFilter f (filter, message) = (filter, f message)

let private initBaseModel navbar = {
    name = ""
    savesAvailable = {saves = []; autosaves = []}
    showsAvailable = []
    utilPage = None
    showLoader = LoadShow.initModel()
    modalDialog = Modal.initialModel()
    navbar = navbar
}
let initModel navbar showModel = {
    connection = Waiting
    baseModel = initBaseModel navbar
    showModel = showModel
}

[<RequireQualifiedAccess>]
/// Top-level message type.
type Message<'cmd, 'rsp, 'msg> =
    /// The connection state of the application has changed.
    | Socket of Socket.SocketMessage
    /// Message to the server, sent over the socket connection.
    | Command of ResponseFilter * ServerCommand<'cmd>
    /// Message from the server.
    | Response of ServerResponse<'rsp>
    /// Action to set which utility page is open, if any.
    | UtilPage of UtilPage option
    /// Navbar actions
    | Navbar of Navbar.Message
    /// Modal dialog actions
    | Modal of Modal.Message
    /// Show loader
    | ShowLoader of LoadShow.Message
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
    | ServerResponse.ShowsAvailable(shows) -> 
        {model with baseModel = {model.baseModel with showsAvailable = shows}}, Cmd.none
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
        printfn "The server sent the quit message.  The socket connection should close."
        model, Cmd.none
    | ServerResponse.Console(m) ->
        let showModel, showMessages = updateShow (wrapShowResponse m) model.showModel
        {model with showModel = showModel}, showMessages |> Cmd.map Message.Inner

/// Update the whole view by processing a message.
let update initCommands socketSend wrapShowResponse updateShow message model =
    let updateBaseModel f = {model with baseModel = f model.baseModel}
    match message with
    | Message.Socket Socket.Connected ->
        {model with connection = Open}, initCommands |> Cmd.map Message.Command
    | Message.Socket Socket.Disconnected ->
        {model with connection = Closed}, Cmd.none
    | Message.Command(filter, msg) ->
        socketSend (filter, msg)
        model, Cmd.none
    | Message.Response(msg) -> updateFromResponse wrapShowResponse updateShow msg model
    | Message.UtilPage(page) ->
        // Update the model with the newly-selected page.
        let newModel = updateBaseModel (fun bm -> {bm with utilPage = page})
        // We may optionally emit some server commands to retrieve updated state for the utility
        // upon opening it.
        let commands =
            match page with
            | Some(p) -> commandsForUtilPageChange p
            | None -> []
            |> List.map (Message.Command >> Cmd.ofMsg)
            |> Cmd.batch

        newModel, commands
    | Message.Navbar(msg) ->
        let newModel =
            updateBaseModel (fun bm -> {bm with navbar = Navbar.update msg bm.navbar})
        newModel, Cmd.none
    | Message.Modal(msg) ->
        let newModel =
            updateBaseModel (fun bm -> {bm with modalDialog = Modal.update msg bm.modalDialog})
        newModel, Cmd.none
    | Message.ShowLoader(msg) ->
        let newModel =
            updateBaseModel (fun bm -> {bm with showLoader = LoadShow.update msg bm.showLoader})
        newModel, Cmd.none
    | Message.Inner(msg) ->
        let showModel, showMessages = updateShow msg model.showModel
        {model with showModel = showModel}, showMessages |> Cmd.map Message.Inner

/// View function to render a particular core console utility.
let private viewUtil utilPage model dispatch dispatchServer =
    let bm = model.baseModel
    let onComplete() = None |> Message.UtilPage |> dispatch
    match utilPage with
    | ShowLoader ->
        LoadShow.view
            bm.showsAvailable
            bm.showLoader
            onComplete
            (Message.ShowLoader >> dispatch)
            dispatchServer

/// Dropdown item for accessing show loader page.
let private showLoaderItem: Navbar.Item<_> = {
    text = "Load show..."
    onClick = (fun dispatch -> ShowLoader |> Some |> Message.UtilPage |> dispatch)
}

/// Dropdown item for saving the current show.
let private saveShowItem: Navbar.Item<_> = {
    text = "Save"
    onClick = (fun dispatch ->
        (Exclusive, ServerCommand.Save)
        |> Message.Command
        |> dispatch)
}

/// Dropdown item to quit the console (this one is of questionable utility).
let private quitItem: Navbar.Item<_> = {
    text = "Quit"
    onClick = (fun dispatch ->
        let modalAction =
            Modal.confirm
                "Are you sure you want to quit?"
                (fun _ -> (All, ServerCommand.Quit) |> Message.Command |> dispatch)
        modalAction |> Modal.Open |> Message.Modal |> dispatch)
}

let utilDropdown: Navbar.DropdownModel<_> = {
    text = "Wiggles"
    items = [
        Navbar.Selection(showLoaderItem)
        Navbar.Separator
        Navbar.Selection(saveShowItem)
        Navbar.Separator
        Navbar.Selection(quitItem)
    ]
    isOpen = false
}

/// View the basic page structure including the navbar and the modal dialog if it's open.
/// Display a utility page if one is selected.  Otherwise, delegate the rest of the view to the console.
let private viewInner viewShow model dispatch =
    let openModal req = req |> Modal.Message.Open |> Message.Modal |> dispatch

    let page =
        match model.baseModel.utilPage with
        | None ->
            /// Dispatch a message to the server, lifting the filter up into the message type.
            let dispatchServer =
                liftResponseAndFilter ServerCommand.Console
                >> Message.Command
                >> dispatch

            viewShow
                openModal
                model.showModel
                (Message.Inner >> dispatch) // show dispatches a message to itself
                dispatchServer // show dispatches a message to the server
        | Some(util) ->
            viewUtil util model dispatch (Message.Command >> dispatch)

    R.div [] [
        R.div [] [Navbar.view model.baseModel.navbar dispatch (Message.Navbar >> dispatch)]
        R.div [Container.Fluid] [
            page
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
   