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

[<RequireQualifiedAccess>]
type ServerCommand<'a> =
    | State
    | Set of 'a * Data

[<RequireQualifiedAccess>]
type ServerResponse<'a> =
    | State of ('a * KnobDescription) list
    | ValueChange of 'a * Data
    | Added of 'a * KnobDescription
    | Removed of 'a

//! Immutable binary tree of knobs, indexed by address.
//! We may need to investigate making this a mutable collection if performance under heavy update
//! load becomes a problem.
type Model<'a when 'a: comparison> = Map<'a, Knob.Model>

let initModel() = Map.empty

let initCommands = [ServerCommand.State]

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
    | Response(ServerResponse.State(state)) ->
        state
        |> List.map (fun (addr, desc) -> (addr, Knob.fromDesc desc))
        |> Map.ofList
    | Response(ServerResponse.ValueChange(addr, value)) ->
        model
        |> transformMapItem addr (Knob.updateFromValueChange value)
    | Response(ServerResponse.Added(addr, desc)) ->
        model
        |> Map.add addr (Knob.fromDesc desc)
    | Response(ServerResponse.Removed(addr)) ->
        model
        |> Map.remove addr
    | Particular(addr, msg) ->
        model
        |> transformMapItem addr (Knob.update msg)

/// Render a particular knob using the provided address.
let viewOne addr knob dispatchLocal dispatchServer =
    let dispatchChange data =
        (AllButSelf, ServerCommand.Set(addr, data))
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

/// Display every knob for which filter addr returns true.
let viewAllWith filter (model: Model<_>) dispatchLocal dispatchServer =
    model
    |> Map.toSeq
    |> Seq.filter (fun (addr, knob) -> filter addr)
    |> Seq.map (fun (addr, knob) -> viewOne addr knob dispatchLocal dispatchServer)
    |> List.ofSeq
    |> R.div []