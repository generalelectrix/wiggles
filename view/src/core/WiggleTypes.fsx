/// Command and response types for the wiggle network.
module WiggleTypes
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#load "Util.fsx"
#load "DataflowTypes.fsx"

open Elmish
open Util
open DataflowTypes

type SetInput = {
    wiggle: WiggleId
    input: InputId
    target: (WiggleId * OutputId) option
}

type CreateWiggle = {
    kind: string
    name: string
}

type RemoveWiggle = {
    id: WiggleId
    force: bool
}

[<RequireQualifiedAccess>]
type Command =
    /// Get a listing of every available type of wiggle.
    | Kinds
    /// Get a summary of the state of every wiggle.  Used to initialize new clients.
    | State
    /// Create a new wiggle.
    | Create of CreateWiggle
    /// Delete an existing wiggle.
    | Remove of RemoveWiggle
    /// Rename a wiggle.
    | Rename of WiggleId * string
    /// Assign the input of a wiggle.
    | SetInput of SetInput
    /// Add a new input to a wiggle.
    | PushInput of WiggleId
    /// Remove an input from a wiggle.
    | PopInput of WiggleId
    /// Add a new output to a wiggle.
    | PushOutput of WiggleId
    /// Remove an output from a wiggle.
    | PopOutput of WiggleId
    /// Set the clock source for a wiggle.
    | SetClock of WiggleId * ClockId option

type UsesClock =
    | Yes of ClockId option
    | No

type WiggleDescription = {
    name: string
    kind: string
    inputs: (WiggleId * OutputId) option list
    outputs: int,
    clock: UsesClock
}

[<RequireQualifiedAccess>]
type Response =
    /// A listing of every available type of wiggle.
    | Classes of string list
    /// A summary of the state of every wiggle.
    | State of (WiggleId * WiggleDescription) list
    /// A new wiggle has been added.
    | New of WiggleId * WiggleDescription
    /// A wiggle has been deleted.
    | Removed of WiggleId
    /// A wiggle has been renamed.
    | Renamed of WiggleId * string
    /// A wiggle's input has been reassigned.
    | SetInput of SetInput
    /// A wiggle has had a new input added.
    | PushInput of WiggleId
    /// A wiggle has had an input removed.
    | PopInput of WiggleId
    /// A wiggle has had a new output added.
    | PushOutput of WiggleId
    /// A wiggle has had an output removed.
    | PopOutput of WiggleId
    /// A wiggle has had its clock source changed.
    | SetClock of WiggleId * ClockId option

