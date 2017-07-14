//! View components for connecting patched fixture inputs to wiggles.
module Controls

#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "../core/Util.fsx"
#load "../core/Types.fsx"
#load "../core/DataflowTypes.fsx"
#load "../core/WiggleTypes.fsx"
#load "../core/Bootstrap.fsx"
#load "../core/Wiggles.fsx"
#load "PatchTypes.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props

open Util
open Types
open Bootstrap
open DataflowTypes
open WiggleTypes
open PatchTypes

/// Select a source for this fixture control parameter.
let private sourceSelector fixtureId controlId (wiggles: Wiggles.Wiggles) (selected: (WiggleId * ControlId) option) dispatchServer =
    /// Yield one option for each output that this wiggle has.
    let options (id: WiggleId, cd: WiggleDescription) = seq {
        for outputId in 0..cd.outputs-1 do
            yield
                R.option
                    [ Value (Case1 (toJson (Some (id, outputId)))) ]
                    [ R.str (sprintf "%s (output %d)" cd.name outputId) ]
    }

    let options = [
        yield R.option [ Value (Case1 (toJson None)) ] [ R.str "{disconnected}" ]
        for wiggle in wiggles |> Map.toSeq do yield! options wiggle
    ]

    R.div [] [
        R.select [
            Form.Control
            OnChange (fun e -> 
                let selected: (WiggleId * OutputId) option = !!e.target?value |> ofJson
                (fixtureId, controlId, selected)
                |> PatchServerRequest.SetControlSource
                |> all
                |> dispatchServer
            )
            Value (Case1 (toJson selected))
        ] options
    ]

let private viewSources (item: PatchItem<_>) wiggles dispatchServer =
    item.controlSources
    |> List.mapi (fun controlId control ->
        R.div [] [
            R.str (sprintf "%s (%O)" control.name control.dataType)
            sourceSelector item.id controlId wiggles control.source dispatchServer
        ])
    |> R.div []
        

let private viewPatchItem wiggles dispatchServer (item: PatchItem<_>) =
    R.div [] [
        R.h4 [] [R.str (sprintf "%s (fixture %d)" item.name item.id)]
        viewSources item wiggles dispatchServer
    ]

/// Display input selectors for every patch item.
let view (patches: PatchItem<_> array) wiggles dispatchServer =
    patches
    |> Array.toSeq
    |> Seq.map (viewPatchItem wiggles dispatchServer)
    |> List.ofSeq
    |> R.div []