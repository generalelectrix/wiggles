//! The basic unit of Wiggles UI interface.
module Knob
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Util.fsx"
#load "Bootstrap.fsx"
#load "Types.fsx"
#load "Slider.fsx"

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

/// For now, just declare the basic Wiggles datatypes inside the Knob module, as we likely won't
/// encounter them too many other places.
module Wiggle =
    /// The basic abstraction for Wiggles data.
    type Data =
        | Unipolar of float
        | Bipolar of float

    /// Datatype markers for Wiggles
    [<RequireQualifiedAccess>]
    type Datatype =
        | Unipolar
        | Bipolar

    let datatype data =
        function
        | Unipolar(_) -> Datatype.Unipolar
        | Bipolar(_) -> Datatype.Bipolar

type Rate =
    | Hz of float
    | Bpm of float
    | Period of float
    with
    member this.inHz() =
        match this with
        | Hz(hz) -> hz
        | Bpm(bpm) -> bpm / 60.
        | Period(t) -> 1.0 / t

    member this.inBpm() =
        match this with
        | Hz(hz) -> hz * 60.
        | Bpm(bpm) -> bpm
        | Period(t) -> 60. / t

[<RequireQualifiedAccess>]
type Datatype =
    | Wiggle of Wiggle.Datatype
    | Rate
    | Button
    | UFloat
    | Picker of string list

type Data =
    | Wiggle of Wiggle.Data
    | Rate of Rate
    | Button of bool
    | UFloat of float
    | Picker of string

type KnobDescription = {
    name: string
    datatype: Datatype
}

// Flat modules for each type of knob to keep things simple.

/// View a knob whose model is a slide.
/// Partially apply the function that wraps the outgoing data message with the appropriate variant.
let viewSlider dataWrapper name model dispatchLocal dispatchChange =
    // When we move the slider, emit a server command.
    let onValueChange v =
        dataWrapper v
        |> dispatchChange
    R.div [] [
        R.str name
        Slider.view onValueChange model dispatchLocal
    ]

module Unipolar =
    let initModel() = Slider.initModel 0.0 0.0 1.0 0.0001 [0.0; 1.0]
    let view name model dispatchLocal dispatchChange =
        viewSlider (Wiggle.Data.Unipolar >> Wiggle) name model dispatchLocal dispatchChange

module Bipolar =
    let initModel() = Slider.initModel 0.0 -1.0 1.0 0.0001 [-1.0; 0.0; 1.0]
    let view name model dispatchLocal dispatchChange =
        viewSlider (Wiggle.Data.Bipolar >> Wiggle) name model dispatchLocal dispatchChange

module Rate =
    // Use BPM for rate for the time being.
    let initModel() = Slider.initModel 60.0 0.0 200.0 0.01 []
    let view name model dispatchLocal dispatchChange =
        viewSlider (Bpm >> Rate) name model dispatchLocal dispatchChange

module UFloat =
    // Positive float is a rather general-purpose knob.
    // The only current use is clock multiplication.
    // For now, go from 0 to 4 and provide some logarithmic detents.
    let initModel() = Slider.initModel 1.0 0.0 4.0 0.001 [0.0; 0.25; 0.5; 1.0; 2.0; 4.0]
    let view name model dispatchLocal dispatchChange =
        viewSlider UFloat name model dispatchLocal dispatchChange

module Button =
    // Button just stores state as a bool, which eagerly updates.
    type Model = bool
    let initModel() = false
    type Message = bool
    let update message _ = message
    let view name state dispatchLocal dispatchChange =
        R.button [
            (if state then Bootstrap.Button.Info else Bootstrap.Button.Default)
            // Toggle state on click.
            OnClick (fun _ ->
                (not state) |> dispatchLocal
                (not state) |> Button |> dispatchChange)
        ] [ R.str name ]
            
module Picker =
    // Picker stores both the current set of options as well as the option that is selected.
    // TODO: allow customizing the view for options besides dropdown (button bank, for example).
    type Model = {
        selected: string
        options: string list
    }

    let initModel (options: string list) =
        let selected =
            if options.IsEmpty then
                logError "Constructing a picker knob with an empty option list."
                ""
            else
                List.head options
        {selected = selected; options = options}

    type Message = string
    let update message model =
        // Ignore values outside of our set of options.
        if model.options |> List.contains message then
            {model with selected = message}
        else
            logError (sprintf
                "Picker knob got a bad value: '%s', expected one of %O." message model.options)
            model
    /// Render this picker as a select.
    let view name model dispatchLocal dispatchChange =
        R.div [] [
            R.str name
            R.select [
                Form.Control
                OnChange (fun e ->
                    let selected: string = !!e.target?value
                    selected |> dispatchLocal
                    selected |> Picker |> dispatchChange)
                Value (Case1 model.selected)
            ] (model.options |> List.map (fun s -> R.option [ Value (Case1 s) ] [ R.str s ]))
        ]
        
