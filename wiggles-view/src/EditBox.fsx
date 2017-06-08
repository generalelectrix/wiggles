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
    parser: string -> Result<'T, ()>
    label: string
    inputType: string}
    with
    /// Is this edit box parsed successfully or never touched?
    member this.IsOk =
        match this.value with
        | Some(Error(_)) -> false
        | _ -> true
    /// Does this edit box have a successfully parsed value?
    member this.ParsedValue =
        match this.value with
        | Some(Ok(v)) -> Some(v)
        | _ -> None
    
type Message<'T> = 
    | Update of string
    | Clear

let initialModel
        label parser inputType =
    {value = None; parser = parser; label = label; inputType = inputType}

let update message (model: Model) =
    match message with
    | Update v ->
        let parseResult = v |> model.parser |> Result.mapError (fun _ -> v)
        {model with value = Some(parseResult)}
    | Clear -> {model with value = None}

/// Draw this edit box.
/// Optionally provide additional attributes to attach to the input, such as keypress handlers.
let view inputAttrs defaultValue model dispatch =
    let divProps, value =
        match model.value with
        | Some(Ok(x)) -> [Form.Group], string x
        | Some(Err(v)) -> [Form.Group; Form.Error], v
        | None -> [Form.Group], string defaultValue

    let attrs = [
        Form.Control
        Type model.inputType
        OnChange (fun e -> !!e.target?value |> Update |> dispatch)
        Value (Case1 (value))
    ]

    R.div [divProps] [
        R.label [] [
            R.str model.label
            R.input (attrs ++ inputAttrs) []
        ]
    ]