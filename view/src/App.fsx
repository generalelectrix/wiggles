#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Types.fsx"
#load "Bootstrap.fsx"
#load "Modal.fsx"
#load "PatchEdit.fsx"
#load "NewPatch.fsx"
#load "Socket.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Types
open Bootstrap
open Socket

// If true, use a mock server rather than a real websocket.
let mock = true

type ConnectionState =
    | Waiting
    | Open
    | Closed

type Model = {
    connection: ConnectionState
    patches: PatchItem array
    // Current fixture ID we have selected, if any.
    selected: FixtureId option
    // Model for the patch editor.
    editorModel: PatchEdit.Model
    newPatchModel: NewPatch.Model
    // Pop-over modal dialog.
    modalDialog: Modal.Model
    // For now show errors in a console of infinite length.
    consoleText: string array
}

let withConsoleMessage msg model =
    {model with consoleText = Array.append model.consoleText [|msg|]}

type UiAction =
    | ClearConsole
    | SetSelected of FixtureId
    | Deselect

type Message =
    | Socket of SocketMessage
    | Request of ServerRequest
    | Response of ServerResponse
    | Action of UiAction
    | Edit of PatchEdit.Message
    | Create of NewPatch.Message
    | ModalDialog of Modal.Message

// Launch the websocket we'll use to talk to the server.
let (subscription, send) = openSocket Socket

// We don't emit these along with the initial model as we need to wait for the socket to connect
// to the server before sending messages.
let initCommands =
    [ServerRequest.PatchState; ServerRequest.GetKinds]
    |> List.map Cmd.ofMsg
    |> Cmd.batch
    |> Cmd.map Request

let initialModel () =
    let m = {
        connection = Waiting
        patches = Array.empty
        selected = None
        editorModel = PatchEdit.initialModel()
        newPatchModel = NewPatch.initialModel()
        modalDialog = Modal.initialModel()
        consoleText = Array.empty
    }
    (m, Cmd.none)

let mutable counter = testPatches.Length

let mockMakePatch (patchReq: PatchRequest) =
    let id = counter
    counter <- counter + 1
    {id = id;
     name = patchReq.name;
     kind = patchReq.kind;
     channelCount = if patchReq.name = "dimmer" then 1 else 2;
     address = patchReq.address;}

/// A fake server to emit messages as if we were talking to a real server.
let mockServer model req =
    let maybeUpdatePatch msgType op patchId =
        model.patches
        |> Array.tryFind (fun p -> p.id = patchId)
        |> Option.map (op >> msgType)
        |> (function
            | Some p -> p
            | None -> ServerResponse.Error (sprintf "Unknown fixture id %d" patchId))

    match req with
    | ServerRequest.PatchState ->
        if model.patches |> Array.isEmpty then testPatches else model.patches
        |> ServerResponse.PatchState
    | ServerRequest.GetKinds -> ServerResponse.Kinds testKinds
    | ServerRequest.NewPatches patches ->
        patches |> Array.map mockMakePatch |> ServerResponse.NewPatches
    | ServerRequest.Rename (id, name) ->
        maybeUpdatePatch
            ServerResponse.Update
            (fun p -> {p with name = name})
            id
    | ServerRequest.Repatch (id, addr) ->
        maybeUpdatePatch
            ServerResponse.Update
            (fun p -> {p with address = addr})
            id
    | ServerRequest.Remove id ->
        maybeUpdatePatch
            ServerResponse.Remove
            (fun _ -> id)
            id

/// Return a command to update the editor's state if fixture id is among those in patches.
let updateEditorState patches selectedFixtureId =
    selectedFixtureId
    |> Option.map (fun fixtureId ->
        patches |> Array.tryFind (fun p -> p.id = fixtureId))
    |> function | None -> None | Some x -> x // Option.flatten not supported by Fable, apparently.
    |> PatchEdit.SetState
    |> Edit
    |> Cmd.ofMsg

