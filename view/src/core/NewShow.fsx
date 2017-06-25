/// View for creating a new show.
module NewShow
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "Util.fsx"
#load "Bootstrap.fsx"
#load "Types.fsx"
#load "SimpleEditor.fsx"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props
open Util
open Bootstrap
open Types
          
type Model = SimpleEditor.Model<string>
let initModel() = SimpleEditor.initModel "New" "Name for new show:" errorIfEmpty InputType.Text

type Message = SimpleEditor.Message<string>

let update = SimpleEditor.update

let view model onComplete dispatch dispatchServer = 
    let onOk name = 
        let command = ServerCommand.NewShow(name)
        (All, command) |> dispatchServer

    SimpleEditor.view "" model onOk onComplete dispatch
 
    