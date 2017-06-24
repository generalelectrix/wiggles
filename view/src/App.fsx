/// Full view application.
/// Serving as a proof-of-concept.
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "core/Base.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Types
open Socket
open Bootstrap

// If true, log verbose and interactive messages to the javascript console on every update.
let withConsoleTrace = false

type Page =
    | TestPage

type ShowModel = {
    page: Page
    model: string
}

type ShowServerCommand =
    | TestCommand
  
type ShowServerResponse =
    | TestResponse

type ShowMessage =
    | SetPage of Page
    | SetModel of string

let navItem: Navbar.Item<_> = {
    text = "Test"
    onClick = (fun dispatch -> SetPage TestPage |> Base.Message.Inner |> dispatch)
}

let navbar: Navbar.Model<_> = {
    leftItems = [Navbar.Dropdown Base.utilDropdown; Navbar.Single navItem]
    rightItems = []
    activeItem = Navbar.Left(1)
}

let initShowModel () = {
    page = TestPage
    model = "nothing so far"
}

/// Master function to initialize the whole interface.
/// Since we need to wait for websocket connection to send any messages to the server, we initially
/// emit no commands.
let initModel () = (Base.initModel navbar (initShowModel()), Cmd.none)

/// Every command we need to emit when we connect to the server.
/// We assume that since that these are query-only, the responses are all filtered to just this
/// client.
let initCommands =
    [Base.initCommands]
    |> List.concat
    |> List.map (fun c -> (Exclusive, c))
    |> List.map Cmd.ofMsg
    |> Cmd.batch

/// Handle taking a server response message and cramming it into the show message type.
/// This is needed to allow the base logic to be totally agnostic about the structure of the
/// response messages expected by this show.
let wrapShowResponse (message: ShowServerResponse) =
    match message with
    | TestResponse -> SetModel "received response"

let updateShow message model =
    match message with
    | SetPage(page) -> {model with page = page}, Cmd.none
    | SetModel(msg) -> {model with model = msg}, Cmd.none

let viewShow openModal model dispatch dispatchServer =
    match model.page with
    | TestPage ->
        R.div [] [
            R.str (sprintf "Text: %s" model.model)
            R.button [
                Button.Basic
                OnClick (fun _ -> (ResponseFilter.Exclusive, TestCommand) |> dispatchServer)
            ] [R.str "issue command"]
        ]

/// Type alias to ensure that generic inference gets the right types all the way down.
type ConcreteMessage = Base.Message<ShowServerCommand, ShowServerResponse, ShowMessage>

type ConcreteModel = Base.Model<ShowModel, ConcreteMessage>


// Launch the websocket we'll use to talk to the server.
let (subscription, send) =
    openSocket<ServerResponse<ShowServerResponse>, ConcreteMessage> Base.Message.Socket

let update
        (msg: ConcreteMessage)
        (model: ConcreteModel)
        : ConcreteModel * Cmd<ConcreteMessage> =
    Base.update initCommands send wrapShowResponse updateShow msg model

let view model dispatch = Base.view viewShow model dispatch

Program.mkProgram
    initModel
    update
    view
|> Program.withSubscription (subscription Base.Message.Response)
|> Program.withReact "app"
|> (if withConsoleTrace then Program.withConsoleTrace else id)
|> Program.run