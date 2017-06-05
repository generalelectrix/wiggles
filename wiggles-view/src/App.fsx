#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"

open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React

let initialModel () = "Hello, Elmish!"

let update model =
    model

let view model dispatch =
    R.div
        []
        [
            (R.a [] [unbox model]);
            (R.p [] [unbox model]);
        ]


Program.mkSimple initialModel update view
|> Program.withReact "app"
|> Program.run