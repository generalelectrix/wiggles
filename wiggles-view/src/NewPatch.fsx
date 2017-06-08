module NewPatch
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "../node_modules/fable-react-toolbox/Fable.Helpers.ReactToolbox.fs"
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
open Types
open Bootstrap

type Model =
    /// Fixture types we have available to patch.
   {kinds: FixtureKind list;
    selectedKind: FixtureKind option;
    name: string;
    universe: UniverseId option;
    address: DmxAddress option;
    quantity: int;}
    with
    member this.TryGetNamedKind(name) = this.kinds |> List.tryFind (fun k -> k.name = name)
    
type Message = 
    | UpdateKinds of FixtureKind list
    | SetSelected of string
    | SetUniverse of UniverseId option
    | SetAddress of DmxAddress option
    | SetQuantity of int
    /// Convenience feature to advance the start address after patching.
    | AdvanceAddress

let initialModel () = {
    kinds = [];
    selectedKind = None;
    name = "";
    universe = None;
    address = None;
    quantity = 1;
}

let positive x = max x 0

let update message (model: Model) =
    match message with
    | UpdateKinds kinds -> {model with kinds = kinds |> List.sortBy (fun k -> k.name)}
    | SetSelected name ->
        match model.TryGetNamedKind(name) with
        | Some(kind) -> {model with selectedKind = Some kind}
        | None -> model
    | SetUniverse u -> {model with universe = u |> Option.map positive}
    | SetAddress a -> {model with address = a |> Option.map (min 512 >> max 1)}
    | SetQuantity q -> {model with quantity = q |> positive}
    | AdvanceAddress ->
        match model.address with
        | Some(addr) ->
            let channelCount =
                match model.selectedKind with
                | Some(k) -> k.channelCount
                | None -> 0
            {model with address = (addr + (model.quantity*channelCount)) |> min 512 |> Some}
        | None -> model

    |> fun m -> (m, Cmd.none)

let [<Literal>] EnterKey = 13.0
let [<Literal>] EscapeKey = 27.0

/// Render type selector dropdown.
let typeSelector (kinds: FixtureKind list) selectedKind dispatchLocal =
    let option (kind: FixtureKind) =
        R.option
            [ Value (Case1 kind.name) ]
            [ R.str (sprintf "%s (%d ch)" kind.name kind.channelCount) ]

    let selected = defaultArg selectedKind kinds.[0]
    R.div [] [
        R.select [
            Form.Control
            OnChange (fun e -> SetSelected !!e.target?value |> dispatchLocal)
            Value (Case1 selected.name)
        ] (kinds |> List.map option)
    ]

let numericEditBox dispatchLocal handleValue label cmd value =
    R.label [] [
        R.str label
        R.input [
            Form.Control
            Type "number"
            OnChange (fun e -> 
                !!e.target?value
                |> handleValue
                |> cmd
                |> dispatchLocal);
            Value (Case1 (value))
        ] []
    ]

let patchButton model dispatchLocal dispatchServer =
    R.button [
        Button.Warning
        OnClick (fun _ ->
            //match model.selectedKind with
            //| None -> ()
            //| Some(kind) ->
            //    AdvanceAddress |> dispatchLocal
            //    let requests =
            //        if model.quantity = 1 then
            //            {name = model.name;
            //             kind = kind.name;
            //             address = 


            //    let makeReq i address = 
            //    let patchRequest = {
            ()
        )
    ][ R.str "Patch" ]

let noneIfEmpty s = if s = "" then None else Some(int s)
let emptyIfNone = function | None -> "" | Some(x) -> string x

/// View function taking two different dispatch callbacks.
/// dispatchLocal dispatches a message local to this subapp.
/// dispatchServer sends a server request.
let view model dispatchLocal dispatchServer =

    if model.kinds.IsEmpty then
        R.div [] [R.str "No patch types available."]
    else
        let universeEntry = 
            numericEditBox
                dispatchLocal
                noneIfEmpty
                "Universe"
                SetUniverse
                (model.universe |> emptyIfNone)

        let addressEntry =
            numericEditBox
                dispatchLocal
                noneIfEmpty
                "Start address"
                SetAddress
                (model.address |> emptyIfNone)

        let quantityEntry =
            numericEditBox
                dispatchLocal
                (fun v -> if v = "" then 1 else int v)
                "Quantity"
                SetQuantity
                (string model.quantity)

        R.div [Form.Group] [
            R.span [] [ R.h3 [] [R.str "Create new patch"]]
            typeSelector model.kinds model.selectedKind dispatchLocal
            Grid.distribute [
                [ universeEntry ]
                [ addressEntry ]
                [ quantityEntry ]
            ]
        ]