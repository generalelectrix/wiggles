/// Common types for the dataflow networks.
module DataflowTypes
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#load "../core/Util.fsx"

open Elmish
open Util

type NodeIndex = uint32
type GenerationId = uint32
type InputId = uint32
type OutputId = uint32