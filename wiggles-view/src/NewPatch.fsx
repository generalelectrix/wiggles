module NewPatch
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "../node_modules/fable-react-toolbox/Fable.Helpers.ReactToolbox.fs"
#load "Util.fsx"
#load "Types.fsx"
#load "Bootstrap.fsx"
#load "EditBox.fsx"

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
open EditBox

type Model =
    /// Fixture types we have available to patch.
   {kinds: FixtureKind list
    selectedKind: FixtureKind option
    name: EditBox.Model<string>
    universe: EditBox.Model<UniverseId option>
    address: EditBox.Model<DmxAddress option>
    quantity: EditBox.Model<int>}
    with
    member this.TryGetNamedKind(name) = this.kinds |> List.tryFind (fun k -> k.name = name)
    
type Message = 
    | UpdateKinds of FixtureKind list
    | SetSelected of string
    | UnivEdit of EditBox.Message<UniverseId option>
    | AddrEdit of EditBox.Message<DmxAddress option>
    | QuantEdit of EditBox.Message<int>
    /// Convenience feature to advance the start address after patching.
    | AdvanceAddress

/// Return Error if number is less than 1.
let parsePositiveInt =
    parseInt
    >> Result.ofOption
    >> Result.bind (fun number -> if number < 1 then Error() else Ok(number))

let initialModel () = {
    kinds = [];
    selectedKind = None;
    name = EditBox.initialModel "Name:" errorIfEmpty "text";
    universe = EditBox.initialModel "Universe:" parseUniverseId "number";
    address = EditBox.initialModel "Address:" parseDmxAddress "number";
    quantity = EditBox.initialModel "Quantity:" parsePositiveInt "number";
}

let update message (model: Model) =
    match message with
    | UpdateKinds kinds -> {model with kinds = kinds |> List.sortBy (fun k -> k.name)}
    | SetSelected name ->
        match model.TryGetNamedKind(name) with
        | Some(kind) -> {model with selectedKind = Some kind}
        | None -> model
    | UnivEdit msg -> {model with universe = EditBox.update msg model.universe}
    | AddrEdit msg -> {model with address = EditBox.update msg model.address}
    | QuantEdit msg -> {model with quantity = EditBox.update msg model.quantity}
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

/// Create patch requests for 1 to N fixtures of the same kind with sequential addresses.
let newPatchesSequential (name: string) (kind: FixtureKind) n startAddress : Result<PatchRequest list,unit> =
    // Just do the naive thing and leave it up to the server to tell us if we made a mistake, like
    // address conflicts.
    let trimmedName = name.Trim()
    let name = if trimmedName = "" then kind.name else trimmedName
    if n < 1 then Error()
    elif n = 1 then
        Ok [{name = name; kind = kind.name; address = startAddress}]
    else
        // add a number into the name to keep things obvious
        let makeOne i : PatchRequest =
            let nameWithCount = sprintf "%s %d" name i
            let addr = startAddress |> Option.map (fun (u, a) -> (u, a + kind.channelCount))
            {name = nameWithCount; kind = kind.name; address = addr}
        [1..n]
        |> List.map makeOne
        |> Ok

let patchButton model dispatchLocal dispatchServer =
    R.button [
        Button.Warning
        OnClick (fun _ ->
            match model.selectedKind with
            | None -> ()
            | Some(kind) ->
                AdvanceAddress |> dispatchLocal
                match globalAddressFromOptions model.universe model.address with
                | Error(_) -> ()
                | Ok(address) ->
                    let newPatchResult =
                        newPatchesSequential model.name kind model.quantity address
                    match newPatchResult with
                    | Ok(patches) -> patches |> ServerRequest.NewPatches |> dispatchServer
                    | _ -> ()
            ()
        )
    ][ R.str "Patch" ]

/// View function taking two different dispatch callbacks.
/// dispatchLocal dispatches a message local to this subapp.
/// dispatchServer sends a server request.
let view model dispatchLocal dispatchServer =

    if model.kinds.IsEmpty then
        R.div [] [R.str "No patch types available."]
    else
        let universeEntry = EditBox.view None "" model.universe (UnivEdit >> dispatchLocal)
        let addressEntry = EditBox.view None "" model.address (AddrEdit >> dispatchLocal)
        let quantityEntry = EditBox.view None "1" model.quantity (QuantEdit >> dispatchLocal)

        R.div [Form.Group] [
            R.span [] [ R.h3 [] [R.str "Create new patch"]]
            typeSelector model.kinds model.selectedKind dispatchLocal
            Grid.distribute [
                [ universeEntry ]
                [ addressEntry ]
            ]
            Grid.distribute [
                [ quantityEntry ]
                [ patchButton model dispatchLocal dispatchServer ]
            ]
        ]