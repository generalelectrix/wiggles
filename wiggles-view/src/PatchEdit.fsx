module PatchEdit
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "../node_modules/fable-react-toolbox/Fable.Helpers.ReactToolbox.fs"
#load "Types.fs"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
module RT = Fable.Helpers.ReactToolbox
open Types

let text x : (Fable.Import.React.ReactElement) = unbox x

type Model =
    /// Real details of the currently-selected patch item.
   {selected: PatchItem option;
    /// Name edit buffer, cleared when change request sent to server.
    nameEdit: string option;
    /// Address edit buffer, cleared when change request sent to server.
    addressEdit: DmxAddress option;
    universeEdit: UniverseId option;}
    
type Message =
    | SetState of PatchItem option
    | NameEdit of string option
    | AddressEdit of DmxAddress option
    | UniverseEdit of UniverseId option

let initialModel () =
   {selected = None;
    nameEdit = None;
    addressEdit = None;
    universeEdit = None}

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
            nameEdit = if clearBuffers then None else model.nameEdit;
            addressEdit = if clearBuffers then None else model.addressEdit;}

    | NameEdit n -> {model with nameEdit = n}
    | AddressEdit a -> {model with addressEdit = a}
    | UniverseEdit u -> {model with universeEdit = u}
    |> fun m -> (m, Cmd.none)

let enterKeyCode = float 13

let nameEditBox fixtureId name dispatchLocal dispatchServer =
    let clearNameEdit() = NameEdit None |> dispatchLocal
    R.input [
        Type "text";
        OnChange (fun e -> !!e.target?value |> Some |> NameEdit |> dispatchLocal);
        OnBlur (fun _ -> clearNameEdit());
        OnKeyDown (fun e ->
            if e.keyCode = enterKeyCode then
                clearNameEdit()
                Rename(fixtureId, name) |> dispatchServer)
        Value (Case1 name)
        ] []


/// View function taking two different dispatch callbacks.
/// dispatchLocal dispatches a message local to this subapp.
/// dispatchServer sends a server request.
let view model dispatchLocal dispatchServer =
    match model.selected with
    | None -> R.div [] [ text (sprintf "No fixture selected.") ]
    | Some(selected) ->
        R.div [] [
            R.div [] [ text (sprintf "Fixture id: %d" selected.id) ];
            R.div [] [ text (sprintf "Fixture type: %s" selected.kind) ];
            nameEditBox
                selected.id
                (defaultArg model.nameEdit selected.name)
                dispatchLocal
                dispatchServer
        ]