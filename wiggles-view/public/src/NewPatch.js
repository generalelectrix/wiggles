var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compare, compareUnions, equalsUnions, compareRecords, equalsRecords, Option, makeGeneric } from "fable-core/Util";
import List from "fable-core/List";
import { FixtureKind } from "./Types";
import { map, delay, fold, item, sortWith, toList, tryFind } from "fable-core/Seq";
import { CmdModule } from "fable-elmish/elmish";
import { createElement } from "react";
import { Form } from "./Bootstrap";
import { fsFormat } from "fable-core/String";
export function text(x) {
  return x;
}
export var Model = function () {
  function Model(kinds, selectedKind) {
    _classCallCheck(this, Model);

    this.kinds = kinds;
    this.selectedKind = selectedKind;
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
          selectedKind: Option(FixtureKind)
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
  return new Model(new List(), null);
}
export function update(message, model) {
  if (message.Case === "SetSelected") {
    var matchValue = model.TryGetNamedKind(message.Fields[0]);

    if (matchValue == null) {
      return [model, CmdModule.none()];
    } else {
      return [function () {
        var selectedKind = matchValue;
        return new Model(model.kinds, selectedKind);
      }(), CmdModule.none()];
    }
  } else {
    return [new Model(toList(sortWith(function (x, y) {
      return compare(function (k) {
        return k.name;
      }(x), function (k) {
        return k.name;
      }(y));
    }, message.Fields[0])), model.selectedKind), CmdModule.none()];
  }
}
export var EnterKey = 13;
export var EscapeKey = 27;
export function option(name) {
  return createElement("option", {
    value: name
  }, text(name));
}
export function typeSelector(kinds, selectedKind, dispatchLocal) {
  var selected = selectedKind != null ? selectedKind : item(0, kinds);
  return createElement("div", {}, createElement("div", {}, createElement.apply(undefined, ["select", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", selected.name], ["onChange", function (e_1) {
    dispatchLocal(new Message("SetSelected", [e_1.target.value]));
  }], Form.Control])].concat(_toConsumableArray(toList(delay(function () {
    return map(function (kind) {
      return option(kind.name);
    }, kinds);
  })))))), createElement("div", {}, createElement("span", {}, text(fsFormat("Required channels: %d")(function (x) {
    return x;
  })(selected.channelCount)))));
}
export function view(model, dispatchLocal, dispatchServer) {
  if (model.kinds.tail == null) {
    return createElement("div", {}, text("No patch types available."));
  } else {
    return createElement("div", fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [Form.Group]), createElement("div", {}, text("Create new patch")), typeSelector(model.kinds, model.selectedKind, dispatchLocal));
  }
}