var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { Option, Any } from "fable-core/Util";
import { Button } from "./Bootstrap";
import { fold, singleton } from "fable-core/Seq";
import { createElement } from "react";
import { ofArray } from "fable-core/List";
export var ModalAction = function () {
  function ModalAction(label, buttonType, action) {
    _classCallCheck(this, ModalAction);

    this.label = label;
    this.buttonType = buttonType;
    this.action = action;
  }

  _createClass(ModalAction, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Modal.ModalAction",
        interfaces: ["FSharpRecord"],
        properties: {
          label: "string",
          buttonType: Any,
          action: "function"
        }
      };
    }
  }]);

  return ModalAction;
}();
setType("Modal.ModalAction", ModalAction);
export var ModalRequest = function () {
  function ModalRequest(message, action0, action1) {
    _classCallCheck(this, ModalRequest);

    this.message = message;
    this.action0 = action0;
    this.action1 = action1;
  }

  _createClass(ModalRequest, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Modal.ModalRequest",
        interfaces: ["FSharpRecord"],
        properties: {
          message: "string",
          action0: ModalAction,
          action1: Option(ModalAction)
        }
      };
    }
  }]);

  return ModalRequest;
}();
setType("Modal.ModalRequest", ModalRequest);
export var Message = function () {
  function Message(caseName, fields) {
    _classCallCheck(this, Message);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(Message, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Modal.Message",
        interfaces: ["FSharpUnion"],
        cases: {
          Close: [],
          Open: [ModalRequest]
        }
      };
    }
  }]);

  return Message;
}();
setType("Modal.Message", Message);
export function confirm(message, action) {
  var okAction = new ModalAction("OK", Button.Basic, action);
  var cancelAction = new ModalAction("Cancel", Button.Default, function (value) {
    value;
  });
  return new ModalRequest(message, okAction, cancelAction);
}
export function initialModel() {
  return new Array(0);
}
export function update(message, model) {
  if (message.Case === "Close") {
    return model.slice(1);
  } else {
    return function (array2) {
      return model.concat(array2);
    }(Array.from(singleton(message.Fields[0])));
  }
}

function modalActionButton(dispatch, action) {
  return createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", function (e_1) {
    dispatch(new Message("Close", []));
    action.action(e_1);
  }], action.buttonType]), action.label);
}

export function view(model, dispatch) {
  if (model.length === 0) {
    return createElement("div", {});
  } else {
    var state = model[0];
    var message = createElement("p", {}, state.message);
    var bodyContents = state.action1 == null ? ofArray([message, modalActionButton(dispatch, state.action0)]) : ofArray([message, modalActionButton(dispatch, state.action0), modalActionButton(dispatch, state.action1)]);
    return createElement("div", {
      className: "modal fade in",
      role: "dialog",
      style: {
        display: "block"
      }
    }, createElement("div", {
      className: "modal-dialog"
    }, createElement("div", {
      className: "modal-content"
    }, createElement.apply(undefined, ["div", {
      className: "modal-body"
    }].concat(_toConsumableArray(bodyContents))))));
  }
}