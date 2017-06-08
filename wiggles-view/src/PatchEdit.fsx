module PatchEdit
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
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
open Util
open Types
open Bootstrap

type EditField<'T> =
    | Absent
    | Present of 'T

type Model = {
    /// Real details of the currently-selected patch item.
    selected: PatchItem option
    /// Name edit buffer, cleared when change request sent to server.
    nameEdit: EditField<string>
    /// Address edit buffer, cleared when change request sent to server.
    addressEdit: EditBox.Model<DmxAddress>
    universeEdit: EditBox.Model<UniverseId>
}
    
type Message =
    | SetState of PatchItem option
    | NameEdit of EditField<string>
    | AddressEdit of EditBox.Message<DmxAddress>
    | UniverseEdit of EditBox.Message<UniverseId>

let initialModel () = {
    selected = None
    nameEdit = Absent
    addressEdit =
        EditBox.initialModel
            "Address:"
            parseDmxAddress >> Result.ofOption
            "number"
    universeEdit = 
        EditBox.initialModel
            "Universe:"
            parseUniverseId >> Result.ofOption
            "number"
}

let update message (model: Model) =

    match message with
    | SetState newState ->
        // We may want to clear our edit buffers if we're getting a different fixture swapped in.
        let clearBuffers =
            match model.selected, newState with
            | Some(current), Some(updated) -> if current.id <> updated.id then true else false
            | _ -> true
        
        
        let updatedModel = {model with selected = newState}
        if clearBuffers then
            {updatedModel with
                nameEdit = Absent
                addressEdit = EditBox.update EditBox.Clear model.addressEdit
                universeEdit = EditBox.update EditBox.Clear model.universeEdit}
        else
            updatedModel
    | NameEdit n -> {model with nameEdit = n}
    | AddressEdit a -> {model with addressEdit = EditBox.update a model.addressEdit}
    | UniverseEdit u -> {model with universeEdit = EditBox.update a model.universeEdit}
    |> fun m -> (m, Cmd.none)

let [<Literal>] EnterKey = 13.0
let [<Literal>] EscapeKey = 27.0

let nameEditBox fixtureId name dispatchLocal dispatchServer =
    let clear() = NameEdit Absent |> dispatchLocal
    R.div [] [
        R.str "Name:"
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
        R.str label
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

    let universeBox =
        EditBox.view
            []
            (selected.universe |> emptyIfNone)
            model.universeEdit
            (UniverseEdit >> dispatchLocal)

    let addressBox =
        EditBox.view
            []
            (selected.dmxAddress |> emptyIfNone)
            model.addressEdit
            (AddressEdit >> dispatchLocal)

    let clear msg = EditBox.Clear |> msg |> dispatchLocal

    let clearAll() =
        clear UniverseEdit
        clear AddressEdit

    let handleRepatchButtonClick _ =
        // If either edit box are in a bad state, don't do anything.
        if !model.addressEdit.IsOk || !model.universeEdit.IsOk then ()
        else
            // Get values for both
            let univ = model.universeEdit.ParsedValue |> Option.orElse selected.universe
            let addr = model.addressEdit.ParsedValue |> Option.orElse selected.dmxAddress
            // We can only do something if both are some or both are none.
            match globalAddressFromOptions univ addr with
            | Ok a ->
                Repatch(selected.id, a) |> dispatchServer
                clearAll()
            | _ -> ()

    R.div [Form.Group] [
        universeBox
        addressBox
        R.button [
            Button.Warning
            OnClick handleRepatchButtonClick
        ] [ R.str "Repatch"]
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