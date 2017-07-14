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
#load "EditBox.fsx"

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


/// Components for creating new clocks.
module NewClock =

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

    /// Render clock kind selector drop-down.
    let private viewKindSelector kinds selected dispatch =
        let options = kinds |> List.map (fun kind -> R.option [ Value (Case1 kind) ] [ R.str kind ])

        R.div [] [
            R.select [
                Form.Control
                OnChange (fun e -> !!e.target?value |> SelectKind |> dispatch)
                Value (Case1 selected)
            ] options
        ]

    /// View the new clock creator.
    let view kinds model dispatchLocal dispatchServer =
        R.div [] [
            R.h4 [] [R.str "Create new clock"]
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
                        let create: CreateClock = {kind = model.selectedKind; name = name}
                        create
                        |> Command.Create
                        |> all
                        |> dispatchServer
                    | _ -> ())
            ] [ R.str "Create" ]
        ]

type Clocks = Map<ClockId, ClockDescription>

type Model = {
    kinds: string list
    clocks: Clocks
    newClock: NewClock.Model
}

let initModel() = {
    kinds = []
    clocks = Map.empty
    newClock = NewClock.initModel()
}

let initCommands = [Command.Kinds; Command.State]

type Message =
    | Response of Response
    | NewClock of NewClock.Message

/// Update this clock collection based on a server response.
let updateFromServer response model =
    /// Return a model updated by transforming a single clock.
    let transformClock id f = {model with clocks = model.clocks |> transformMapItem id f}

    /// Return a model updated by tranforming a single clock's inputs.
    let transformInputs id f =
        let transform clock = {clock with inputs = f clock.inputs}
        transformClock id transform

    match response with
    | Response.Kinds(cls) ->
        {model with
            kinds = cls
            newClock = {model.newClock with selectedKind = List.head cls}}
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

let update message model =
    match message with
    | Response(r) -> updateFromServer r model
    | NewClock(msg) -> {model with newClock = NewClock.update msg model.newClock}

/// Render input selector drop-down.
/// On change, send a input swap command to the server and allow the response to update the state
/// of this control.
let private inputSelector clockId inputId (currentValue: ClockId option) (clocks: Clocks) dispatchServer =
    let option (id: ClockId, cd: ClockDescription) =
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
                let cmd: SetInput = {clock = clockId; input = inputId; target = selected}
                cmd |> ClockTypes.Command.SetInput |> all |> dispatchServer
            )
            Value (Case1 (toJson currentValue))
        ] options
    ]

let addInput clockId dispatchServer =
    R.button [
        Button.Primary
        OnClick (fun _ -> clockId |> ClockTypes.Command.PushInput |> all |> dispatchServer)
    ] [R.str "Add Input"]

let dropInput clockId dispatchServer =
    R.button [
        Button.Default
        OnClick (fun _ -> clockId |> ClockTypes.Command.PopInput |> all |> dispatchServer)
    ] [R.str "Drop Input"]

/// Render a single clock.
/// TODO: name edit in-place
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

    R.div [
        Style [CSSProp.Width "200px"]
    ] [
        R.str (sprintf "%s (%s)" clock.name clock.kind)
        addInput clockId dispatchClockServer
        dropInput clockId dispatchClockServer
        inputSelectors
        Knobs.viewAllWith addrFilter knobs dispatchKnobLocal dispatchKnobServer
    ]

/// Render every clock.  Primarily for test purposes.
let viewAllClocks clocks knobs dispatchKnobLocal dispatchKnobServer dispatchClockServer =
    clocks
    |> Map.toSeq
    |> Seq.map (fun (clockId, clock) ->
        viewClock clockId clock clocks knobs dispatchKnobLocal dispatchKnobServer dispatchClockServer)
    |> List.ofSeq
    |> R.div []

let view knobs model dispatchKnob dispatchClock dispatchKnobServer dispatchClockServer =
    // Draw the new clock editor first.
    R.div [] [
        NewClock.view model.kinds model.newClock (NewClock >> dispatchClock) dispatchClockServer
        viewAllClocks model.clocks knobs dispatchKnob dispatchKnobServer dispatchClockServer
    ]

