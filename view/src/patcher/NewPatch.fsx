module NewPatch
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "../core/Types.fsx"
#load "../core/Util.fsx"
#load "../core/Bootstrap.fsx"
#load "../core/EditBox.fsx"
#load "PatchTypes.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Types
open Util
open PatchTypes
open Bootstrap
open EditBox

type Model =
    /// Fixture types we have available to patch.
   {kinds: FixtureKind array
    selectedKind: FixtureKind option
    name: EditBox.Model<string>
    universe: EditBox.Model<Optional<UniverseId>>
    address: EditBox.Model<Optional<DmxAddress>>
    quantity: EditBox.Model<int>}
    with
    member this.TryGetNamedKind(name) = this.kinds |> Array.tryFind (fun k -> k.name = name)
    
type Message = 
    | UpdateKinds of FixtureKind array
    | SetSelected of string
    | NameEdit of EditBox.Message<string>
    | UnivEdit of EditBox.Message<Optional<UniverseId>>
    | AddrEdit of EditBox.Message<Optional<DmxAddress>>
    | QuantEdit of EditBox.Message<int>
    /// Convenience feature to advance the start address after patching.
    | AdvanceAddress

/// Return Error if number is less than 1.
let private parsePositiveInt =
    parseInt
    >> Result.ofOption
    >> Result.bind (fun number -> if number < 1 then Error() else Ok(number))

let initialModel () = {
    kinds = [||]
    selectedKind = None
    name =
        EditBox.initialModel "Name:" errorIfEmpty InputType.Text
        |> EditBox.setFailed ""
    universe =
        EditBox.initialModel "Universe:" parseUniverseId InputType.Number
        |> EditBox.setParsed Absent
    address =
        EditBox.initialModel "Address:" parseDmxAddress InputType.Number
        |> EditBox.setParsed Absent
    quantity =
        EditBox.initialModel "Quantity:" parsePositiveInt InputType.Number
        |> EditBox.setParsed 1
}

let update message (model: Model) =
    match message with
    | UpdateKinds kinds ->
        let sortedKinds = kinds |> Array.sortBy (fun k -> k.name)
        {model with
            kinds = sortedKinds
            selectedKind = if sortedKinds |> Array.isEmpty then None else Some sortedKinds.[0]}
    | SetSelected name ->
        match model.TryGetNamedKind(name) with
        | Some(kind) -> {model with selectedKind = Some kind}
        | None -> model
    | NameEdit msg -> {model with name = EditBox.update msg model.name}
    | UnivEdit msg -> {model with universe = EditBox.update msg model.universe}
    | AddrEdit msg -> {model with address = EditBox.update msg model.address}
    | QuantEdit msg -> {model with quantity = EditBox.update msg model.quantity}
    | AdvanceAddress ->
        match model.address, model.quantity, model.selectedKind with
        // We can only proceed if we have valid values for address, quantity, and have a kind selected.
        | Parsed(Present(addr)), Parsed(quantity), Some(kind) ->
            let newStartAddress =
                (addr + (quantity*kind.channelCount))
                |> min 512

            {model with address = EditBox.setParsed (Present newStartAddress) model.address}
        | _ -> model

    |> fun m -> (m, Cmd.none)

let [<Literal>] EnterKey = 13.0
let [<Literal>] EscapeKey = 27.0

/// Render type selector dropdown.
let private typeSelector (kinds: FixtureKind array) selectedKind dispatchLocal =
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
        ] (kinds |> Array.map option |> List.ofArray)
    ]

/// Create patch requests for 1 to N fixtures of the same kind with sequential addresses.
let private newPatchesSequential (name: string) (kind: FixtureKind) n startAddress : Result<PatchRequest array,unit> =
    // Just do the naive thing and leave it up to the server to tell us if we made a mistake, like
    // address conflicts.
    let trimmedName = name.Trim()
    let name = if trimmedName = "" then kind.name else trimmedName
    if n < 1 then Error()
    elif n = 1 then
        Ok [|{name = name; kind = kind.name; address = startAddress}|]
    else
        // add a number into the name to keep things obvious
        let makeOne i : PatchRequest =
            let nameWithCount = sprintf "%s %d" name (i+1)
            let addr = startAddress |> Option.map (fun (u, a) -> (u, a + kind.channelCount*i))
            {name = nameWithCount; kind = kind.name; address = addr}
        [| 0..n-1 |]
        |> Array.map makeOne
        |> Ok

/// Issues a server request for new patches if all data is correctly parsed and valid.
let private patchButton model dispatchLocal dispatchServer =
    R.button [
        Button.Warning
        OnClick (fun _ ->
            printfn "%+A" model
            match model.selectedKind, model.name, model.universe, model.address, model.quantity with
            | Some(kind), Parsed(name), Parsed(univ), Parsed(addr), Parsed(quant) ->
                match globalAddressFromOptionals univ addr with
                | Error(_) -> ()
                | Ok(globalAddress) ->
                    printfn "Addr: %+A" globalAddress
                    let newPatchResult =
                        newPatchesSequential name kind quant globalAddress
                    match newPatchResult with
                    | Ok(patches) ->
                        patches |> PatchServerRequest.NewPatches |> all |> dispatchServer
                        AdvanceAddress |> dispatchLocal
                    | _ -> ()
            | x ->
                printfn "%+A" x
                ()
        )
    ][ R.str "Patch" ]

/// View function taking two different dispatch callbacks.
/// dispatchLocal dispatches a message local to this subapp.
/// dispatchServer sends a server request.
let view model dispatchLocal dispatchServer =

    if model.kinds |> Array.isEmpty then
        R.div [] [R.str "No patch types available."]
    else
        let nameEntry = EditBox.view None "" model.name (NameEdit >> dispatchLocal)
        let universeEntry = EditBox.view None "" model.universe (UnivEdit >> dispatchLocal)
        let addressEntry = EditBox.view None "" model.address (AddrEdit >> dispatchLocal)
        let quantityEntry = EditBox.view None "" model.quantity (QuantEdit >> dispatchLocal)

        R.div [Form.Group] [
            R.span [] [ R.h3 [] [R.str "Create new patch"]]
            typeSelector model.kinds model.selectedKind dispatchLocal
            nameEntry
            Grid.distribute [
                [ universeEntry ]
                [ addressEntry ]
            ]
            Grid.distribute [
                [ quantityEntry ]
                [ patchButton model dispatchLocal dispatchServer ]
            ]
        ]