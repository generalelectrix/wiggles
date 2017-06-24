//! A table of fixed height, with selectable items, that will present a scrollbar for many rows.
module Table
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

type IRow =
    abstract member ToStrings: unit -> string list

type Model = {
    /// Height of the table in pixels.
    height: int
    /// The header items of the table.
    header: string list
    /// The selected row, if any.
    selected: int option
}

type Message =
    | Select of int
    | Deselect

let update message model =
    match message with
    | Select(i) -> {model with selected = Some(i)}
    | Deselect -> {model with selected = None}

/// Render row.
let private viewRow dispatch selected (index: int) row =
    let rowAttrs: IHTMLProp list =
        let onClick = OnClick (fun _ -> index |> Select |> dispatch)
        if Some(index) = selected then [onClick; Table.Row.Active] else [onClick]

    let rowItems =
        (row :> IRow).ToStrings()
        |> List.map (fun x -> R.td [] [R.str x])

    R.tr rowAttrs rowItems

let private viewHeader header =
    header
    |> List.map (fun x -> R.th [] [R.str x])
    |> R.tr []

let view (rows: 'row list when 'row :> IRow) (model: Model) dispatch =
    let styles: ICSSProp list = [Height model.height; Overflow "scroll"]
    R.div [Style styles] [
        R.table [Table.Condensed] [
            R.thead [] [viewHeader model.header]
            R.tbody [] (rows |> List.mapi (viewRow dispatch model.selected))
        ]
    ]