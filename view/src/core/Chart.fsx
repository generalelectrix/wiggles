/// Basic charting for Wiggles.
#r "../node_modules/fable-core/Fable.Core.dll"
#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish/Fable.Elmish.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"
#load "../../node_modules/fable-import-d3/Fable.Import.D3.fs"

open Fable.Core
open Fable.Import
open Elmish
open Elmish.React
open Fable.Core.JsInterop
module R = Fable.Helpers.React
open Fable.Helpers.React.Props

type Datatype =
    | Unipolar
    | Bipolar
    | UInt of int

// charting for visualizing Wiggles.
// Since periodicity is core in Wiggles, we expect the domain of every dataset to be a phase on
// the range [0.0, 1.0].
// Futher, this chart is specialized to display a "waveform repertoire" (a fixed number of samples
// spanning the entire phase range) plus a current value moving on top of it.

type Dataset = (float * float) array

let yScale datatype =
    let domain =
        match datatype with
        | Unipolar -> [|0.0; 1.0|]
        | Bipolar -> [|-1.0; 1.0|]
        | UInt(i) -> [|0.0; float (i-1)|]
    D3.Scale.Globals.linear().domain(domain).range([|1.0; 0.0|])

let xScale = D3.Scale.Globals.linear().domain([|0.0; 1.0|]).range([|0.0; 1.0|])

let view width (dataset: Dataset) (datatype: Datatype) (currentValue: float * float) dispatch =

    let yScale = yScale datatype

    let getX (x, _) _ = xScale.Invoke(x)
    let getY (_, y) _ = yScale.Invoke(y)

    let line = D3.Svg.Globals.line().x(getX).y(getY).Invoke(dataset)

    let (markerX, markerY) = currentValue

    R.svg [
        ViewBox "0 0 1 1"
        unbox ("width", size)
    ] [
        R.path [
            D line
            Stroke "white"
            StrokeWidth (Case1 0.02)
            Fill "none"
        ] []
        R.circle [
            Cx (Case1 xScale.Invoke(markerX))
            Cy (Case1 xScale.Invoke(markerY))
            R (Case1 0.025)
            Fill "magenta"
        ] []
    ]
