module PatchEdit
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Util.fsx"
#load "Types.fsx"
#load "Bootstrap.fsx"
#load "EditBox.fsx"
#load "Modal.fsx"

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

type Model = {
    /// Real details of the currently-selected patch item.
    selected: PatchItem option
    nameEdit: EditBox.Model<string>
    addressEdit: EditBox.Model<Optional<DmxAddress>>
    universeEdit: EditBox.Model<Optional<UniverseId>>
}
    
type Message =
    | SetState of PatchItem option
    | NameEdit of EditBox.Message<string>
    | AddressEdit of EditBox.Message<Optional<DmxAddress>>
    | UniverseEdit of EditBox.Message<Optional<UniverseId>>

let initialModel () = {
    selected = None
    nameEdit = EditBox.initialModel "Name:" (fun s -> Ok(s)) "text"
    addressEdit = EditBox.initialModel "Address:" parseDmxAddress "number"
    universeEdit = EditBox.initialModel "Universe:" parseUniverseId "number"
}

let update message (model: Model) =
    let clear submodel = EditBox.update EditBox.Clear submodel

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
                nameEdit = clear model.nameEdit
                addressEdit = clear model.addressEdit
                universeEdit = clear model.universeEdit}
        else
            updatedModel
    | NameEdit n -> {model with nameEdit = EditBox.update n model.nameEdit}
    | AddressEdit a -> {model with addressEdit = EditBox.update a model.addressEdit}
    | UniverseEdit u -> {model with universeEdit = EditBox.update u model.universeEdit}
    |> fun m -> (m, Cmd.none)

/// When the enter key is pressed, submit a rename request if we've made an edit.
/// When the escape key is pressed, clear any existing edit.
let private nameEditOnKeyDown
        fixtureId
        dispatchLocal
        dispatchServer
        (nameEditModel: EditBox.Model<string>) =
    let clear() = EditBox.Clear |> NameEdit |> dispatchLocal
    OnKeyDown (fun event ->
        match event.keyCode with
        | EnterKey ->
            match nameEditModel.value with
            | Some(Ok(name)) ->
                clear()
                ServerRequest.Rename(fixtureId, name) |> dispatchServer
            | _ -> ()
        | EscapeKey ->
            clear()
        | _ -> ()
    ) :> IHTMLProp

let private nameEditBox selected model dispatchLocal dispatchServer = 
    let onKeyDown = nameEditOnKeyDown selected.id dispatchLocal dispatchServer
    EditBox.view
        (Some onKeyDown)
        selected.name
        model.nameEdit
        (NameEdit >> dispatchLocal)

let private addressEditor (selected: PatchItem) model dispatchLocal dispatchServer openModal =

    let universeBox =
        EditBox.view
            None
            (selected.universe |> emptyIfNone)
            model.universeEdit
            (UniverseEdit >> dispatchLocal)

    let addressBox =
        EditBox.view
            None
            (selected.dmxAddress |> emptyIfNone)
            model.addressEdit
            (AddressEdit >> dispatchLocal)

    let clear msg = EditBox.Clear |> msg |> dispatchLocal

    let clearAll() =
        clear UniverseEdit
        clear AddressEdit

    let handleRepatchButtonClick _ =
        // If either edit box are in a bad state, don't do anything.
        if not model.addressEdit.IsOk || not model.universeEdit.IsOk then ()
        else
            // Get values for both
            let univ = model.universeEdit.ParsedValueOr(selected.universe |> Optional.ofOption)
            let addr = model.addressEdit.ParsedValueOr(selected.dmxAddress |> Optional.ofOption)
            // We can only do something if both are some or both are none.
            match globalAddressFromOptionals univ addr with
            | Ok a ->
                ServerRequest.Repatch(selected.id, a) |> dispatchServer
                clearAll()
            | _ -> ()

    let repatchButton =
        R.button [
            Button.Warning
            OnClick handleRepatchButtonClick
        ] [ R.str "Repatch"]

    let removeButton =
        R.button [
            Button.Danger
            OnClick (fun e ->
                e.currentTarget?blur()
                let confirmMessage =
                    sprintf
                        "Are you sure you want to delete fixture %d (%s)?"
                        selected.id
                        selected.name
                let removeAction _ =
                    ServerRequest.Remove selected.id |> dispatchServer
                Modal.confirm confirmMessage removeAction
                |> openModal
            )
        ] [ R.str "Remove" ]

    R.div [Form.Group] [
        universeBox
        addressBox
        repatchButton
        removeButton
    ]
    

///<summary>
/// Display the patch editor.
///</summary>
let view model dispatchLocal dispatchServer openModal =
    let header = R.h3 [] [ R.str "Edit patch" ]
    let editor =
        match model.selected with
        | None -> R.str (sprintf "No fixture selected.")
        | Some(selected) ->
            R.div [] [
                Grid.layout [
                    (3, [R.str (sprintf "Id: %d" selected.id)])
                    (9, [R.str (sprintf "Type: %s" selected.kind)])
                ]
                nameEditBox selected model dispatchLocal dispatchServer
                addressEditor selected model dispatchLocal dispatchServer openModal
            ]
    R.div [] [
        header
        editor
    ]