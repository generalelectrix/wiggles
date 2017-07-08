//! A Wiggles floating-point slider component.
module Slider
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Util.fsx"
#load "Bootstrap.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Util
open Bootstrap

let mutable nextId = 0

// TODO: provide view hooks for editing the range of the slider and/or directly entering a value.

type Model = {
    /// Becomes the id of this slider element's datalist, for binding step values/detents to it.
    uniqueId: string
    value: float
    min: float
    max: float
    step: float
    detents: float list
    // For implementing https://stackoverflow.com/a/37623959
    inputEventHasFired: bool
}

let private getUniqueId() =
    let id = nextId
    nextId <- nextId + 1
    sprintf "slider%d" id

let initModel value min max step detents = {
    uniqueId = getUniqueId()
    value = value
    min = min
    max = max
    step = step
    detents = detents
    inputEventHasFired = false
}

type Message =
    | ValueChange of float
    | InputHasFired

let update message model =
    match message with
    | ValueChange(v) -> {model with value = v}
    | InputHasFired -> {model with inputEventHasFired = true}

let view onValueChange model dispatch =

    // Technique for achieving consistent messaging while dragging a slider.
    // https://stackoverflow.com/a/37623959
    let valueChangeAction v =
        ValueChange v |> dispatch
        v |> onValueChange
       

    let onInput (e: React.FormEvent) =
        InputHasFired |> dispatch
        match parseFloat !!e.target?value with
        | Some(v) when v <> model.value ->
            valueChangeAction v
        | _ -> ()

    let onChange (e: React.FormEvent) =
        if not model.inputEventHasFired then
            match parseFloat !!e.target?value with
            | Some(v) ->
                valueChangeAction v
            | _ -> ()

    let detents = model.detents |> List.map (fun d -> R.option [Value (Case1 (string d))] [])

    R.div [] [
        R.input [
            InputType.Range
            Min (Case1 model.min)
            Max (Case1 model.max)
            Step (Case1 model.step)
            OnInput onInput
            OnChange onChange
            List model.uniqueId
            Value (Case1 (string model.value))
        ] []
        R.datalist [Id model.uniqueId] detents
    ]