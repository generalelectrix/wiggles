//! A model and message system for maintaining a collection of knobs.
module Knobs
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Util.fsx"
#load "Bootstrap.fsx"
#load "Types.fsx"
#load "Knob.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Types
open Util
open Bootstrap
open Knob

type ValueChange<'a> = {
    addr: 'a
    value: Data
}

type KnobAdded<'a> = {
    addr: 'a
    desc: KnobDescription
}

[<RequireQualifiedAccess>]
type ServerCommand<'a> =
    | Set of ValueChange<'a>


[<RequireQualifiedAccess>]
type ServerResponse<'a> =
    | ValueChange of ValueChange<'a>
    | KnobAdded of KnobAdded<'a>
    | KnobRemoved of 'a

//! Immutable binary tree of knobs, indexed by address.
//! We may need to investigate making this a mutable collection if performance under heavy update
//! load becomes a problem.
type Model<'a when 'a: comparison> = Map<'a, Knob.Model>

let initModel() = Map.empty

type Message<'a> =
    | Response of ServerResponse<'a>
    | Particular of 'a * Knob.Message

/// Operate on a particular knob if it is present, replacing its value with that returned by op.
/// Log an error if the address is missing and return the same knob collection unchanged.
let operateOnKnob addr op model =
    match model |> Map.tryFind addr with
    | Some(knob) ->
        model |> Map.add addr (op knob)
    | None ->
        logError (sprintf "Tried to operate on knob at address %+A but it is not present." addr)
        model

let update message model =
    match message with
    | Response(ServerResponse.ValueChange(vc)) ->
        model
        |> operateOnKnob vc.addr (Knob.updateFromValueChange vc.value)
    | Response(ServerResponse.KnobAdded(ka)) ->
        model
        |> Map.add ka.addr (Knob.fromDesc ka.desc)
    | Response(ServerResponse.KnobRemoved(addr)) ->
        model
        |> Map.remove addr
    | Particular(addr, msg) ->
        model
        |> operateOnKnob addr (Knob.update msg)

/// Render a particular knob using the provided address.
let viewOne addr knob dispatchLocal dispatchServer =
    let dispatchChange data =
        (AllButSelf, ServerCommand.Set({addr = addr; value = data}))
        |> dispatchServer
    let dispatchLocal msg = (addr, msg) |> Particular |> dispatchLocal 
    Knob.view knob dispatchLocal dispatchChange

/// Display a particular knob.
/// Return an empty div if the requested address is missing.
let view addr model dispatchLocal dispatchServer =
    match model |> Map.tryFind addr with
    | Some(knob) ->
        viewOne addr knob dispatchLocal dispatchServer
    | None ->
        logError (sprintf "Could not view knob at address %+A because it is not present." addr)
        R.div [] []