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

type Model = {
    /// Real details of the currently-selected patch item.
    selected: PatchItem option
    nameEdit: EditBox.Model<string>
    addressEdit: EditBox.Model<DmxAddress option>
    universeEdit: EditBox.Model<UniverseId option>
}
    
type Message =
    | SetState of PatchItem option
    | NameEdit of EditBox.Message<string>
    | AddressEdit of EditBox.Message<DmxAddress option>
    | UniverseEdit of EditBox.Message<UniverseId option>

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

let [<Literal>] EnterKey = 13.0
let [<Literal>] EscapeKey = 27.0

/// When the enter key is pressed, submit a rename request if we've made an edit.
/// When the escape key is pressed, clear any existing edit.
let nameEditOnKeyDown
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

let nameEditBox selected model dispatchLocal dispatchServer = 
    let onKeyDown = nameEditOnKeyDown selected.id dispatchLocal dispatchServer
    EditBox.view
        (Some onKeyDown)
        selected.name
        model.nameEdit
        (NameEdit >> dispatchLocal)

let addressEditor (selected: PatchItem) model dispatchLocal dispatchServer =

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
            let univ = model.universeEdit.ParsedValueOr(selected.universe)
            let addr = model.addressEdit.ParsedValueOr(selected.dmxAddress)
            // We can only do something if both are some or both are none.
            match globalAddressFromOptions univ addr with
            | Ok a ->
                ServerRequest.Repatch(selected.id, a) |> dispatchServer
                clearAll()
            | _ -> ()

    let repatchButton =
        R.button [
            Button.Warning
            OnClick handleRepatchButtonClick
        ] [ R.str "Repatch"]

    R.div [Form.Group] [
        universeBox
        addressBox
        repatchButton
    ]
    

/// View function taking two different dispatch callbacks.
/// dispatchLocal dispatches a message local to this subapp.
/// dispatchServer sends a server request.
let view model dispatchLocal dispatchServer =
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
                addressEditor selected model dispatchLocal dispatchServer
            ]
    R.div [] [
        header
        editor
    ]