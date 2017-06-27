/// Create, view, alter, and delete DMX universes.
/// This is a view-only module, no local state is necessary.
module Universes
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "../core/Types.fsx"
#load "../core/Util.fsx"
#load "../core/Bootstrap.fsx"
#load "../core/EditBox.fsx"
#load "../core/Modal.fsx"
#load "PatchTypes.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Types
open Util
open PatchTypes
open Bootstrap
open EditBox

let private addButton dispatchServer =
    R.button [
        Button.Primary
        OnClick (fun _ -> PatchServerRequest.AddUniverse |> all |> dispatchServer)
    ] [R.str "Add universe"]

let private refreshPortsButton dispatchServer =
    R.button [
        Button.Default
        OnClick (fun _ -> PatchServerRequest.AvailablePorts |> all |> dispatchServer)
    ] [R.str "Refresh ports"]

let private deleteButton universeId openModal dispatchServer =
    R.button [
        Button.Default
        OnClick (fun _ ->
            Modal.confirm
                (sprintf "Are you sure you want to delete universe %d?" universeId)
                // For now, don't allow removing universes with fixtures in them at all.
                (fun _ -> PatchServerRequest.RemoveUniverse(universeId, false) |> all |> dispatchServer)
            |> openModal)
    ] [R.str "Delete"]

/// Render a port selection drop-down that fires a remap port message on change for this universe.
let private portSelector (universe: UnivWithPort) (ports: Port array) dispatchServer =
    let portOption (portIndex: int) ((portNamespace, portId): Port) =
        R.option
            [ Value (Case1 (string portIndex)) ]
            [ R.str (sprintf "%s: %s" portNamespace portId) ]

    let selected =
        ports
        |> Array.tryFindIndex (fun (portNamespace, portId) ->
            universe.portNamespace = portNamespace && universe.portId = portId)
        |> Option.map string
        |> withDefault ""

    if selected = "" then
        logError
            (sprintf
                "Could not find port entry for universe %d (%s: %s)."
                universe.universe
                universe.portNamespace
                universe.portId)

    R.div [] [
        R.select [
            Form.Control
            OnChange (fun e ->
                let selectedValue = !!e.target?value
                match parseInt selectedValue with
                | None -> logError (sprintf "Could not reparse universe index: %s" selectedValue)
                | Some(index) ->
                    match ports |> Array.tryItem index with
                    | None -> logError (sprintf "Port index %d is out of bounds." index)
                    | Some((portNamespace, portId)) ->
                        PatchServerRequest.AttachPort(
                            {universe with portNamespace = portNamespace; portId = portId})
                        |> all
                        |> dispatchServer
            )
            Value (Case1 selected)
        ] (ports |> Array.mapi portOption |> List.ofArray)
    ]

let tableHeader =
    ["id"; "port"; ""]
    |> List.map (fun item -> R.th [] [R.str item])
    |> R.tr []
    |> List.singleton
    |> R.thead []

/// Render a table row for a single universe.
let private universeRow ports openModal dispatchServer (universe: UnivWithPort) =
    let td item = R.td [] [item]
    R.tr [] [
        td (R.str (string universe.universe))
        td (portSelector universe ports dispatchServer)
        td (deleteButton universe.universe openModal dispatchServer)
    ]

/// Render the whole table of universes.
let private viewTable ports openModal dispatchServer universes =
    R.table [Table.Condensed] [
        tableHeader
        R.tbody [] (universes |> Seq.map (universeRow ports openModal dispatchServer) |> List.ofSeq)
    ]

/// Render the universe and port editor.
let view universes ports openModal dispatchServer =
    R.div [] [
        R.h4 [] [R.str "Universes"]
        viewTable ports openModal dispatchServer universes
        addButton dispatchServer
        refreshPortsButton dispatchServer
    ]

