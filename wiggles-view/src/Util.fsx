/// Helper functions of general use.
module Util
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"

open System
open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props

/// Stringify a option with an empty string for None.
let emptyIfNone opt =
    match opt with
    | Some(s) -> string s
    | None -> ""

/// Try to parse a string as an integer.  Return None if it cannot be parsed.
/// This uses Javascript's amazing number parsing that will happily parse "32foo" as 32.
let parseInt s =
    let parsed = int s
    if parsed |> float |> System.Double.IsNaN then None
    else Some parsed

/// Concatenate two Fable KeyValueLists.
[<Emit("Object.assign({}, $0, $1)")>]
let ( ++ ) (a:'a list) (b:'a list) : 'a list = jsNative