/// Simple view for editing a field, with confirm and cancel buttons.
module SimpleEditor
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Util.fsx"
#load "Bootstrap.fsx"
#load "EditBox.fsx"
#load "Types.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Util
open Bootstrap
open Types
          
type Model<'a> = {
    editBox: EditBox.Model<'a>
    okText: string
}

let initModel okText label parser inputType = {
    editBox = EditBox.initialModel label parser inputType
    okText = okText
}

type Message<'a> = EditBox.Message<'a>

let update message model = {model with editBox = EditBox.update message model.editBox}

/// A button that, when pressed, will dispatch a message that is passed a successfully parsed value.
let private okButton (model: Model<'a>) onOk onComplete =
    /// When we click load, dispatch a server message to save this show as a new name.
    /// Also call the provided callback that we should execute on show save.
    let onClick _ =
        match model.editBox.value with
        | Some(Ok(newName)) ->
            /// If we've parsed a good value, go for it.
            newName |> onOk
            onComplete()
        | _ -> ()

    R.button
        [Button.Primary; OnClick onClick]
        [R.str model.okText]

/// Button to exit the load subsystem.
let cancelButton onComplete =
    let onClick _ = onComplete()
    R.button
        [Button.Default; OnClick onClick]
        [R.str "Cancel"]

let view defaultVal model onOk onComplete dispatch = 
    let okButton = okButton model onOk onComplete
    R.div [] [
        Grid.fullRow [EditBox.view None defaultVal model.editBox dispatch]
        Grid.layout [
            (1, [okButton])
            (1, [cancelButton onComplete])
        ]
    ]

 
    