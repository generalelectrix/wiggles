//! View a knob-controlled wiggle.
module Wiggles
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Util.fsx"
#load "Types.fsx"
#load "DataflowTypes.fsx"
#load "WiggleTypes.fsx"
#load "Knob.fsx"
#load "Knobs.fsx"
#load "Bootstrap.fsx"
#load "EditBox.fsx"
#load "Clocks.fsx"

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


/// Components for creating new wiggles.
module NewWiggle =

    type Model = {
        name: EditBox.Model<string>
        selectedKind: string
    }

    let initModel() = {
        name =
            EditBox.initialModel "Name:" errorIfEmpty (InputType.Text)
            |> EditBox.setFailed ""
        selectedKind = ""
    }

    type Message =
        | NameEdit of EditBox.Message<string>
        | SelectKind of string

    let update message (model: Model) =
        match message with
        | NameEdit(msg) -> {model with name = EditBox.update msg model.name}
        | SelectKind(kind) -> {model with selectedKind = kind}

    /// Render wiggle kind selector drop-down.
    let private viewKindSelector kinds selected dispatch =
        let options = kinds |> List.map (fun kind -> R.option [ Value (Case1 kind) ] [ R.str kind ])

        R.div [] [
            R.select [
                Form.Control
                OnChange (fun e -> !!e.target?value |> SelectKind |> dispatch)
                Value (Case1 selected)
            ] options
        ]

    /// View the new wiggle creator.
    let view kinds model dispatchLocal dispatchServer =
        R.div [] [
            R.h4 [] [R.str "Create new wiggle"]
            EditBox.view None "" model.name (NameEdit >> dispatchLocal)
            R.label [] [
                R.str "Kind:"
                viewKindSelector kinds model.selectedKind dispatchLocal
            ]
            R.button [
                Button.Primary
                OnClick (fun _ ->
                    match model.name with
                    | EditBox.Parsed(name) ->
                        let create: CreateWiggle = {kind = model.selectedKind; name = name}
                        create
                        |> Command.Create
                        |> all
                        |> dispatchServer
                    | _ -> ())
            ] [ R.str "Create" ]
        ]

type Wiggles = Map<WiggleId, WiggleDescription>

type Model = {
    kinds: string list
    wiggles: Wiggles
    newWiggle: NewWiggle.Model
}

let initModel() = {
    kinds = []
    wiggles = Map.empty
    newWiggle = NewWiggle.initModel()
}

let initCommands = [Command.Kinds; Command.State]

type Message =
    | Response of Response
    | NewWiggle of NewWiggle.Message

/// Update this wiggle collection based on a server response.
let updateFromServer response model =
    /// Return a model updated by transforming a single wiggle.
    let transformWiggle id f = {model with wiggles = model.wiggles |> transformMapItem id f}

    /// Return a model updated by tranforming a single wiggle's inputs.
    let transformInputs id f =
        let transform wiggle = {wiggle with inputs = f wiggle.inputs}
        transformWiggle id transform

    match response with
    | Response.Kinds(cls) ->
        {model with
            kinds = cls
            newWiggle = {model.newWiggle with selectedKind = List.head cls}}
    | Response.State(state) -> {model with wiggles = state |> Map.ofList}
    | Response.New(id, desc) -> {model with wiggles = model.wiggles |> Map.add id desc}
    | Response.Removed(id) -> {model with wiggles = model.wiggles |> Map.remove id}
    | Response.Renamed(id, name) -> transformWiggle id (fun wiggle -> {wiggle with name = name})
    | Response.SetInput(si) ->
        transformInputs
                si.wiggle
                (List.mapi (fun inputId input -> if si.input = inputId then si.target else input))
    | Response.PushInput(id) ->
        transformInputs id (fun inputs -> List.append inputs [None])
    | Response.PopInput(id) ->
        transformInputs id (fun (inputs: (WiggleId * OutputId) option list) ->
            if inputs.IsEmpty then
                logError (sprintf
                    "Got a command to pop an input from wiggle %+A but it already has no inputs."
                    id)
                inputs
            else inputs.[..inputs.Length-1]
        )
    | Response.PushOutput(id) ->
        transformWiggle id (fun wiggle -> {wiggle with outputs = wiggle.outputs + 1})
    | Response.PopOutput(id) ->
        transformWiggle id (fun wiggle ->
            if wiggle.outputs = 0 then
                logError (sprintf
                    "Got a command to pop an output from wiggle %+A but it already has no outputs."
                    id)
                wiggle
            else
                {wiggle with outputs = wiggle.outputs - 1})
    | Response.SetClock(id, source) ->
        transformWiggle id (fun wiggle -> {wiggle with clock = Yes source})

let update message model =
    match message with
    | Response(r) -> updateFromServer r model
    | NewWiggle(msg) -> {model with newWiggle = NewWiggle.update msg model.newWiggle}

