module Types
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#load "Util.fsx"

open Elmish
open Util

type FixtureKind = {
    name: string;
    channelCount: int}

type UniverseId = int
type DmxAddress = int

/// Return Ok(u) if this is a valid universe ID (>= 0).
let validUniverse u = if u >= 0 then Ok(u) else Error()

/// Return Ok(a) if this is a valid DMX address.
let validDmxAddress a = if a > 0 && a < 513 then Ok(a) else Error()

type GlobalAddress = UniverseId * DmxAddress

let globalAddressFromOptions univOpt addrOpt =
    match univOpt, addrOpt with
    | Some(u), Some(a) -> Some(u, a) |> Ok
    | None, None -> None |> Ok
    | _ -> Error()

type FixtureId = int

type PatchRequest = {
    name: string;
    kind: string;
    address: GlobalAddress option;
}

type PatchItem = {
    id: FixtureId;
    name: string;
    kind: string;
    address: GlobalAddress option;
    channelCount: int}
    with
    member this.universe = this.address |> Option.map fst
    member this.dmxAddress = this.address |> Option.map snd

let testPatches = [
    {id = 0; name = "foo"; kind = "dimmer"; address = None; channelCount = 2}
    {id = 1; name = "charlie"; kind = "roto"; address = Some(0, 27); channelCount = 1}
]

let testKinds : FixtureKind list = [
    {name = "dimmer"; channelCount = 1}
    {name = "roto"; channelCount = 2}
]
    

/// All possible requests we can make to the patch server.
[<RequireQualifiedAccess>]
type ServerRequest =
    /// Request the full state of the patch to be sent.
    | PatchState
    /// Create one or more new patches; may fail.
    | NewPatches of PatchRequest list
    /// Rename a patch item by id.
    | Rename of FixtureId * string
    /// Repatch a fixture to a new universe/address, possibly unpatching.
    | Repatch of FixtureId * GlobalAddress option
    /// Remove a fixture from the patch entirely.
    | Remove of FixtureId
    /// Retrieve a listing of every available fixture kind.
    | GetKinds

/// All possible responses we can receive from the patch server.
[<RequireQualifiedAccess>]
type ServerResponse =
    /// Generic error message from the server, we may log or display to user.
    | Error of string
    /// Full current state of the patch.
    | PatchState of PatchItem list
    /// One or more new patches added.
    | NewPatches of PatchItem list
    /// A patch has been updated, update our version if we have it.
    | Update of PatchItem
    /// A patch item has been removed.
    | Remove of FixtureId
    /// A listing of every available fixture kind.
    | Kinds of FixtureKind list