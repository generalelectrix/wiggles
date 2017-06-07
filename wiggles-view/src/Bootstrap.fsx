/// React CSS props for various Bootstrap CSS classes.
module Bootstrap

#r "../node_modules/fable-react/Fable.React.dll"
#r "../node_modules/fable-elmish-react/Fable.Elmish.React.dll"

open Elmish.React
open Fable.Helpers.React.Props
module R = Fable.Helpers.React
open Fable.Helpers.React.Props

let private cn = ClassName

type ReactElement = Fable.Import.React.ReactElement

let private elementWithClass
        (elemFunc: IHTMLProp list -> ReactElement list -> ReactElement)
        cls
        (elems: ReactElement list) =
    elemFunc [cn cls] elems

module Container =
    let Fixed = cn "container"
    let Fluid = cn "container-fluid"

module Grid =
    let private divWithClass = elementWithClass R.div
    let row = divWithClass "row"
    let col num elems = divWithClass (sprintf "col-sm-%d" num) elems
    let fullRow elems =
        row [ col 12 elems ]

    let layout (elementsWithWidths: #seq<int * (ReactElement list)>) =
        row [for (width, elements) in elementsWithWidths -> col width elements]

    /// Attempt to evenly grid-distribute elements.
    /// Best results with 1, 2, 3, 4, 6, 12
    let distribute (elements: ReactElement list list) =
        let width = 12 / elements.Length
        elements
        |> List.map (fun elements -> width, elements)
        |> layout


module Form =
    let Control = cn "form-control"
    let Group = cn "form-group"

module Table =
    let Basic = cn "table"
    let Striped = cn "table table-striped"
    let Bordered = cn "table table-bordered"
    let Hover = cn "table table-hover"
    let Condensed = cn "table table-condensed"
    let Responsive = cn "table table-responsive"

    module Row =
        let Active = cn "active"
        let Success = cn "success"
        let Info = cn "info"
        let Warning = cn "warning"
        let Danger = cn "danger"

module Button =
    let Basic = cn "btn"
    let Warning = cn "btn btn-warning"