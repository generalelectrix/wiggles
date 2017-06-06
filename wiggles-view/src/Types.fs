module Types

type UniverseId = int
type DmxAddress = int

type GlobalAddress = UniverseId * DmxAddress

type FixtureId = int

type PatchItem = {
    id: FixtureId;
    name: string;
    kind: string;
    address: GlobalAddress option;
    channelCount: int}
    with
    member this.universe = this.address |> Option.map fst
    member this.dmxAddress = this.address |> Option.map snd
    

/// All possible requests we can make to the patch server.
type ServerRequest =
    /// Request the full state of the patch to be sent.
    | PatchState
    /// Create a new patch; may fail due to address conflict.
    | NewPatch of PatchItem
    /// Rename a patch item by id.
    | Rename of FixtureId * string
    /// Repatch a fixture to a new universe/address, possibly unpatching.
    | Repatch of FixtureId * GlobalAddress option
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