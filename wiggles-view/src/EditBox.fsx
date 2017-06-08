/// An input component that keeps track of its state as well as whether or not its current state is
/// valid.
module EditBox
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "../node_modules/fable-react-toolbox/Fable.Helpers.ReactToolbox.fs"
#load "Util.fsx"
#load "Types.fsx"
#load "Bootstrap.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
module RT = Fable.Helpers.ReactToolbox
open Util
open Types
open Bootstrap

type Model<'T> = {
    /// The current value, if one is set.
    /// If None, no entry action has been performed yet and a default will be shown.
    /// If Some(Ok(...)), then entry has been performed and it has been parsed successfully.
    /// If Some(Err(...)), then entry has been performed but parsing was not successful.
    value: Result<'T, string> option
    parser: string -> Result<'T, unit>
    label: string
    inputType: string}
    with
    /// Is this edit box parsed successfully or never touched?
    member this.IsOk =
        match this.value with
        | Some(Error(_)) -> false
        | _ -> true
    /// Has this edit box successfully parsed a value?
    member this.HasParsed =
        match this.value with
        | Some(Ok(_)) -> true
        | _ -> false
    /// Return this edit box's parsed value or a default.
    member this.ParsedValueOr(defaultValue) =
        match this.value with
        | Some(Ok(v)) -> v
        | _ -> defaultValue

    
type Message<'T> = 
    | Update of string
    | Clear

let initialModel
        label parser inputType =
    {value = None; parser = parser; label = label; inputType = inputType}

let update message (model: Model<'T>) =
    match message with
    | Update v ->
        let parseResult = match model.parser v with | Error(_) -> Error(v) | Ok(x) -> Ok(x)
        {model with value = Some(parseResult)}
    | Clear -> {model with value = None}

/// Draw this edit box.
/// Optionally provide an additional attribute to attach to the input, such as a keypress handler.
let view (extraAction: (Model<'T> -> IHTMLProp) option) defaultValue (model: Model<'T>) dispatch =
    let value =
        match model.value with
        | None -> defaultValue
        | Some(Ok(v)) -> v.ToString()
        | Some(Error(v)) -> v

    let (attrs: IHTMLProp list) = [
        Form.Control
        Type model.inputType
        OnChange (fun e -> !!e.target?value |> Update |> dispatch)
        Value (Case1 (value))
    ]

    let allAttrs =
        match extraAction with
        | Some(action) -> [action model] ++ attrs
        | None -> attrs

    R.div [
        (if model.IsOk then Form.Group else Form.GroupError)
    ] [
        R.label [Form.ControlLabel] [
            R.str model.label
            R.input allAttrs []
        ]
    ]