let update message model =

    match message with
    | Socket Connected ->
        // The socket connected, issue the initial commands.
        ({model with connection = Open}, initCommands)
    | Socket Disconnected ->
        // The server closed the connection, hopefully not because it crashed.
        ({model with connection = Closed}, Cmd.none)
    | Request r ->
        // Dispatch a request to the websocket.
        if mock then
            (model, mockServer model r |> Response |> Cmd.ofMsg)
        else
            send r
            (model, Cmd.none)
    | Response r ->
        match r with
        | ServerResponse.Error msg ->
            model |> withConsoleMessage msg, Cmd.none
        | ServerResponse.PatchState s ->
            {model with patches = s}, updateEditorState s model.selected
        | ServerResponse.NewPatches patches ->
            {model with patches = patches |> Array.append model.patches}, Cmd.none
        | ServerResponse.Update p ->
            let newPatches =
                model.patches
                |> Array.map (fun existing -> if existing.id = p.id then p else existing)
            {model with patches = newPatches}, updateEditorState newPatches model.selected
        | ServerResponse.Remove id ->
            let newPatches = model.patches |> Array.filter (fun p -> p.id <> id)
            {model with patches = newPatches}, updateEditorState newPatches model.selected
        | ServerResponse.Kinds kinds ->
            model, kinds |> NewPatch.UpdateKinds |> Create |> Cmd.ofMsg
    | Action a ->
        match a with
        | ClearConsole -> {model with consoleText = Array.empty}, Cmd.none
        | SetSelected id ->
            {model with selected = Some id}, updateEditorState model.patches (Some(id))
        | Deselect -> {model with selected = None}, updateEditorState model.patches None
    | Edit m ->
        let editorModel, editorCmds = PatchEdit.update m model.editorModel
        {model with editorModel = editorModel}, editorCmds |> Cmd.map Edit
    | Create m ->
        let newPatchModel, newPatchCmds = NewPatch.update m model.newPatchModel
        {model with newPatchModel = newPatchModel}, newPatchCmds |> Cmd.map Create
    | ModalDialog m ->
        {model with modalDialog = Modal.update m model.modalDialog}, Cmd.none
    

let updateAndLog message model =
    let model, cmds = update message model
    (model |> withConsoleMessage (sprintf "%+A" message), cmds)

/// Render a patch item as a basic table row.
let viewPatchTableRow dispatch selectedId item =
    let td x = R.td [] [R.str (x.ToString())]
    let universe, address =
        match item.address with
        | Some(u, a) -> string u, string a
        | None -> "", ""
    let rowAttrs: IHTMLProp list =
        let onClick = OnClick (fun _ -> SetSelected item.id |> Action |> dispatch)
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


let viewConsole dispatch lines =
    R.div [Form.Group] [
        R.span [] [
            R.str "Console";
            R.button [
                Button.Warning
                OnClick (fun _ -> ClearConsole |> Action |> dispatch)
            ] [ R.str "clear" ];
        ];
        R.div [] [
            R.textarea [
                Form.Control
                Style [Overflow "scroll"];
                Value (String.concat "\n" lines |> Case1);
                Rows 20.
                Cols 80.
            ] [];
        ]
    ]


let view model dispatch =
    let dispatchServer = Request >> dispatch

    /// Helper function passed to views that want to be able to open modal dialogs to confirm actions.
    let openModal req = req |> Modal.Open |> ModalDialog |> dispatch

    R.div [Container.Fluid] [
        Grid.layout [
            (8, [ viewPatchTable dispatch model.patches model.selected ])
            (4, [
                Grid.fullRow [
                    PatchEdit.view model.editorModel (Edit >> dispatch) dispatchServer openModal]
                Grid.fullRow [
                    NewPatch.view model.newPatchModel (Create >> dispatch) dispatchServer]
            ])
        ]
        Grid.fullRow [
            viewConsole dispatch model.consoleText
        ]
        Modal.view model.modalDialog (ModalDialog >> dispatch)
    ]

/// Outer view wrapper to show splashes if we're waiting on a server connection or if the connection
/// has gone away.
let viewWithConnection model dispatch =
    match model.connection with
    | Waiting -> Modal.viewSplash "Waiting for console server connection to be established."
    | Open -> view model dispatch
    | Closed -> Modal.viewSplash "The console server disconnected."

Program.mkProgram initialModel updateAndLog viewWithConnection
|> Program.withSubscription subscription
|> Program.withReact "app"
|> Program.withConsoleTrace
|> Program.run