/// A websocket subscription that asynchronously sends and receives messages.
module Socket
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"

open System
open Fable.Core
open Fable.Import
open Elmish
open Fable.Core.JsInterop

/// Messages to convey state change in the socket communicating with the console server.
/// The socket connection should not be used to send data until the connected message has been
/// emitted.
type SocketMessage =
    | Connected
    | Disconnected

/// Print an exception to the console with extra leading text.
let private logException msg (e: System.Exception) = Browser.console.error(msg, e)

[<PassGenerics>]
/// Open a websocket connection to the current host on the same port used to serve this application.
/// Returns a subscription that will produce a stream of messages received, as well as a function
/// that will send a message on the socket.
/// Pass a function that lifts a socket message into the overall message type for this application.
let openSocket wrapSocketMessage =

    // let host = sprintf "ws://%s/" Browser.window.location.host
    let host = "ws://127.0.0.1:2794"

    let ws: Browser.WebSocket = Browser.WebSocket.Create(host, Case1("wiggles"))

    /// Return a subscription.  Curry on the function that converts whatever we expect to receive
    /// on the wire into the top-level message type for the application.
    let subscription messageWrapper _ =
        /// This function will be called during application init and passed the dispatch function,
        /// which is attached to the socket and used on receipt of a message to pass that message
        /// into the message queue.
        Cmd.ofSub (fun (dispatch: 'msg -> unit) ->
            ws.addEventListener_message(
                fun (event: Browser.MessageEvent) ->
                    try
                        unbox event.data
                        |> ofJson
                        |> messageWrapper
                        |> dispatch
                    with e ->
                        logException "Message deserialization error:" e
                    null
            )
            ws.addEventListener_open(fun _ ->
                Connected |> wrapSocketMessage |> dispatch
                null
            )
            ws.addEventListener_close(fun _ ->
                Disconnected |> wrapSocketMessage |> dispatch
                null
            )
        )
       
    /// Send a message using the socket, catching an exception and printing it to the console.
    let send msg =
        let jsonMessage = msg |> toJson
        try
            ws.send jsonMessage
        with e ->
            logException (sprintf "Websocket error while sending message:\n%s\n\n" jsonMessage) e

    (subscription, send)