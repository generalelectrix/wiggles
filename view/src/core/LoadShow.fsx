//! View for loading a different show.
module LoadShow
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Util.fsx"
#load "Bootstrap.fsx"
#load "Table.fsx"
#load "Types.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Util
open Bootstrap
open Types

type Row = Row of string
    with
    interface Table.IRow with
        member this.ToStrings() =
            let (Row s) = this
            [s]
          
type Model = {
    table: Table.Model
    loadSpec: LoadSpec
}

let initModel() = {
    table = {height = 300; header = ["Show name"]; selected = None}
    loadSpec = Latest
}

type Message =
    | Table of Table.Message
    | LoadSpec of LoadSpec

let update message model =
    match message with
    | Table(t) -> {model with table = Table.update t model.table}
    | LoadSpec(spec) -> {model with loadSpec = spec}

/// Radio buttons to select whether to load from save or autosave.
let loadModeSelector selected dispatch =
    let radio text spec =
        let onClick _ = spec |> LoadSpec |> dispatch
        R.div [ClassName "radio"] [
            R.label [] [
                R.input [
                    InputType.Radio
                    OnClick onClick
                    Checked (selected = spec)
                    ReadOnly true // not actually read-only but react complains otherwise
                ] []
                R.str text
            ]
        ]

    Grid.layout [
        (3, [radio "Load from save" Latest])
        (3, [radio "Recover from autosave" LatestAutosave])
    ]

/// A button that, when pressed, will dispatch a message to load the selected show.
let loadButton (shows: string list) model onComplete dispatchServer =
    /// When we click load, dispatch a server message to load this show.
    /// Also call the provided callback that we should execute on show load.
    let onClick _ =
        match model.table.selected with
        | Some(i) ->
            match shows |> List.tryItem i with
            | Some(save) ->
                let command = ServerCommand.Load({name = save; spec = model.loadSpec})
                (All, command) |> dispatchServer
                onComplete()
            | None ->
                // log this as this is unexpected
                logError
                    (sprintf
                        "Load action had a selected value %d that was not in range."
                        i)
        | None -> ()

    R.button
        [Button.Primary; OnClick onClick]
        [R.str "Load"]

/// Button to exit the load subsystem.
let cancelButton onComplete =
    let onClick _ = onComplete()
    R.button
        [Button.Default; OnClick onClick]
        [R.str "Cancel"]

let view shows model onComplete dispatch dispatchServer = 
    let showTable = Table.view (shows |> List.map Row) model.table (Table >> dispatch)
    let loadButton = loadButton shows model onComplete dispatchServer
    R.div [] [
        Grid.fullRow [showTable]
        Grid.fullRow [loadModeSelector model.loadSpec dispatch]
        Grid.layout [
            (1, [loadButton])
            (1, [cancelButton onComplete])
        ]
    ]

 
    