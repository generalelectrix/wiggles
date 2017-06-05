#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "../node_modules/fable-react-toolbox/Fable.Helpers.ReactToolbox.fs"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
module RT = Fable.Helpers.ReactToolbox

let text x : (Fable.Import.React.ReactElement) = unbox x

type UniverseId = int

type DmxAddress = UniverseId * int

type FixtureId = int

type PatchItem = {
    id: FixtureId;
    name: string;
    address: DmxAddress option;
    channelCount: int;
}

type Model = {
    patches: PatchItem list;
    // Current fixture ID we have selected, if any.
    selected: FixtureId option;
    // For now show errors in a console of infinite length.
    consoleText: string array}

let withConsoleMessage msg model =
    {model with consoleText = Array.append model.consoleText [|msg|]}

/// All possible requests we can make to the patch server.
type ServerRequest =
    /// Request the full state of the patch to be sent.
    | PatchState
    /// Create a new patch; may fail due to address conflict.
    | NewPatch of PatchItem
    /// Rename a patch item by id.
    | Rename of FixtureId * string
    /// Repatch a fixture to a new universe/address, possibly unpatching.
    | Repatch of FixtureId * DmxAddress option
    /// Remove a fixture from the patch entirely.
    | Remove of FixtureId

/// All possible responses we can receive from the patch server.
type ServerResponse =
    /// Generic error message from the server, we may log or display to user.
    | Error of string
    /// Full current state of the patch.
    | PatchState of PatchItem list
    /// Single new patch added.
    | NewPatch of PatchItem
    /// A patch has been updated, update our version if we have it.
    | Update of PatchItem
    /// A patch item has been removed.
    | Remove of FixtureId

type UiAction =
    | ClearConsole
    | SetSelected of FixtureId
    | Deselect

type Message =
    | Request of ServerRequest
    | Response of ServerResponse
    | Action of UiAction

let testPatches = [
    {id = 0; name = "foo"; address = None; channelCount = 2}
    {id = 1; name = "charlie"; address = Some(0, 27); channelCount = 1}
]


let initialModel () =
    ({patches = testPatches; selected = None; consoleText = Array.empty}, Cmd.none)

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

let update message model =

    match message with
    | Request r ->
        (model, mockServer model r |> Response |> Cmd.ofMsg)
    | Response r ->
        let newModel =
            match r with
            | Error msg -> model |> withConsoleMessage msg
            | PatchState s -> {model with patches = s}
            | NewPatch p -> {model with patches = p::model.patches}
            | Update p ->
                let newPatches =
                    model.patches
                    |> List.map (fun existing -> if existing.id = p.id then p else existing)
                {model with patches = newPatches}
            | Remove id ->
                {model with patches = model.patches |> List.filter (fun p -> p.id = id)}
        (newModel, Cmd.none)
    | Action a ->
        a |>
        function
            | ClearConsole -> {model with consoleText = Array.empty}
            | SetSelected id -> {model with selected = Some id}
            | Deselect -> {model with selected = None}
        |> fun m -> (m, Cmd.none)

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
        td item.name;
        td universe;
        td address;
        td item.channelCount;
    ]

let patchTableHeader =
    ["selected"; "id"; "name"; "universe"; "address"; "channel count"]
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
        viewConsole dispatch model.consoleText
    ]

Program.mkProgram initialModel updateAndLog view
|> Program.withReact "app"
|> Program.run