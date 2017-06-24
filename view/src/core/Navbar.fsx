/// A compact and minimal navbar using a drop-down only.
module Navbar
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Util.fsx"
#load "Bootstrap.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Util
open Bootstrap

/// Single navbar item that has an action when clicked; can be styled as a single nav item or as an
/// entry in a drop-down.
type Item<'msg> = {
    /// The text that will be displayed for the menu option.
    text: string
    /// Action to take when this item is clicked.  Passed the dispatch function as sole argument.
    onClick: Dispatch<'msg> -> unit
}

/// Dropdowns can have regular items as well as spacers.
type DropdownItem<'msg> =
    /// Separator line.
    | Separator
    /// Single selection in the drop-down.
    | Selection of Item<'msg>

/// Model for a dropdown menu.
type DropdownModel<'msg> = {
    text: string 
    items: DropdownItem<'msg> list
    isOpen: bool
}

/// Nav items can be single buttons or dropdown menus.
type NavItem<'msg> =
    | Single of Item<'msg>
    | Dropdown of DropdownModel<'msg>

/// Position and index to refer to a nav item.
type Position =
    | Left of int
    | Right of int

/// The full model for a navbar.
type Model<'msg> = {
    leftItems: NavItem<'msg> list
    rightItems: NavItem<'msg> list
    /// At least one item is selected at all times in this model.
    /// Nothing is drawn as active if this position and index doesn't correspond to an item we have.
    activeItem: Position
}

type Message =
    /// Set a particular nav item as active by position.
    | SetActive of Position
    /// Open a drop-down.  Ignored if the item at position is not a drop-down.
    | OpenDropdown of Position
    /// Close a drop-down.  Ignored if the item at position is not a drop-down.
    | CloseDropdown of Position
    /// Toggle a drop-down's open state.  Ignored if the item at position is not a drop-down.
    | ToggleDropdown of Position

/// Apply f to the item at index if it is a dropdown.
/// Return an updated list of items with the change made.
let private mapDropdown index f collection =
    collection
    |> List.mapi (fun i item ->
        if i = index then
            match item with
            | Single(_) -> item
            | Dropdown(i) -> Dropdown(f i)
        else item)

/// Update either the left or right nav item that matches pos with the given update function.
let private updateDropdown f pos model =
    match pos with
    | Left(i) -> {model with leftItems = mapDropdown i f model.leftItems}
    | Right(i) -> {model with rightItems = mapDropdown i f model.rightItems}

let update message model =
    match message with
    | SetActive(pos) -> {model with activeItem = pos}
    | OpenDropdown(pos) -> updateDropdown (fun d -> {d with isOpen = true}) pos model
    | CloseDropdown(pos) -> updateDropdown (fun d -> {d with isOpen = false}) pos model
    | ToggleDropdown(pos) -> updateDropdown (fun d -> {d with isOpen = not d.isOpen}) pos model

/// Render a single navbar item.
let private viewSingle active position (model: Item<'msg>) dispatch dispatchLocal =
    let onClick e =
        // dispatch this item's action
        model.onClick dispatch
        // set this item as active
        position |> SetActive |> dispatchLocal

    R.li 
        (if active then [ClassName "active"] else [])
        [R.a [Href "#"; OnClick onClick] [R.str model.text]]

/// Render an item as a dropdown entry.
let private viewItemAsDropdownEntry position dispatch dispatchLocal (model: Item<'msg>) =
    let onClick e =
        // dispatch this item's action
        model.onClick dispatch
        // Emit a message to close this drop-down.
        CloseDropdown position |> dispatchLocal

    R.li [] [R.a [Href "#"; OnClick onClick] [R.str model.text]]

/// Render a dropdown item.
let viewDropdownItem position dispatch dispatchLocal (model: DropdownItem<'msg>) =
    match model with
    | Selection(item) -> viewItemAsDropdownEntry position dispatch dispatchLocal item
    | Separator -> R.li [Role "separator"; ClassName "divider"] []
 
/// Render a whole dropdown.
let private viewDropdown position (model: DropdownModel<'msg>) dispatch dispatchLocal =
    // The link that forms the button that opens the dropdown.
    let dropdownItem =
        R.a [
            Href "#"
            ClassName "dropdown-toggle"
            Role "button"
            AriaHasPopup true
            AriaExpanded model.isOpen
            OnClick (fun _ -> ToggleDropdown position |> dispatchLocal)
        ] [
            R.str model.text
            R.span [ClassName "caret"] []
    ]
    let subitems = model.items |> List.map (viewDropdownItem position dispatch dispatchLocal)

    R.li [
        ClassName (if model.isOpen then "dropdown open" else "dropdown")
    ] [
        dropdownItem
        R.ul
            [ClassName "dropdown-menu"; OnBlur (fun _ -> CloseDropdown position |> dispatchLocal)]
            subitems
    ]

let private viewNavSection
        leftSide (active: int option) (items: NavItem<'msg> list) dispatch dispatchLocal =
    let viewItem index item =
        let position = if leftSide then Left(index) else Right(index)
        match item with
        | Single(item) ->
            let isActive = Some(index) = active
            viewSingle isActive position item dispatch dispatchLocal
        | Dropdown(dropModel) ->
            viewDropdown position dropModel dispatch dispatchLocal

    R.ul
        [ClassName (if leftSide then "nav navbar-nav" else "nav navbar-nav navbar-right")]
        (items |> List.mapi viewItem)

let view (model: Model<'msg>) dispatch dispatchLocal =
    let (leftActive, rightActive) =
        match model.activeItem with
        | Left(i) -> (Some(i), None)
        | Right(i) -> (None, Some(i))
    let divLeftRight =
        R.div [] [
            viewNavSection true leftActive model.leftItems dispatch dispatchLocal
            viewNavSection false rightActive model.rightItems dispatch dispatchLocal
        ]

    R.nav
        [ClassName "navbar navbar-default navbar-static-top"]
        [R.div [Container.Fluid] [divLeftRight]]