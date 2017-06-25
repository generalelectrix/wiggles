import { view as view_1, update as update_1, initModel as initModel_1 } from "./SimpleEditor";
import { errorIfEmpty } from "./Util";
import { InputType } from "./Bootstrap";
import { ResponseFilter, ServerCommand } from "./Types";
export function initModel() {
  return initModel_1("Save", "Save show as...", errorIfEmpty, InputType.Text);
}
export function update() {
  return function (message) {
    return function (model) {
      return update_1(message, model);
    };
  };
}
export function view(showName, model, onComplete, dispatch, dispatchServer) {
  var onOk = function onOk(newName) {
    var command = new ServerCommand("SaveAs", [newName]);
    dispatchServer([new ResponseFilter("All", []), command]);
  };

  return view_1(showName, model, onOk, onComplete, dispatch);
}