module PatchEdit
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

// Fable converts options to nullable and thus chokes on nested options.
// Fake it with a fresh outer type.
type EditField<'T> =
    | Absent
    | Present of 'T

type Model =
    /// Real details of the currently-selected patch item.
   {selected: PatchItem option;
    /// Name edit buffer, cleared when change request sent to server.
    nameEdit: EditField<string>;
    /// Address edit buffer, cleared when change request sent to server.
    addressEdit: EditField<DmxAddress option>;
    universeEdit: EditField<UniverseId option>;}
    
type Message =
    | SetState of PatchItem option
    | NameEdit of EditField<string>
    | AddressEdit of EditField<DmxAddress option>
    | UniverseEdit of EditField<UniverseId option>

let initialModel () =
   {selected = None;
    nameEdit = Absent;
    addressEdit = Absent;
    universeEdit = Absent}

let update message (model: Model) =

    match message with
    | SetState newState ->
        // We may want to clear our edit buffers if we're getting a different fixture swapped in.
        let clearBuffers =
            match model.selected, newState with
            | Some(current), Some(updated) -> if current.id <> updated.id then true else false
            | _ -> true
        
        {model with
            selected = newState;
            nameEdit = if clearBuffers then Absent else model.nameEdit;
            addressEdit = if clearBuffers then Absent else model.addressEdit;
            universeEdit = if clearBuffers then Absent else model.universeEdit}

    | NameEdit n -> {model with nameEdit = n}
    | AddressEdit a -> {model with addressEdit = a}
    | UniverseEdit u -> {model with universeEdit = u}
    |> fun m -> (m, Cmd.none)

let [<Literal>] EnterKey = 13.0
let [<Literal>] EscapeKey = 27.0

let nameEditBox fixtureId name dispatchLocal dispatchServer =
    let clear() = NameEdit Absent |> dispatchLocal
    R.div [] [
        text "Name:"
        R.input [
            Form.Control
            Type "text";
            OnChange (fun e -> !!e.target?value |> Present |> NameEdit |> dispatchLocal);
            OnBlur (fun _ -> clear());
            OnKeyDown (fun e ->
                match e.keyCode with
                | EnterKey ->
                    clear()
                    Rename(fixtureId, name) |> dispatchServer
                | EscapeKey ->
                    clear()
                | _ -> ()
            )
            Value (Case1 name)
        ] []
    ]

let withDefault value editField =
    match editField with
    | Present(v) -> v
    | Absent -> value

let addressPieceEditBox label cmd addr dispatchLocal =
    let displayAddr = addr |> function | Some(a) -> string a | None -> ""
    R.label [] [
        text label
        R.input [
            Form.Control
            Type "number"
            OnChange (fun e -> 
                match !!e.target?value with | "" -> None | x -> Some(int x)
                |> Present
                |> cmd
                |> dispatchLocal);
            Value (Case1 displayAddr)
        ] []
    ]

let addressEditor (selected: PatchItem) model dispatchLocal dispatchServer =
    let displayUniv = model.universeEdit |> withDefault selected.universe
    let displayAddr = model.addressEdit |> withDefault selected.dmxAddress
    let clear msg = Absent |> msg |> dispatchLocal
    R.div [Form.Group] [
        addressPieceEditBox "Universe:" UniverseEdit displayUniv dispatchLocal
        addressPieceEditBox "Address:" AddressEdit displayAddr dispatchLocal
        R.button [
            Button.Basic
            OnClick (fun _ ->
                clear UniverseEdit
                clear AddressEdit
                match displayUniv, displayAddr with
                | Some(u), Some(a) -> Repatch(selected.id, Some(u, a)) |> dispatchServer
                | _ -> ()
            )
        ] [ text "Repatch"]
    ]


/// View function taking two different dispatch callbacks.
/// dispatchLocal dispatches a message local to this subapp.
/// dispatchServer sends a server request.
let view model dispatchLocal dispatchServer =
    let header = R.h3 [] [ text "Edit patch" ]
    let editor =
        match model.selected with
        | None -> text (sprintf "No fixture selected.")
        | Some(selected) ->
            R.div [] [
                Grid.layout [
                    (3, [text (sprintf "Id: %d" selected.id)])
                    (9, [text (sprintf "Type: %s" selected.kind)])
                ]
                nameEditBox
                    selected.id
                    (model.nameEdit |> withDefault selected.name)
                    dispatchLocal
                    dispatchServer
                addressEditor selected model dispatchLocal dispatchServer
            ]
    R.div [] [
        header
        editor
    ]