/// Render input selector drop-down.
/// On change, send a input swap command to the server and allow the response to update the state
/// of this control.
let private inputSelector wiggleId inputId (currentValue: (WiggleId * OutputId) option) (wiggles: Wiggles) dispatchServer =
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
                let cmd: SetInput = {wiggle = wiggleId; input = inputId; target = selected}
                cmd |> WiggleTypes.Command.SetInput |> all |> dispatchServer
            )
            Value (Case1 (toJson currentValue))
        ] options
    ]

let addInput wiggleId dispatchServer =
    R.button [
        Button.Primary
        OnClick (fun _ -> wiggleId |> WiggleTypes.Command.PushInput |> all |> dispatchServer)
    ] [R.str "Add Input"]

let dropInput wiggleId dispatchServer =
    R.button [
        Button.Default
        OnClick (fun _ -> wiggleId |> WiggleTypes.Command.PopInput |> all |> dispatchServer)
    ] [R.str "Drop Input"]

let addOutput wiggleId dispatchServer =
    R.button [
        Button.Primary
        OnClick (fun _ -> wiggleId |> WiggleTypes.Command.PushOutput |> all |> dispatchServer)
    ] [R.str "Add Output"]

let dropOutput wiggleId dispatchServer =
    R.button [
        Button.Default
        OnClick (fun _ -> wiggleId |> WiggleTypes.Command.PopOutput |> all |> dispatchServer)
    ] [R.str "Drop Output"]

/// Render a clock selector drop-down.
let private clockSelector wiggleId (currentValue: ClockId option) (clocks: Clocks.Clocks) dispatchServer =
    let option (id: ClockId, cd: ClockTypes.ClockDescription) =
        R.option
            [ Value (Case1 (toJson (Some id))) ]
            [ R.str cd.name ]

    let options = [
        yield R.option [ Value (Case1 (toJson None)) ] [ R.str "{disconnected}" ]
        for clock in clocks |> Map.toSeq do yield option clock
    ]

    R.div [] [
        R.select [
            Form.Control
            OnChange (fun e -> 
                let selected: ClockId option = !!e.target?value |> ofJson
                (wiggleId, selected) |> WiggleTypes.Command.SetClock |> all |> dispatchServer
            )
            Value (Case1 (toJson currentValue))
        ] options
    ]

/// Render a single wiggle.
/// TODO: name edit in-place
let viewWiggle
        wiggleId
        (wiggle: WiggleDescription)
        (wiggles: Wiggles)
        knobs
        clocks
        dispatchKnobLocal
        dispatchKnobServer
        dispatchWiggleServer =
    let inputSelectors =
        wiggle.inputs
        |> List.mapi (fun inputId source ->
            inputSelector wiggleId inputId source wiggles dispatchWiggleServer)
        |> R.div []

    let clockSelector =
        match wiggle.clock with
        | Yes(source) ->
            let selector = clockSelector wiggleId source clocks dispatchWiggleServer
            R.div [] [R.str "Clock Source:"; selector]
        | No -> R.div [] []

    let addrFilter (addr: KnobAddress) =
        match addr with
        | Wiggle(id, _) when wiggleId = id -> true
        | _ -> false

    R.div [
        Style [CSSProp.Width "200px"]
    ] [
        R.str (sprintf "%s (%s)" wiggle.name wiggle.kind)
        clockSelector
        R.div [] [
            R.str "Inputs:"
            inputSelectors
            addInput wiggleId dispatchWiggleServer
            dropInput wiggleId dispatchWiggleServer
        ]
        R.div [] [
            R.str (sprintf "Outputs: %d" wiggle.outputs)
            addOutput wiggleId dispatchWiggleServer
            dropOutput wiggleId dispatchWiggleServer
        ]
        Knobs.viewAllWith addrFilter knobs dispatchKnobLocal dispatchKnobServer
    ]

/// Render every wiggle.  Primarily for test purposes.
let viewAllWiggles wiggles knobs clocks dispatchKnobLocal dispatchKnobServer dispatchWiggleServer =
    wiggles
    |> Map.toSeq
    |> Seq.map (fun (wiggleId, wiggle) ->
        viewWiggle wiggleId wiggle wiggles knobs clocks dispatchKnobLocal dispatchKnobServer dispatchWiggleServer)
    |> List.ofSeq
    |> R.div []

let view knobs clocks model dispatchKnob dispatchWiggle dispatchKnobServer dispatchWiggleServer =
    // Draw the new wiggle editor first.
    R.div [] [
        NewWiggle.view model.kinds model.newWiggle (NewWiggle >> dispatchWiggle) dispatchWiggleServer
        viewAllWiggles model.wiggles knobs clocks dispatchKnob dispatchKnobServer dispatchWiggleServer
    ]

