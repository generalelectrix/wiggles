/// Command and response types for the clock network.
module ClockTypes
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#load "Util.fsx"
#load "DataflowTypes.fsx"

open Elmish
open Util
open DataflowTypes

type ClockId = ClockId of NodeIndex * GenerationId

type SetInput = {
    clock: ClockId
    input: InputId
    target: Option<ClockId>
}

type CreateClock = {
    kind: string
    name: string
}

type RemoveClock = {
    id: ClockId
    force: bool
}

type Command =
    /// Get a listing of every available type of clock.
    | Classes
    /// Get a summary of the state of every clock.  Used to initialize new clients.
    | State
    /// Create a new clock.
    | Create of CreateClock
    /// Delete an existing clock.
    | Remove of RemoveClock
    /// Rename a clock.
    | Rename of ClockId * string
    /// Assign the input of a clock.
    | SetInput of SetInput
    /// Add a new input to a clock.
    | PushInput of ClockId
    /// Remove an input from a clock.
    | PopInput of ClockId

type ClockDescription = {
    name: string
    kind: string
    inputs: ClockId option list
}

type Response =
    /// A listing of every available type of clock.
    | Classes of string list
    /// A summary of the state of every clock.
    | State of (ClockId * ClockDescription) list
    /// A new clock has been added.
    | New of ClockId * ClockDescription
    /// A clock has been deleted.
    | Removed of ClockId
    /// A clock has been renamed.
    | Renamed of ClockId * string
    /// A clock's input has been reassigned.
    | SetInput of SetInput
    /// A clock has had a new input added.
    | PushInput of ClockId
    /// A clock has had an input removed.
    | PopInput of ClockId

