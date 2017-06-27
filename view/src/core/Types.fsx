/// Core types used by wiggles view applications.
module Types
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Modal.fsx"
#load "Navbar.fsx"
#load "Socket.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props

/// Message metadata indicating the intended response filter for the result of processing this
/// message.
type ResponseFilter =
    /// Response to this message is of general interest to all clients.
    | All
    /// Response is only of interest to this client.
    | Exclusive
    /// Disable talkback on this request.
    | AllButSelf

let private withFilter filter message = (filter, message)

/// Attach a "All" response filter to a message.
let all message = withFilter All message
/// Attach a "Exclusive" response filter to a message.
let exclusive message = withFilter Exclusive message
/// Attach a "AllButSelf" response filter to a message.
let allButSelf message = withFilter AllButSelf message

type ConnectionState =
    | Waiting
    | Open
    | Closed

type SavesAvailable = {saves: string list; autosaves: string list}

/// Specification for which saved state of a show to load.
type LoadSpec =
    /// Load the latest saved state.
    | Latest
    /// Load this particular saved state.  Should consist only of a timestamp with no extension.
    | Exact of string
    /// Load the latest autosave.
    | LatestAutosave
    /// Load this particular autosave.  Should consist only of a timestamp with no extension.
    | ExactAutosave of string

type LoadShow = {
    name: string
    spec: LoadSpec
}

[<RequireQualifiedAccess>]
/// Outer wrapper for console command. Generic over the top-level command type used by an
/// implementation.
type ServerCommand<'m> =
    /// Get the name of the running show.
    | ShowName
    /// Create a new, empty show.
    | NewShow of string
    /// List all available shows.
    | SavedShows
    /// List all available saves and autosaves.
    | AvailableSaves
    /// Load a show using this spec.
    | Load of LoadShow
    /// Save the current state of the show.
    | Save
    /// Save the current show as a new show with a different name.  This show will become the one
    /// running in the reactor.
    | SaveAs of string
    /// Change the name of the currently-running show.  This will move all of the files in the
    /// saved show library.
    | Rename of string
    /// Quit the console, cleanly closing down every running thread.
    | Quit
    /// A message to be passed into the console logic running in the reactor.
    | Console of 'm

[<RequireQualifiedAccess>]
/// Outer wrapper for console response.  Generic over the response type used by an implementation.
type ServerResponse<'msg> =
    /// The name of the running show.
    | ShowName of string
    /// A listing of all available save and autosave files for the running show.
    | SavesAvailable of SavesAvailable
    /// Listing of all saved shows for this console.
    | ShowsAvailable of string list
    /// A new show was loaded, with this name.
    | Loaded of string
    /// The running show's name changed.
    | Renamed of string
    /// The show was saved successfully.
    | Saved
    /// A show library error occurred.
    | ShowLibErr of string
    /// The console is going to quit.
    | Quit
    /// A response emanting from the console itself.
    | Console of 'msg

