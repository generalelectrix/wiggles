var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compareUnions, equalsUnions, compareRecords, equalsRecords, makeGeneric } from "fable-core/Util";
import { map } from "fable-core/List";
import List from "fable-core/List";
import { fsFormat } from "fable-core/String";
import { parseFloat } from "./Util";
import { createElement } from "react";
import { fold } from "fable-core/Seq";
import { InputType } from "./Bootstrap";
export var nextId = 0;
export var Model = function () {
  function Model(uniqueId, value, min, max, step, detents, inputEventHasFired) {
    _classCallCheck(this, Model);

    this.uniqueId = uniqueId;
    this.value = value;
    this.min = min;
    this.max = max;
    this.step = step;
    this.detents = detents;
    this.inputEventHasFired = inputEventHasFired;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Slider.Model",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          uniqueId: "string",
          value: "number",
          min: "number",
          max: "number",
          step: "number",
          detents: makeGeneric(List, {
            T: "number"
          }),
          inputEventHasFired: "boolean"
        }
      };
    }
  }, {
    key: "Equals",
    value: function (other) {
      return equalsRecords(this, other);
    }
  }, {
    key: "CompareTo",
    value: function (other) {
      return compareRecords(this, other);
    }
  }]);

  return Model;
}();
setType("Slider.Model", Model);

function getUniqueId() {
  var id = nextId;
  nextId = nextId + 1;
  return fsFormat("slider%d")(function (x) {
    return x;
  })(id);
}

export function initModel(value, min, max, step, detents) {
  return new Model(getUniqueId(), value, min, max, step, detents, false);
}
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
        type: "Slider.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          InputHasFired: [],
          ValueChange: ["number"]
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
setType("Slider.Message", Message);
export function update(message, model) {
  if (message.Case === "InputHasFired") {
    var inputEventHasFired = true;
    return new Model(model.uniqueId, model.value, model.min, model.max, model.step, model.detents, inputEventHasFired);
  } else {
    return new Model(model.uniqueId, message.Fields[0], model.min, model.max, model.step, model.detents, model.inputEventHasFired);
  }
}
export function view(onValueChange, model, dispatch) {
  var valueChangeAction = function valueChangeAction(v) {
    dispatch(new Message("ValueChange", [v]));
    onValueChange(v);
  };

  var onInput = function onInput(e) {
    dispatch(new Message("InputHasFired", []));
    var matchValue = parseFloat(e.target.value);
    var $var61 = matchValue != null ? matchValue !== model.value ? [0, matchValue] : [1] : [1];

    switch ($var61[0]) {
      case 0:
        valueChangeAction($var61[1]);
        break;

      case 1:
        break;
    }
  };

  var onChange = function onChange(e_1) {
    if (!model.inputEventHasFired) {
      var matchValue_1 = parseFloat(e_1.target.value);

      if (matchValue_1 != null) {
        valueChangeAction(matchValue_1);
      }
    }
  };

  var detents = map(function (d) {
    return createElement("option", {
      value: String(d)
    });
  }, model.detents);
  return createElement("div", {}, createElement("input", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", String(model.value)], ["list", model.uniqueId], ["onChange", onChange], ["onInput", onInput], ["step", model.step], ["max", model.max], ["min", model.min], InputType.Range])), createElement.apply(undefined, ["datalist", {
    id: model.uniqueId
  }].concat(_toConsumableArray(detents))));
}