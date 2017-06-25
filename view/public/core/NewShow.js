import { view as view_1, update as update_1, initModel as initModel_1 } from "./SimpleEditor";
import { errorIfEmpty } from "./Util";
import { InputType } from "./Bootstrap";
import { ResponseFilter, ServerCommand } from "./Types";
export function initModel() {
  return initModel_1("New", "Name for new show:", errorIfEmpty, InputType.Text);
}
export function update() {
  return function (message) {
    return function (model) {
      return update_1(message, model);
    };
  };
}
export function view(model, onComplete, dispatch, dispatchServer) {
  var onOk = function onOk(name) {
    var command = new ServerCommand("NewShow", [name]);
    dispatchServer([new ResponseFilter("All", []), command]);
  };

  return view_1("", model, onOk, onComplete, dispatch);
}