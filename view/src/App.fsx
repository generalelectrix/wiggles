/// Full view application.
/// Serving as a proof-of-concept.
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "core/Base.fsx"
#load "core/Knobs.fsx"
#load "core/Clocks.fsx"
#load "core/Wiggles.fsx"
#load "patcher/Patcher.fsx"
#load "patcher/Controls.fsx"

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
let logSocketTraffic = false

type Page =
    | Patcher
    | Controls
    | KnobTest
    | ClockTest
    | WiggleTest

type KnobAddress = DataflowTypes.KnobAddress
type ControlSource = DataflowTypes.WiggleId * DataflowTypes.OutputId

type ShowModel = {
    page: Page
    patcher: Patcher.Model<ControlSource>
    knobs: Knobs.Model<KnobAddress>
    clocks: Clocks.Model
    wiggles: Wiggles.Model
}

[<RequireQualifiedAccess>]
type ShowServerCommand =
    | Patcher of PatchTypes.PatchServerRequest<ControlSource>
    | Knob of Knobs.ServerCommand<KnobAddress>
    | Clock of ClockTypes.Command
    | Wiggle of WiggleTypes.Command
  
[<RequireQualifiedAccess>]
type ShowServerResponse =
    | Error of string
    | Patcher of PatchTypes.PatchServerResponse<ControlSource>
    | Knob of Knobs.ServerResponse<KnobAddress>
    | Clock of ClockTypes.Response
    |Wiggle of WiggleTypes.Response

[<RequireQualifiedAccess>]
type ShowMessage =
    | Error of string
    | SetPage of Page
    | Patcher of Patcher.Message<ControlSource>
    | Knob of Knobs.Message<KnobAddress>
    | Clock of Clocks.Message
    | Wiggle of Wiggles.Message

/// Type alias to ensure that generic inference gets the right types all the way down.
type ConcreteMessage = Base.Message<ShowServerCommand, ShowServerResponse, ShowMessage>

type ConcreteModel = Base.Model<ShowModel, ConcreteMessage>

let pageNavItem name page : Navbar.Item<ConcreteMessage> = {
    text = name
    onClick = (fun dispatch -> ShowMessage.SetPage page |> Base.Message.Inner |> dispatch)
}

let navbar: Navbar.Model<ConcreteMessage> = {
    leftItems =
       [Navbar.Dropdown (Base.utilDropdown())
        Navbar.Single (pageNavItem "Knobs" KnobTest)
        Navbar.Single (pageNavItem "Clocks" ClockTest)
        Navbar.Single (pageNavItem "Wiggles" WiggleTest)]
    rightItems =
       [Navbar.Single (pageNavItem "Controls" Controls)
        Navbar.Single (pageNavItem "Patch" Patcher)]
    activeItem = Navbar.Right(1)
}

let initShowModel () = {
    page = Patcher
    patcher = Patcher.initialModel()
    knobs = Knobs.initModel()
    clocks = Clocks.initModel()
    wiggles = Wiggles.initModel()
}

/// Master function to initialize the whole interface.
/// Since we need to wait for websocket connection to send any messages to the server, we initially
/// emit no commands.
let initModel () = (Base.initModel navbar (initShowModel()), Cmd.none)

/// Every command we need to emit when we connect to the server.
/// We assume that these are query-only and the responses are all filtered to just this
/// client.
let initCommands =
    [
        Base.initCommands
        Patcher.initCommands |> List.map (ShowServerCommand.Patcher >> ServerCommand.Console)
        Knobs.initCommands |> List.map (ShowServerCommand.Knob >> ServerCommand.Console)
        Clocks.initCommands |> List.map (ShowServerCommand.Clock >> ServerCommand.Console)
        Wiggles.initCommands |> List.map (ShowServerCommand.Wiggle >> ServerCommand.Console)
    ]
    |> List.concat
    |> List.map exclusive
    |> List.map Cmd.ofMsg
    |> Cmd.batch

/// Handle taking a server response message and cramming it into the show message type.
/// This is needed to allow the base logic to be totally agnostic about the structure of the
/// response messages expected by this show.
let wrapShowResponse (message: ShowServerResponse) =
    match message with
    | ShowServerResponse.Error(e) -> e |> ShowMessage.Error
    | ShowServerResponse.Patcher(m) -> m |> Patcher.Message.Response |> ShowMessage.Patcher
    | ShowServerResponse.Knob(k) -> k |> Knobs.Message.Response |> ShowMessage.Knob
    | ShowServerResponse.Clock(c) -> c |> Clocks.Message.Response |> ShowMessage.Clock
    | ShowServerResponse.Wiggle(c) -> c |> Wiggles.Message.Response |> ShowMessage.Wiggle

let updateShow message model : ShowModel * Cmd<ConcreteMessage> =
    match message with
    | ShowMessage.Error(e) ->
        model, Modal.prompt e |> Modal.Open |> Base.Message.Modal |> Cmd.ofMsg
    | ShowMessage.SetPage(page) -> {model with page = page}, Cmd.none
    | ShowMessage.Patcher(msg) ->
        let updatedPatcher, commands = Patcher.update msg model.patcher
        {model with patcher = updatedPatcher}, commands |> Cmd.map (ShowMessage.Patcher >> Base.Message.Inner)
    | ShowMessage.Knob(msg) ->
        let updatedKnobs = Knobs.update msg model.knobs
        {model with knobs = updatedKnobs}, Cmd.none
    | ShowMessage.Clock(msg) ->
        let updatedClocks = Clocks.update msg model.clocks
        {model with clocks = updatedClocks}, Cmd.none
    | ShowMessage.Wiggle(msg) ->
        let updatedWiggles = Wiggles.update msg model.wiggles
        {model with wiggles = updatedWiggles}, Cmd.none

let viewShow openModal model dispatch dispatchServer =
    match model.page with
    | Patcher ->
        Patcher.view
            openModal
            model.patcher
            (ShowMessage.Patcher >> dispatch)
            ((Base.liftResponseAndFilter ShowServerCommand.Patcher) >> dispatchServer)
    | Controls ->
        Controls.view
            model.patcher.patches
            model.wiggles.wiggles
            ((Base.liftResponseAndFilter ShowServerCommand.Patcher) >> dispatchServer)
    | KnobTest ->
        let knobs =
            model.knobs
            |> Map.toSeq
            |> Seq.map ( fun (addr, knob) ->
                Knobs.viewOne
                    addr
                    knob
                    (ShowMessage.Knob >> dispatch)
                    ((Base.liftResponseAndFilter ShowServerCommand.Knob) >> dispatchServer))
            |> List.ofSeq
        R.div [] knobs
    | ClockTest ->
        Clocks.view
            model.knobs
            model.clocks
            (ShowMessage.Knob >> dispatch)
            (ShowMessage.Clock >> dispatch)
            ((Base.liftResponseAndFilter ShowServerCommand.Knob) >> dispatchServer)
            ((Base.liftResponseAndFilter ShowServerCommand.Clock) >> dispatchServer)
    | WiggleTest ->
        Wiggles.view
            model.knobs
            model.clocks.clocks
            model.wiggles
            (ShowMessage.Knob >> dispatch)
            (ShowMessage.Wiggle >> dispatch)
            ((Base.liftResponseAndFilter ShowServerCommand.Knob) >> dispatchServer)
            ((Base.liftResponseAndFilter ShowServerCommand.Wiggle) >> dispatchServer)

// Launch the websocket we'll use to talk to the server.
let (subscription, send) =
    openSocket<ServerResponse<ShowServerResponse>, ConcreteMessage>
        logSocketTraffic
        Base.Message.Socket

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