/// Flattened combination of datatype info and current state.
/// Illegal combinations are unrepresentable.
[<RequireQualifiedAccess>]
type ViewModel =
    | Unipolar of Slider.Model
    | Bipolar of Slider.Model
    | Rate of Slider.Model
    | Button of Button.Model
    | UFloat of Slider.Model
    | Picker of Picker.Model
  
type Model = {
    name: string
    data: ViewModel
}

/// Create a new knob model from a description.
let fromDesc (d: KnobDescription) : Model =
    let initData =
        match d.datatype with
        | Datatype.Wiggle(Wiggle.Datatype.Unipolar) -> Unipolar.initModel() |> ViewModel.Unipolar
        | Datatype.Wiggle(Wiggle.Datatype.Bipolar) -> Bipolar.initModel() |> ViewModel.Bipolar
        | Datatype.Rate -> Rate.initModel() |> ViewModel.Rate
        | Datatype.Button -> Button.initModel() |> ViewModel.Button
        | Datatype.UFloat -> UFloat.initModel() |> ViewModel.UFloat
        | Datatype.Picker(items) -> Picker.initModel items |> ViewModel.Picker

    {name = d.name; data = initData}

[<RequireQualifiedAccess>]
type Message =
    /// Internal slider event, shared among all knob types that are glorified sliders.
    | Slider of Slider.Message
    /// Internal button event.
    | Button of Button.Message
    /// Internal picker event.
    | Picker of Picker.Message

/// Update the state of this knob using the provided data.
/// This is directly called by a parent collection when it handles a server response to update the
/// state of a knob.
let updateFromValueChange data model =
    // Ensure that this value change matches the type of knob we have.  If this is a mismatch,
    // ignore it and log an error.
    match (data, model.data) with
    | Wiggle(Wiggle.Unipolar(u)), ViewModel.Unipolar(vm) ->
        let newDat = {vm with value = u}
        {model with data = ViewModel.Unipolar(newDat)}
    | Wiggle(Wiggle.Bipolar(b)), ViewModel.Bipolar(vm) ->
        let newDat = {vm with value = b}
        {model with data = ViewModel.Bipolar(newDat)}
    | Rate(r), ViewModel.Rate(vm) ->
        let newDat = {vm with value = r.inBpm()}
        {model with data = ViewModel.Rate(newDat)}
    | Button(b), ViewModel.Button(_) ->
        {model with data = ViewModel.Button(b)}
    | UFloat(u), ViewModel.UFloat(vm) ->
        // TODO: consider rescaling slider min/max?
        let newDat = {vm with value = u}
        {model with data = ViewModel.UFloat(newDat)}
    | Picker(p), ViewModel.Picker(picker) ->
        {model with data = Picker.update p picker |> ViewModel.Picker}
    | _ ->
        logError (sprintf
            "Invalid knob value change message for knob %s.  Current data: %+A"
            model.name
            model.data)
        model

let update message model =
    match message with
    | Message.Slider(msg) ->
        match model.data with
        | ViewModel.Unipolar(s) -> Some (ViewModel.Unipolar, s)
        | ViewModel.Bipolar(s) -> Some (ViewModel.Bipolar, s)
        | ViewModel.Rate(s) -> Some (ViewModel.Rate, s)
        | ViewModel.UFloat(s) -> Some (ViewModel.UFloat, s)
        | _ -> None
        |> Option.map (fun (kind, sliderModel) ->
            let updatedSlider = Slider.update msg sliderModel
            {model with data = updatedSlider |> kind})
        |> function
            | Some(model) -> model
            | None ->
                logError (sprintf "Knob %s ignored a slider message." model.name)
                model

    | Message.Button(buttonVal) ->
        match model.data with
        | ViewModel.Button(_) -> {model with data = ViewModel.Button(buttonVal)}
        | _ ->
            logError (sprintf "Knob %s ignored a button message." model.name)
            model
    | Message.Picker(msg) ->
        match model.data with
        | ViewModel.Picker(picker) ->
            {model with data = Picker.update msg picker |> ViewModel.Picker}
        | _ ->
            logError (sprintf "Knob %s ignored a picker message." model.name)
            model

/// Render a particular knob.
let view model dispatchLocal dispatchChange =
    match model.data with
    | ViewModel.Unipolar(slider) ->
        Unipolar.view model.name slider (Message.Slider >> dispatchLocal) dispatchChange
    | ViewModel.Bipolar(slider) ->
        Bipolar.view model.name slider (Message.Slider >> dispatchLocal) dispatchChange
    | ViewModel.Rate(slider) ->
        Rate.view model.name slider (Message.Slider >> dispatchLocal) dispatchChange
    | ViewModel.UFloat(slider) ->
        UFloat.view model.name slider (Message.Slider >> dispatchLocal) dispatchChange
    | ViewModel.Button(b) ->
        Button.view model.name b (Message.Button >> dispatchLocal) dispatchChange
    | ViewModel.Picker(p) ->
        Picker.view model.name p (Message.Picker >> dispatchLocal) dispatchChange
