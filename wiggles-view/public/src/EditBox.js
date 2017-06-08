var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { toString, compareUnions, equalsUnions, makeGeneric, GenericParam, Option } from "fable-core/Util";
import { Result } from "fable-elmish/result";
import { fold } from "fable-core/Seq";
import { Form } from "./Bootstrap";
import { createElement } from "react";
export var Model = function () {
  function Model(value, parser, label, inputType) {
    _classCallCheck(this, Model);

    this.value = value;
    this.parser = parser;
    this.label = label;
    this.inputType = inputType;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "EditBox.Model",
        interfaces: ["FSharpRecord"],
        properties: {
          value: Option(makeGeneric(Result, {
            s: GenericParam("T"),
            f: "string"
          })),
          parser: "function",
          label: "string",
          inputType: "string"
        }
      };
    }
  }, {
    key: "ParsedValueOr",
    value: function (defaultValue) {
      var $var7 = this.value != null ? this.value.Case === "Ok" ? [0, this.value.Fields[0]] : [1] : [1];

      switch ($var7[0]) {
        case 0:
          return $var7[1];

        case 1:
          return defaultValue;
      }
    }
  }, {
    key: "IsOk",
    get: function () {
      var $var5 = this.value != null ? this.value.Case === "Error" ? [0] : [1] : [1];

      switch ($var5[0]) {
        case 0:
          return false;

        case 1:
          return true;
      }
    }
  }, {
    key: "HasParsed",
    get: function () {
      var $var6 = this.value != null ? this.value.Case === "Ok" ? [0] : [1] : [1];

      switch ($var6[0]) {
        case 0:
          return true;

        case 1:
          return false;
      }
    }
  }]);

  return Model;
}();
setType("EditBox.Model", Model);
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
        type: "EditBox.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Clear: [],
          Update: ["string"]
        }
      };
    }
  }, {
    key: "Equals",
    value: function (other) {
      return equalsUnions(this, other);
    }
  }, {
    key: "CompareTo",
    value: function (other) {
      return compareUnions(this, other);
    }
  }]);

  return Message;
}();
setType("EditBox.Message", Message);
export function initialModel(label, parser, inputType) {
  return new Model(null, parser, label, inputType);
}
export function update(message, model) {
  if (message.Case === "Clear") {
    return new Model(null, model.parser, model.label, model.inputType);
  } else {
    var parseResult = void 0;
    var matchValue = model.parser(message.Fields[0]);

    if (matchValue.Case === "Ok") {
      parseResult = new Result("Ok", [matchValue.Fields[0]]);
    } else {
      parseResult = new Result("Error", [message.Fields[0]]);
    }

    return new Model(parseResult, model.parser, model.label, model.inputType);
  }
}
export function view(extraAction, defaultValue, model, dispatch) {
  var value = model.value != null ? model.value.Case === "Error" ? model.value.Fields[0] : toString(model.value.Fields[0]) : defaultValue;
  var attrs = fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", value], ["onChange", function (e_1) {
    dispatch(new Message("Update", [e_1.target.value]));
  }], ["type", model.inputType], Form.Control]);
  var allAttrs = extraAction == null ? attrs : Object.assign({}, fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [extraAction(model)]), attrs);
  return createElement("div", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [model.IsOk ? Form.Group : Form.GroupError]), createElement("label", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Form.ControlLabel]), model.label, createElement("input", allAttrs)));
}