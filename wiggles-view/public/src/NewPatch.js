var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compare, defaultArg, compareUnions, equalsUnions, compareRecords, equalsRecords, Option, makeGeneric } from "fable-core/Util";
import { ofArray, map } from "fable-core/List";
import List from "fable-core/List";
import { FixtureKind } from "./Types";
import { fold, item, sortWith, toList, tryFind } from "fable-core/Seq";
import { CmdModule } from "fable-elmish/elmish";
import { createElement } from "react";
import { fsFormat } from "fable-core/String";
import { Grid, Button, Form } from "./Bootstrap";
export var Model = function () {
  function Model(kinds, selectedKind, name, universe, address, quantity) {
    _classCallCheck(this, Model);

    this.kinds = kinds;
    this.selectedKind = selectedKind;
    this.name = name;
    this.universe = universe;
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
          name: "string",
          universe: Option("number"),
          address: Option("number"),
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
          AdvanceAddress: [],
          SetAddress: [Option("number")],
          SetQuantity: ["number"],
          SetSelected: ["string"],
          SetUniverse: [Option("number")],
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
  return new Model(new List(), null, "", null, null, 1);
}
export function positive(x) {
  if (x > 0) {
    return x;
  } else {
    return 0;
  }
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
      return new Model(model.kinds, selectedKind, model.name, model.universe, model.address, model.quantity);
    }
  }() : message.Case === "SetUniverse" ? function () {
    var universe = function (option) {
      return defaultArg(option, null, function (x) {
        return positive(x);
      });
    }(message.Fields[0]);

    return new Model(model.kinds, model.selectedKind, model.name, universe, model.address, model.quantity);
  }() : message.Case === "SetAddress" ? function () {
    var address = defaultArg(message.Fields[0], null, function ($var42) {
      return 1 > (512 < $var42 ? 512 : $var42) ? 1 : 512 < $var42 ? 512 : $var42;
    });
    return new Model(model.kinds, model.selectedKind, model.name, model.universe, address, model.quantity);
  }() : message.Case === "SetQuantity" ? function () {
    var quantity = positive(message.Fields[0]);
    return new Model(model.kinds, model.selectedKind, model.name, model.universe, model.address, quantity);
  }() : message.Case === "AdvanceAddress" ? model.address == null ? model : function () {
    var channelCount = model.selectedKind == null ? 0 : model.selectedKind.channelCount;
    var address_1 = 512 < model.address + model.quantity * channelCount ? 512 : model.address + model.quantity * channelCount;
    return new Model(model.kinds, model.selectedKind, model.name, model.universe, address_1, model.quantity);
  }() : new Model(toList(sortWith(function (x, y) {
    return compare(function (k) {
      return k.name;
    }(x), function (k) {
      return k.name;
    }(y));
  }, message.Fields[0])), model.selectedKind, model.name, model.universe, model.address, model.quantity));
}
export var EnterKey = 13;
export var EscapeKey = 27;
export function typeSelector(kinds, selectedKind, dispatchLocal) {
  var option = function option(kind) {
    return createElement("option", {
      value: kind.name
    }, fsFormat("%s (%d ch)")(function (x) {
      return x;
    })(kind.name)(kind.channelCount));
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
export function numericEditBox(dispatchLocal, handleValue, label, cmd, value) {
  return createElement("label", {}, label, createElement("input", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", value], ["onChange", function (e_1) {
    dispatchLocal(cmd(handleValue(e_1.target.value)));
  }], ["type", "number"], Form.Control])));
}
export function patchButton(model, dispatchLocal, dispatchServer) {
  return createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", function (_arg1_1) {}], Button.Warning]), "Patch");
}
export function noneIfEmpty(s) {
  if (s === "") {
    return null;
  } else {
    return Number.parseInt(s);
  }
}
export function emptyIfNone(_arg1) {
  if (_arg1 != null) {
    return String(_arg1);
  } else {
    return "";
  }
}
export function view(model, dispatchLocal, dispatchServer) {
  if (model.kinds.tail == null) {
    return createElement("div", {}, "No patch types available.");
  } else {
    var universeEntry = numericEditBox(dispatchLocal, function (s) {
      return noneIfEmpty(s);
    }, "Universe", function (arg0) {
      return new Message("SetUniverse", [arg0]);
    }, emptyIfNone(model.universe));
    var addressEntry = numericEditBox(dispatchLocal, function (s_1) {
      return noneIfEmpty(s_1);
    }, "Start address", function (arg0_1) {
      return new Message("SetAddress", [arg0_1]);
    }, emptyIfNone(model.address));
    var quantityEntry = numericEditBox(dispatchLocal, function (v) {
      return v === "" ? 1 : Number.parseInt(v);
    }, "Quantity", function (arg0_2) {
      return new Message("SetQuantity", [arg0_2]);
    }, String(model.quantity));
    return createElement("div", fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [Form.Group]), createElement("span", {}, createElement("h3", {}, "Create new patch")), typeSelector(model.kinds, model.selectedKind, dispatchLocal), Grid.distribute(ofArray([ofArray([universeEntry]), ofArray([addressEntry]), ofArray([quantityEntry])])));
  }
}