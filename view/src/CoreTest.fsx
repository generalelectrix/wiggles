/// Full view application.
/// Serving as a proof-of-concept.
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Types.fsx"
#load "Navbar.fsx"
#load "Socket.fsx"
#load "WigglesBase.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Types
open Socket

// If true, log verbose and interactive messages to the javascript console on every update.
let withConsoleTrace = true

type Page =
    | TestPage

type ShowModel = {
    page: Page
    model: string
}

type ShowServerCommand =
    | C

type ShowServerResponse =
    | PatchResponse of PatchServerResponse

type ShowMessage =
    | SetPage of Page
    | Patch of Patcher.Message

// Configure the patcher and create a nav item for it.
let patcherNavItem: Navbar.Item<_> = {
    text = "Patch"
    onClick = (fun dispatch -> SetPage PatchPage |> WigglesBase.Message.Inner |> dispatch)
}

let navbar: Navbar.Model<_> = {
    leftItems = [Navbar.Single patcherNavItem]
    rightItems = []
    activeItem = Navbar.Left(0)
}

let initShowModel () = {
    page = PatchPage
    patcher = Patcher.initialModel()
}

/// Master function to initialize the whole interface.
/// Since we need to wait for websocket connection to send any messages to the server, we initially
/// emit no commands.
let initModel () = (WigglesBase.initModel navbar (initShowModel()), Cmd.none)

/// Every command we need to emit when we connect to the server.
let initCommands =
    let patcherCommands =
        Patcher.initCommands
        |> List.map (PatchCommand >> WigglesBase.ServerCommand.Console)

    [patcherCommands; WigglesBase.initCommands]
    |> List.concat
    |> List.map Cmd.ofMsg
    |> Cmd.batch

/// Handle taking a server response message and cramming it into the show message type.
/// This is needed to allow the base logic to be totally agnostic about the structure of the
/// response messages expected by this show.
let wrapShowResponse (message: ShowServerResponse) =
    match message with
    | PatchResponse(rsp) -> rsp |> Patcher.Response |> Patch

let updateShow message model =
    match message with
    | SetPage(page) -> {model with page = page}, Cmd.none
    | Patch(msg) ->
        let patcher, commands = Patcher.update msg model.patcher
        {model with patcher = patcher}, commands |> Cmd.map Patch

let viewShow openModal model dispatch dispatchServer =
    match model.page with
    | PatchPage ->
        Patcher.view openModal model.patcher (Patch >> dispatch) (PatchCommand >> dispatchServer)


// Launch the websocket we'll use to talk to the server.
let (subscription, send: WigglesBase.ServerCommand<ShowServerCommand> -> unit) =
    openSocket WigglesBase.Message.Socket

/// Type alias to ensure that generic inference gets the right types all the way down.
type ConcreteMessage = WigglesBase.Message<ShowServerCommand, ShowServerResponse, ShowMessage>

type ConcreteModel = WigglesBase.Model<ShowModel, ConcreteMessage>

let update
        (msg: ConcreteMessage)
        (model: ConcreteModel)
        : ConcreteModel * Cmd<ConcreteMessage> =
    WigglesBase.update initCommands send wrapShowResponse updateShow msg model

let view model dispatch = WigglesBase.view viewShow model dispatch

Program.mkProgram
    initModel
    update
    view
|> Program.withSubscription (subscription WigglesBase.Message.Response)
|> Program.withReact "app"
|> (if withConsoleTrace then Program.withConsoleTrace else id)
|> Program.run