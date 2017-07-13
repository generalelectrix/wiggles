/// Common types for the dataflow networks.
module DataflowTypes
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#load "../core/Util.fsx"

open Elmish
open Util

type NodeIndex = int
type GenerationId = int
type InputId = int
type OutputId = int

type ClockId = ClockId of NodeIndex * GenerationId
type WiggleId = WiggleId of NodeIndex * GenerationId

type KnobAddr = int

type ClockKnobAddr = ClockId * KnobAddr
type WiggleKnobAddr = WiggleId * KnobAddr

/// Top-level knob address type.
type KnobAddress =
    | Clock of ClockKnobAddr
    | Wiggle of WiggleKnobAddr