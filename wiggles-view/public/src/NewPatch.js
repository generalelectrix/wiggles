var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compare, compareUnions, equalsUnions, compareRecords, equalsRecords, Option, makeGeneric } from "fable-core/Util";
import { ofArray, map } from "fable-core/List";
import List from "fable-core/List";
import { FixtureKind } from "./Types";
import { fold, item, sortWith, toList, tryFind } from "fable-core/Seq";
import { CmdModule } from "fable-elmish/elmish";
import { createElement } from "react";
import { fsFormat } from "fable-core/String";
import { Grid, Form } from "./Bootstrap";
export function text(x) {
  return x;
}
export var Model = function () {
  function Model(kinds, selectedKind, address, quantity) {
    _classCallCheck(this, Model);

    this.kinds = kinds;
    this.selectedKind = selectedKind;
    this.address = address;
    this.quantity = quantity;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "NewPatch.Model",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          kinds: makeGeneric(List, {
            T: FixtureKind
          }),
          selectedKind: Option(FixtureKind),
          address: "number",
          quantity: "number"
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
  }, {
    key: "TryGetNamedKind",
    value: function (name) {
      return tryFind(function (k) {
        return k.name === name;
      }, this.kinds);
    }
  }]);

  return Model;
}();
setType("NewPatch.Model", Model);
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
        type: "NewPatch.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          SetAddress: ["number"],
          SetQuantity: ["number"],
          SetSelected: ["string"],
          UpdateKinds: [makeGeneric(List, {
            T: FixtureKind
          })]
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
setType("NewPatch.Message", Message);
export function initialModel() {
  return new Model(new List(), null, 1, 1);
}
export function update(message, model) {
  return function (m) {
    return [m, CmdModule.none()];
  }(message.Case === "SetSelected" ? function () {
    var matchValue = model.TryGetNamedKind(message.Fields[0]);

    if (matchValue == null) {
      return model;
    } else {
      var selectedKind = matchValue;
      return new Model(model.kinds, selectedKind, model.address, model.quantity);
    }
  }() : message.Case === "SetAddress" ? (message.Fields[0] > 0 ? message.Fields[0] < 513 : false) ? new Model(model.kinds, model.selectedKind, message.Fields[0], model.quantity) : model : message.Case === "SetQuantity" ? message.Fields[0] > 0 ? new Model(model.kinds, model.selectedKind, model.address, message.Fields[0]) : model : new Model(toList(sortWith(function (x, y) {
    return compare(function (k) {
      return k.name;
    }(x), function (k) {
      return k.name;
    }(y));
  }, message.Fields[0])), model.selectedKind, model.address, model.quantity));
}
export var EnterKey = 13;
export var EscapeKey = 27;
export function typeSelector(kinds, selectedKind, dispatchLocal) {
  var option = function option(kind) {
    return createElement("option", {
      value: kind.name
    }, text(fsFormat("%s (%d ch)")(function (x) {
      return x;
    })(kind.name)(kind.channelCount)));
  };

  var selected = selectedKind != null ? selectedKind : item(0, kinds);
  return createElement("div", {}, createElement.apply(undefined, ["select", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", selected.name], ["onChange", function (e_1) {
    dispatchLocal(new Message("SetSelected", [e_1.target.value]));
  }], Form.Control])].concat(_toConsumableArray(function (list) {
    return map(option, list);
  }(kinds)))));
}
export function numericEditBox(dispatchLocal, label, cmd, value) {
  return createElement("label", {}, text(label), createElement("input", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", String(value)], ["onChange", function (e_1) {
    dispatchLocal(cmd(function () {
      var matchValue_1 = e_1.target.value;

      if (matchValue_1 === "") {
        return 1;
      } else {
        return matchValue_1;
      }
    }()));
  }], ["type", "number"], Form.Control])));
}
export function view(model, dispatchLocal, dispatchServer) {
  if (model.kinds.tail == null) {
    return createElement("div", {}, text("No patch types available."));
  } else {
    return createElement("div", fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [Form.Group]), createElement("span", {}, createElement("h3", {}, text("Create new patch"))), typeSelector(model.kinds, model.selectedKind, dispatchLocal), Grid.distribute(ofArray([ofArray([numericEditBox(dispatchLocal, "Quantity", function (arg0) {
      return new Message("SetQuantity", [arg0]);
    }, model.quantity)]), ofArray([numericEditBox(dispatchLocal, "Start address", function (arg0_1) {
      return new Message("SetAddress", [arg0_1]);
    }, model.address)])])));
  }
}