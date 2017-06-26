/// DMX patcher.
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "../core/Bootstrap.fsx"
#load "PatchTypes.fsx"
#load "PatchEdit.fsx"
#load "NewPatch.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open PatchTypes
open Bootstrap

type Model = {
    patches: PatchItem array
    universes: UnivWithPort array
    availablePorts: Port array
    // Current fixture ID we have selected, if any.
    selected: FixtureId option
    // Model for the patch editor.
    editorModel: PatchEdit.Model
    newPatchModel: NewPatch.Model
}

type Message =
    | Response of PatchServerResponse
    | SetSelected of FixtureId
    | Deselect
    | Edit of PatchEdit.Message
    | Create of NewPatch.Message

// Commands needed to initialize the patcher.
let initCommands = [PatchServerRequest.PatchState; PatchServerRequest.GetKinds]

let initialModel () = {
    patches = Array.empty
    universes = Array.empty
    availablePorts = Array.empty
    selected = None
    editorModel = PatchEdit.initialModel()
    newPatchModel = NewPatch.initialModel()
}

/// Return a command to update the editor's state if fixture id is among those in patches.
let updateEditorState patches selectedFixtureId =
    selectedFixtureId
    |> Option.map (fun fixtureId ->
        patches |> Array.tryFind (fun p -> p.id = fixtureId))
    |> function | None -> None | Some x -> x // Option.flatten not supported by Fable, apparently.
    |> PatchEdit.SetState
    |> Edit
    |> Cmd.ofMsg

let private updateFromServerMessage message model =
    match message with
    | PatchServerResponse.PatchState(patches, universes) ->
        {model with patches = patches; universes = universes}, updateEditorState patches model.selected
    | PatchServerResponse.NewPatches patches ->
        {model with patches = patches |> Array.append model.patches}, Cmd.none
    | PatchServerResponse.Update p ->
        let newPatches =
            model.patches
            |> Array.map (fun existing -> if existing.id = p.id then p else existing)
        {model with patches = newPatches}, updateEditorState newPatches model.selected
    | PatchServerResponse.Remove id ->
        let newPatches = model.patches |> Array.filter (fun p -> p.id <> id)
        {model with patches = newPatches}, updateEditorState newPatches model.selected
    | PatchServerResponse.Kinds kinds ->
        model, kinds |> NewPatch.UpdateKinds |> Create |> Cmd.ofMsg
    | PatchServerResponse.UpdateUniverse newUniv ->
        let mutable found = false
        let universes =
            model.universes
            |> Array.map (fun u ->
                if u.universe = newUniv.universe then
                    found <- true
                    newUniv
                else u)
        let universes =
            if not found then Array.append universes [|newUniv|] else universes
        {model with universes = universes}, Cmd.none
    | PatchServerResponse.UniverseRemoved id ->
        {model with universes = model.universes |> Array.filter (fun u -> u.universe <> id)}, Cmd.none
    | PatchServerResponse.AvailablePorts ports ->
        {model with availablePorts = ports}, Cmd.none


let update message model =
    match message with
    | Response r -> updateFromServerMessage r model
    | SetSelected id ->
        {model with selected = Some id}, updateEditorState model.patches (Some(id))
    | Deselect -> {model with selected = None}, updateEditorState model.patches None
    | Edit m ->
        let editorModel, editorCmds = PatchEdit.update m model.editorModel
        {model with editorModel = editorModel}, editorCmds |> Cmd.map Edit
    | Create m ->
        let newPatchModel, newPatchCmds = NewPatch.update m model.newPatchModel
        {model with newPatchModel = newPatchModel}, newPatchCmds |> Cmd.map Create

/// Render a patch item as a basic table row.
let viewPatchTableRow dispatch selectedId item =
    let td x = R.td [] [R.str (x.ToString())]
    let universe, address =
        match item.address with
        | Some(u, a) -> string u, string a
        | None -> "", ""
    let rowAttrs: IHTMLProp list =
        let onClick = OnClick (fun _ -> SetSelected item.id |> dispatch)
        if Some(item.id) = selectedId
        then [onClick; Table.Row.Active]
        else [onClick]
    R.tr rowAttrs [
        td item.id;
        td item.name;
        td item.kind;
        td universe;
        td address;
        td item.channelCount;
    ]

let patchTableHeader =
    ["id"; "name"; "kind"; "universe"; "address"; "channel count"]
    |> List.map (fun x -> R.th [] [R.str x])
    |> R.tr []

let viewPatchTable dispatch patches selectedId =
    R.table [Table.Condensed] [
        R.tbody [] [
            yield patchTableHeader
            for patch in patches -> viewPatchTableRow dispatch selectedId patch
        ]
    ]

/// View the patcher page.
let view openModal model dispatch dispatchServer =
    Grid.layout [
        (8, [ viewPatchTable dispatch model.patches model.selected ])
        (4, [
            Grid.fullRow [
                PatchEdit.view model.editorModel (Edit >> dispatch) dispatchServer openModal]
            Grid.fullRow [
                NewPatch.view model.newPatchModel (Create >> dispatch) dispatchServer]
        ])
    ]

