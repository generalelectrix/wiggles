/// A compact and minimal navbar using a drop-down only.
module Navbar
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

/// Is this item on the left or right side?
type Position =
    | Left
    | Right

type ItemId =
    /// Single item.
    | Single of int
    /// A particular item in a dropdown.
    | Dropdown of int * int

/// Address that allows us to route messages to nav items.
type ItemAddress = {position: Position; id: ItemId}

type ItemState =
    | Active
    | Inactive
    /// A nav item that just triggers an action but doesn't represent a persistent state such as
    /// being on a particular page.
    | AlwaysInactive

/// Single navbar item, can be styled as a main option or a dropdown item.
type Item<'msg> = {
    /// The text that will be displayed for the menu option.
    text: string
    /// Action to take when this item is clicked.  Passed the dispatch function as sole argument.
    onClick: Dispatch<'msg> -> unit
    state: ItemState}
    with
    member this.active = this.state = Active

type DropdownItem<'msg> =
    /// Separator line.
    | Separator
    /// Single selection in the drop-down.
    | Selection of Item<'msg>

type DropdownModel<'msg> = {
    items: DropdownItem<'msg> array
    dropped: bool
}

type NavItem<'msg> =
    | Single of Item<'msg>
    | Dropdown of DropdownModel<'msg>

/// The full model for a navbar.
type Model<'msg> = {
    leftItems: NavItem<'msg> array
    rightItems: NavItem<'msg> array
}

type Message =
    /// Set the active state of a particular nav item by address. Ignored if the item is always
    /// inactive.
    | SetActive of ItemAddress * bool
    /// Open or close a drop-down.  Ignored if the item at address is not a drop-down.
    | SetDropped of ItemAddress * bool

/// Call this model's action function and also potentially issue a message setting it to active.
let sendMessageSetState address (model: Item<'msg>) dispatch dispatchLocal =
    model.onClick dispatch
    match model.state with
    | Active | Inactive -> dispatchLocal (SetActive(address, true))
    | _ -> ()


/// Render a single navbar item.
let viewSingle address (model: Item<'msg>) dispatch dispatchLocal =
    let onClick e = sendMessageSetState address model dispatch dispatchLocal
    R.li 
        (if model.active then [ClassName "active"] else [])
        [R.a [Href "#"; OnClick onClick] [R.str model.text]]

/// Render a dropdown item.
let viewDropItem (model: Item<'msg>) dispatch dispatchLocal =
    let onClick e =
        // Send messages and update item state.
        sendMessageSetState address model dispatch dispatchLocal
        // Emit a message to close this drop-down.
    ()

/// Generate a navbar li that acts as a dropdown submenu.
let viewDropdown (model: DropdownModel<'msg>) = ()


let view (model: Model<'msg>) dispatch = ()
