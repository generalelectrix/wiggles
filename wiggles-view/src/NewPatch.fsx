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

let text x : (Fable.Import.React.ReactElement) = unbox x

type Model =
    /// Fixture types we have available to patch.
   {kinds: FixtureKind list;
    selectedKind: FixtureKind option}
    with
    member this.TryGetNamedKind(name) = this.kinds |> List.tryFind (fun k -> k.name = name)
    
type Message = 
    | UpdateKinds of FixtureKind list
    | SetSelected of string

let initialModel () = {
    kinds = [];
    selectedKind = None;
}

let update message (model: Model) =
    match message with
    | UpdateKinds kinds -> {model with kinds = kinds |> List.sortBy (fun k -> k.name)}, Cmd.none
    | SetSelected name ->
        match model.TryGetNamedKind(name) with
        | Some(kind) -> {model with selectedKind = Some kind}, Cmd.none
        | None -> model, Cmd.none

let [<Literal>] EnterKey = 13.0
let [<Literal>] EscapeKey = 27.0

let option name = R.option [ Value (Case1 name) ] [ text name ]

/// Render type selector dropdown.
let typeSelector (kinds: FixtureKind list) selectedKind dispatchLocal =
    let selected = defaultArg selectedKind kinds.[0]
    R.div [] [
        R.div [] [
            R.select [
                Form.Control
                OnChange (fun e -> SetSelected !!e.target?value |> dispatchLocal)
                Value (Case1 selected.name)
            ] [
                for kind in kinds -> option kind.name
            ]
        ]
        R.div [] [
            R.span [] [text (sprintf "Required channels: %d" selected.channelCount)]
        ]
    ]

/// View function taking two different dispatch callbacks.
/// dispatchLocal dispatches a message local to this subapp.
/// dispatchServer sends a server request.
let view model dispatchLocal dispatchServer =
    if model.kinds.IsEmpty then R.div [] [text "No patch types available."]
    else
        R.div [Form.Group] [
            R.div [] [text "Create new patch"]
            typeSelector model.kinds model.selectedKind dispatchLocal
        ]