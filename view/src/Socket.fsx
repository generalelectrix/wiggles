/// A websocket subscription that asynchronously sends and receives messages.
module Socket
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#load "Util.fsx"
#load "Types.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Fable.Core.JsInterop
open Util
open Types
open Bootstrap
