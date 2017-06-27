var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { Option, Any } from "fable-core/Util";
import { Button } from "./Bootstrap";
import { fold, singleton, tryHead } from "fable-core/Seq";
import { enqueueBrowserAction } from "./Util";
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
  function ModalRequest(message, focused, action0, action1) {
    _classCallCheck(this, ModalRequest);

    this.message = message;
    this.focused = focused;
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
          focused: "boolean",
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
          Focus: [],
          Open: [ModalRequest]
        }
      };
    }
  }]);

  return Message;
}();
setType("Modal.Message", Message);
export function confirm(message, action) {
  var okAction = new ModalAction("OK", Button.Primary, action);
  var cancelAction = new ModalAction("Cancel", Button.Default, function (value) {
    value;
  });
  return new ModalRequest(message, false, okAction, cancelAction);
}
export function prompt(message) {
  var okAction = new ModalAction("OK", Button.Primary, function (value) {
    value;
  });
  return new ModalRequest(message, false, okAction, null);
}
export function initialModel() {
  return new Array(0);
}
export var modalOkButtonId = "modal-ok-button";
export function update(message, model) {
  if (message.Case === "Close") {
    return model.slice(1);
  } else if (message.Case === "Focus") {
    var matchValue = tryHead(model);

    if (matchValue != null) {
      if (!matchValue.focused) {
        enqueueBrowserAction(function () {
          document.getElementById(modalOkButtonId).focus();
        });
        var head = void 0;
        var focused = true;
        head = new ModalRequest(matchValue.message, focused, matchValue.action0, matchValue.action1);

        if (model.length > 1) {
          return [head].concat(model.slice(1));
        } else {
          return [head];
        }
      } else {
        return model;
      }
    } else {
      return model;
    }
  } else {
    var newModel = function (array2) {
      return model.concat(array2);
    }(Array.from(singleton(message.Fields[0])));

    return newModel;
  }
}

function modalActionButton(dispatch, id, action) {
  var onClick = ["onClick", function (e) {
    dispatch(new Message("Close", []));
    action.action(e);
  }];
  var buttonAttrs = id == null ? fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [onClick, action.buttonType]) : fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [onClick, action.buttonType, ["id", id]]);
  return createElement("button", buttonAttrs, action.label);
}

export function view(model, dispatch) {
  var matchValue = tryHead(model);

  if (matchValue != null) {
    var message = createElement("p", {}, matchValue.message);
    var bodyContents = void 0;
    var okButton = modalActionButton(dispatch, modalOkButtonId, matchValue.action0);

    if (matchValue.action1 == null) {
      bodyContents = ofArray([message, okButton]);
    } else {
      bodyContents = ofArray([message, okButton, modalActionButton(dispatch, null, matchValue.action1)]);
    }

    if (!matchValue.focused) {
      dispatch(new Message("Focus", []));
    }

    return createElement("div", {
      className: "modal in",
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
  } else {
    return createElement("div", {});
  }
}
export function viewSplash(message) {
  return createElement("div", {
    className: "modal in",
    role: "dialog",
    style: {
      display: "block"
    }
  }, createElement("div", {
    className: "modal-dialog"
  }, createElement("div", {
    className: "modal-content"
  }, createElement("div", {
    className: "modal-body"
  }, createElement("p", {}, message)))));
}