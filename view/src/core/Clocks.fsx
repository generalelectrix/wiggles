//! View a knob-controlled clock.
module Clocks
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Util.fsx"
#load "Types.fsx"
#load "DataflowTypes.fsx"
#load "ClockTypes.fsx"
#load "Knob.fsx"
#load "Knobs.fsx"
#load "Bootstrap.fsx"

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
open ClockTypes

type Clocks = Map<ClockId, ClockDescription>

type Model = {
    classes: string list
    clocks: Clocks
}

/// Update this clock collection based on a server response.
let updateFromServer response model =
    /// Return a model updated by transforming a single clock.
    let transformClock id f = {model with clocks = model.clocks |> transformMapItem id f}

    /// Return a model updated by tranforming a single clock's inputs.
    let transformInputs id f =
        let transform clock = {clock with inputs = f clock.inputs}
        transformClock id transform

    match response with
    | Response.Classes(cls) -> {model with classes = cls}
    | Response.State(state) -> {model with clocks = state |> Map.ofList}
    | Response.New(id, desc) -> {model with clocks = model.clocks |> Map.add id desc}
    | Response.Removed(id) -> {model with clocks = model.clocks |> Map.remove id}
    | Response.Renamed(id, name) -> transformClock id (fun clock -> {clock with name = name})
    | Response.SetInput(si) ->
        transformInputs
                si.clock
                (List.mapi (fun inputId input -> if si.input = inputId then si.target else input))
    | Response.PushInput(id) ->
        transformInputs id (fun inputs -> List.append inputs [None])
    | Response.PopInput(id) ->
        transformInputs id (fun (inputs: ClockId option list) ->
            if inputs.IsEmpty then
                logError (sprintf
                    "Got a command to pop an input from clock %+A but it already has no inputs."
                    id)
                inputs
            else inputs.[..inputs.Length-1]
        )

/// Render input selector drop-down.
/// On change, send a input swap command to the server and allow the response to update the state
/// of this control.
let private inputSelector clockId inputId (currentValue: ClockId option) (clocks: Clocks) dispatchServer =
    let option (id: ClockId, cd: ClockDescription) =
        R.option
            [ Value (Case1 (toJson (Some id))) ]
            [ R.str cd.name ]

    let options = clocks |> Map.toSeq |> Seq.map option |> List.ofSeq

    R.div [] [
        R.select [
            Form.Control
            OnChange (fun e -> 
                let selected: ClockId option = !!e.target?value |> ofJson
                let cmd: SetInput = {clock = clockId; input = inputId; target = selected}
                cmd |> ClockTypes.Command.SetInput |> all |> dispatchServer
            )
            Value (Case1 (toJson currentValue))
        ] options
    ]

let viewClock
        clockId
        (clock: ClockDescription)
        (clocks: Clocks)
        knobs
        dispatchKnobLocal
        dispatchKnobServer
        dispatchClockServer =
    let inputSelectors =
        clock.inputs
        |> List.mapi (fun inputId source ->
            inputSelector clockId inputId source clocks dispatchClockServer)
        |> R.div []


    let addrFilter (addr: KnobAddress) =
        match addr with
        | Clock(id, _) when clockId = id -> true
        | _ -> false

    R.div [] [
        R.str (sprintf "%s (%s)" clock.name clock.kind)
        inputSelectors
        Knobs.viewAllWith addrFilter knobs dispatchKnobLocal dispatchKnobServer
    ]


let viewAllClocks clocks knobs dispatchKnobLocal dispatchKnobServer dispatchClockServer =
    clocks
    |> Map.toSeq
    |> Seq.map (fun (clockId, clock) ->
        viewClock clockId clock clocks knobs dispatchKnobLocal dispatchKnobServer dispatchClockServer)
    |> List.ofSeq
    |> R.div []
