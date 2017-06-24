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

// Key codes
/// Key code for 'enter'
let [<Literal>] EnterKey = 13.0
/// Key code for 'escape'
let [<Literal>] EscapeKey = 27.0

module Result =
    /// Map Some(v) -> Ok(v), None -> Error()
    let ofOption o =
        match o with
        | Some(x) -> Ok(x)
        | None -> Error()

/// Stringify a option with an empty string for None.
let emptyIfNone opt =
    match opt with
    | Some(s) -> s.ToString()
    | None -> ""

/// Lift a string into Option unless it is the empty string or whitespace.
let noneIfEmpty (s: string) = if s.Trim() = "" then None else Some(s)

/// Lift a string into Result unless it is the empty string or whitespace.
let errorIfEmpty = noneIfEmpty >> Result.ofOption

/// Try to parse a string as an integer.  Return None if it cannot be parsed.
/// This uses Javascript's amazing number parsing that will happily parse "32foo" as 32.
let parseInt (s: string) =
    let parsed = int s
    if parsed |> float |> System.Double.IsNaN then None
    else Some parsed

/// Try to parse a string as a float.  Return None if it cannot be parsed.
/// This uses Javascript's amazing number parsing that will happily parse "32foo" as 32.
let parseFloat (s: string) =
    let parsed = float s
    if parsed |> System.Double.IsNaN then None
    else Some parsed

/// Roll a basic optional type to avoid the issues with doubly-nested Option represented as a
/// nullable value.  Use this with edit boxes if your 'T will itself be an option.
type Optional<'T> =
    | Present of 'T
    | Absent
    with
    override this.ToString() =
        match this with
        | Present(x) -> x.ToString()
        | Absent -> ""

module Optional =
    let ofOption opt =
        match opt with
        | Some(x) -> Present(x)
        | None -> Absent

/// Parse an optional integer.  Validator is applied to a successfully parsed number.
let parseOptionalNumber validator v =
    match noneIfEmpty v with
    | None -> Ok(Absent)
    | Some(v) ->
        v
        |> parseInt
        |> Result.ofOption
        |> Result.bind validator
        |> Result.map Present

/// Concatenate two Fable KeyValueLists.
[<Emit("Object.assign({}, $0, $1)")>]
let ( ++ ) (a:'a list) (b:'a list) : 'a list = jsNative


/// Push a new event action onto the end of the browser processing queue.
/// Useful for running actions that need to wait until the DOM is done rendering, such as
/// non-declarative DOM aspects like setting or removing focus upon drawing.
/// Uses setTimeout of 0 to enqueue.
let enqueueBrowserAction action =
    Browser.window.setTimeout(
        (fun _ -> action()),
        0
    ) |> ignore


/// Print an exception to the console with extra leading text.
let logException msg (e: System.Exception) = Browser.console.error(msg, e)

/// Print an error message to the console.
let logError msg = Browser.console.error(msg) |> ignore