/// Full view application.
/// Serving as a proof-of-concept.
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Types.fsx"
#load "WigglesBase.fsx"
#load "Patcher.fsx"
#load "Navbar.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Types

// If true, log verbose and interactive messages to the javascript console on every update.
let withConsoleTrace = true

type Page =
    | PatchPage

type ShowModel = {
    page: Page
    patcher: Patcher.Model
}

type ServerCommand =
    | PatchCommand of PatchServerRequest

type ShowMessage =
    | SetPage of Page
    | Patch of Patcher.Message

// Configure the patcher and create a nav item for it.
let patcherNavItem: Navbar.Item<_> = {
    text = "Patch"
    onClick = (fun dispatch -> SetPage PatchPage |> WigglesBase.Message.Show |> dispatch)
}

let navbar: NavBar.Model<_> = {
    leftItems = [NavBar.Single patcherNavItem]
    rightItems = []
    activeItem = NavBar.Left(0)
}

let initShowModel () = {
    page = PatchPage
    patcher = Patcher.initialModel()
}

/// Master function to initialize the whole interface.
let initModel (): WigglesBase.Model = WigglesBase.initModel navbar (initShowModel())

/// Every command we need to emit when we connect to the server.
let initCommands = [
        Patcher.initCommands
        WigglesBase.initCommands
    ] |> Cmd.batch

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

let update msg model = WigglesBase.update updateShow msg model

let view model dispatch = WigglesBase.view viewShow model dispatch

Program.mkProgram initModel update view
|> Program.withSubscription WigglesBase.subscription
|> Program.withReact "app"
|> (if withConsoleTrace then Program.withConsoleTrace else id)
|> Program.run