#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "../node_modules/fable-react-toolbox/Fable.Helpers.ReactToolbox.fs"
#load "Types.fs"
#load "PatchEdit.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
module RT = Fable.Helpers.ReactToolbox
open Types

let text x : (Fable.Import.React.ReactElement) = unbox x


type Model = {
    patches: PatchItem list;
    // Current fixture ID we have selected, if any.
    selected: FixtureId option;
    // Model for the patch editor.
    editorModel: PatchEdit.Model;
    // For now show errors in a console of infinite length.
    consoleText: string array}

let withConsoleMessage msg model =
    {model with consoleText = Array.append model.consoleText [|msg|]}

type UiAction =
    | ClearConsole
    | SetSelected of FixtureId
    | Deselect

type Message =
    | Request of ServerRequest
    | Response of ServerResponse
    | Action of UiAction
    | Edit of PatchEdit.Message

let testPatches = [
    {id = 0; name = "foo"; kind = "dimmer"; address = None; channelCount = 2}
    {id = 1; name = "charlie"; kind = "roto"; address = Some(0, 27); channelCount = 1}
]


let initialModel () =
    let m = 
       {patches = testPatches;
        selected = None;
        editorModel = PatchEdit.initialModel();
        consoleText = Array.empty}
    (m, Cmd.none)

/// A fake server to emit messages as if we were talking to a real server.
let mockServer model req =
    let maybeUpdatePatch msgType op patchId =
        model.patches
        |> List.tryFind (fun p -> p.id = patchId)
        |> Option.map (op >> msgType)
        |> (function
            | Some p -> p
            | None -> Error (sprintf "Unknown fixture id %d" patchId))

    match req with
    | ServerRequest.PatchState -> PatchState model.patches
    | ServerRequest.NewPatch p -> NewPatch p
    | Rename (id, name) ->
        maybeUpdatePatch
            Update
            (fun p -> {p with name = name})
            id
    | Repatch (id, addr) ->
        maybeUpdatePatch
            Update
            (fun p -> {p with address = addr})
            id
    | ServerRequest.Remove id ->
        maybeUpdatePatch
            Remove
            (fun _ -> id)
            id

let purple = Color "#6600ff"
let cyan = Color "#00ccff"

/// Return a command to update the editor's state if fixture id is among those in patches.
let updateEditorState patches selectedFixtureId =
    selectedFixtureId
    |> Option.map (fun fixtureId ->
        patches |> List.tryFind (fun p -> p.id = fixtureId))
    |> function | None -> None | Some x -> x // Option.flatten not supported by Fable, apparently.
    |> PatchEdit.SetState
    |> Edit
    |> Cmd.ofMsg

let update message model =

    match message with
    | Request r ->
        (model, mockServer model r |> Response |> Cmd.ofMsg)
    | Response r ->
        match r with
        | Error msg -> model |> withConsoleMessage msg, Cmd.none
        | PatchState s -> {model with patches = s}, updateEditorState s model.selected
        | NewPatch p -> {model with patches = p::model.patches}, Cmd.none
        | Update p ->
            let newPatches =
                model.patches
                |> List.map (fun existing -> if existing.id = p.id then p else existing)
            {model with patches = newPatches}, updateEditorState newPatches model.selected
        | Remove id ->
            let newPatches = model.patches |> List.filter (fun p -> p.id = id)
            {model with patches = newPatches}, updateEditorState newPatches model.selected
    | Action a ->
        match a with
        | ClearConsole -> {model with consoleText = Array.empty}, Cmd.none
        | SetSelected id ->
            {model with selected = Some id}, updateEditorState model.patches (Some(id))
        | Deselect -> {model with selected = None}, updateEditorState model.patches None
    | Edit e ->
        let editorModel, editorCmds = PatchEdit.update e model.editorModel
        {model with editorModel = editorModel}, editorCmds |> Cmd.map Edit
    

let updateAndLog message model =
    let model, cmds = update message model
    (model |> withConsoleMessage (sprintf "%+A" message), cmds)

/// Render a patch item as a basic table row.
let viewPatchTableRow dispatch selectedId item =
    let td x = R.td [] [text x]
    let universe, address =
        match item.address with
        | Some(u, a) -> string u, string a
        | None -> "", ""
    R.tr [] [
        R.td [] [
            R.button [
                OnClick (fun _ -> SetSelected item.id |> Action |> dispatch);
                Style [(if Some item.id = selectedId then purple else cyan)]
            ] []
        ];
        td item.id;
        td item.kind;
        td item.name;
        td universe;
        td address;
        td item.channelCount;
    ]

let patchTableHeader =
    ["selected"; "id"; "kind"; "name"; "universe"; "address"; "channel count"]
    |> List.map (fun x -> R.th [] [text x])
    |> R.tr []

let viewPatchTable dispatch patches selectedId =
    R.table [] [
        R.tbody [] [
            yield patchTableHeader
            for patch in patches -> viewPatchTableRow dispatch selectedId patch
        ]
    ]

let viewConsole dispatch lines =
    R.div [] [
        R.span [] [
            text "Console";
            R.button [ OnClick (fun _ -> ClearConsole |> Action |> dispatch) ] [ text "clear" ];
        ];
        R.div [] [
            R.textarea [
                Style [Overflow "scroll"];
                Value (String.concat "\n" lines |> Case1);
            ] [];

        ]
    ]

let view model dispatch =
    R.div [] [
        viewPatchTable dispatch model.patches model.selected
        PatchEdit.view model.editorModel (Edit >> dispatch) (Request >> dispatch)
        viewConsole dispatch model.consoleText
    ]

Program.mkProgram initialModel updateAndLog view
|> Program.withReact "app"
|> Program.run