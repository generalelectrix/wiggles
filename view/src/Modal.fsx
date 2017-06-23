/// A flexible, popover modal dialog box.  Intended to be used at the top level of an application
/// to allow multiple clients to use it simultaneously.
/// Keeps a stack of pending dialog actions, and displays them until it has run out.
module Modal
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Util.fsx"
#load "Types.fsx"
#load "Bootstrap.fsx"

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

type ModalAction = {
    label: string
    /// A Bootstrap button style to apply to this button.
    buttonType: IHTMLProp
    /// An action to fire on button click.
    action: React.MouseEvent -> unit
}

type ModalRequest = {
    message: string
    // Once the abomination that is KeyValueList is gone, these can become a list.
    action0: ModalAction
    action1: ModalAction option
}

/// If the array is not empty, the 0th modal dialog will be displayed.
type Model = ModalRequest array

type Message = 
    /// Add a pending modal dialog interaction.
    | Open of ModalRequest
    /// Set focus on the OK button of an open modal dialog.
    | Focus
    /// Closes the currently-open modal dialog.
    /// Clicking any model dialog button will always emit this message.
    | Close

/// Create a message to open a dialog to confirm or cancel an action.
let confirm message action =
    let okAction = {
        label = "OK"
        buttonType = Button.Basic
        action = action}
    let cancelAction = {
        label = "Cancel"
        buttonType = Button.Default
        action = ignore}
    {message = message; action0 = okAction; action1 = Some cancelAction}

/// Create a request to open a dialog that provides a message and prompts the user to dismiss it.
let prompt message =
    let okAction = {
        label = "OK"
        buttonType = Button.Basic
        action = ignore}
    {message = message; action0 = okAction; action1 = None}

let initialModel() = Array.empty

let modalOkButtonId = "modal-ok-button"

let update message (model: Model) =
    match message with
    | Open(req) ->
        let newModel = req |> Array.singleton |> Array.append model
        newModel
    | Close -> (model |> Array.skip 1)
    | Focus ->
        enqueueBrowserAction (fun () -> Browser.document.getElementById(modalOkButtonId).focus())
        model

/// Draw a button to present a modal action option, such as "Ok" or "Cancel".
/// Clicking will run the provided action as well as close the dialog.
/// Optionally assign the button an id.
let private modalActionButton dispatch (id: string option) action =
    let onClick =
        OnClick (fun e ->
            Close |> dispatch
            action.action e
        )

    let buttonAttrs: IHTMLProp list =
        match id with
        | Some(id) -> [Id id; action.buttonType; onClick]
        | None -> [action.buttonType; onClick]

    R.button buttonAttrs [ R.str action.label ]

/// Display a float-on-top Bootstrap modal dialog.
let view (model: Model) dispatch =
    if model |> Array.isEmpty then
        R.div [] []
    else
        let state = model.[0]
        let message = R.p [] [R.str state.message]
        let bodyContents =
            let okButton = modalActionButton dispatch (Some modalOkButtonId) state.action0
            match state.action1 with
            | Some(action1) ->
                [message
                 okButton
                 modalActionButton dispatch None action1]
            | None -> [message; okButton]

        // Emit a message to focus on our about-to-be-created OK button.
        Focus |> dispatch
           
        R.div [
            ClassName "modal in"
            Role "dialog"
            Style [Display "block"]
        ] [
            R.div [ClassName "modal-dialog"] [
                R.div [ClassName "modal-content"] [
                    R.div [ClassName "modal-body"] bodyContents
                ]
            ]
        ]


/// Helper function that just renders a modal-style splash with no options, such as if the
/// application is waiting for a server connection.
let viewSplash message =
    R.div [
        ClassName "modal in"
        Role "dialog"
        Style [Display "block"]
    ] [
        R.div [ClassName "modal-dialog"] [
            R.div [ClassName "modal-content"] [
                R.div [ClassName "modal-body"] [R.p [] [R.str message]]
            ]
        ]
    ]