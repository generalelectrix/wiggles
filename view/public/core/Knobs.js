var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compare, Tuple, makeGeneric, compareUnions, equalsUnions, GenericParam } from "fable-core/Util";
import { view as view_1, fromDesc, updateFromValueChange, update as update_1, Message as Message_1, KnobDescription, Data } from "./Knob";
import { map as map_2, ofArray } from "fable-core/List";
import List from "fable-core/List";
import { remove, add, tryFind, create } from "fable-core/Map";
import GenericComparer from "fable-core/GenericComparer";
import { transformMapItem, logError } from "./Util";
import { fsFormat } from "fable-core/String";
import { ResponseFilter } from "./Types";
import { createElement } from "react";
import { filter as filter_1, map as map_3, toList } from "fable-core/Seq";
export var ServerCommand = function () {
  function ServerCommand(caseName, fields) {
    _classCallCheck(this, ServerCommand);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ServerCommand, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knobs.ServerCommand",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Set: [GenericParam("a"), Data],
          State: []
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

  return ServerCommand;
}();
setType("Knobs.ServerCommand", ServerCommand);
export var ServerResponse = function () {
  function ServerResponse(caseName, fields) {
    _classCallCheck(this, ServerResponse);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ServerResponse, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knobs.ServerResponse",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Added: [GenericParam("a"), KnobDescription],
          Removed: [GenericParam("a")],
          State: [makeGeneric(List, {
            T: Tuple([GenericParam("a"), KnobDescription])
          })],
          ValueChange: [GenericParam("a"), Data]
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

  return ServerResponse;
}();
setType("Knobs.ServerResponse", ServerResponse);
export function initModel() {
  return create(null, new GenericComparer(compare));
}
export function initCommands() {
  return ofArray([new ServerCommand("State", [])]);
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
        type: "Knobs.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Particular: [GenericParam("a"), Message_1],
          Response: [makeGeneric(ServerResponse, {
            a: GenericParam("a")
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
setType("Knobs.Message", Message);
export function operateOnKnob(addr, op, model) {
  var matchValue = function (table) {
    return tryFind(addr, table);
  }(model);

  if (matchValue == null) {
    logError(fsFormat("Tried to operate on knob at address %+A but it is not present.")(function (x) {
      return x;
    })(addr));
    return model;
  } else {
    return function () {
      var value = op(matchValue);
      return function (table_1) {
        return add(addr, value, table_1);
      };
    }()(model);
  }
}
export function update(message, model) {
  if (message.Case === "Particular") {
    return function () {
      var f = function f(model_1) {
        return update_1(message.Fields[1], model_1);
      };

      return function (map) {
        return transformMapItem(message.Fields[0], f, map);
      };
    }()(model);
  } else if (message.Fields[0].Case === "ValueChange") {
    return function () {
      var f_1 = function f_1(model_2) {
        return updateFromValueChange(message.Fields[0].Fields[1], model_2);
      };

      return function (map_1) {
        return transformMapItem(message.Fields[0].Fields[0], f_1, map_1);
      };
    }()(model);
  } else if (message.Fields[0].Case === "Added") {
    return function () {
      var value = fromDesc(message.Fields[0].Fields[1]);
      return function (table) {
        return add(message.Fields[0].Fields[0], value, table);
      };
    }()(model);
  } else if (message.Fields[0].Case === "Removed") {
    return function (table_1) {
      return remove(message.Fields[0].Fields[0], table_1);
    }(model);
  } else {
    return create(map_2(function (tupledArg) {
      return [tupledArg[0], fromDesc(tupledArg[1])];
    }, message.Fields[0].Fields[0]), new GenericComparer(compare));
  }
}
export function viewOne(addr, knob, dispatchLocal, dispatchServer) {
  var dispatchChange = function dispatchChange(data) {
    dispatchServer([new ResponseFilter("AllButSelf", []), new ServerCommand("Set", [addr, data])]);
  };

  var dispatchLocal_1 = function dispatchLocal_1(msg) {
    dispatchLocal(function (tupledArg) {
      return new Message("Particular", [tupledArg[0], tupledArg[1]]);
    }([addr, msg]));
  };

  return view_1(knob, dispatchLocal_1, dispatchChange);
}
export function view(addr, model, dispatchLocal, dispatchServer) {
  var matchValue = function (table) {
    return tryFind(addr, table);
  }(model);

  if (matchValue == null) {
    logError(fsFormat("Could not view knob at address %+A because it is not present.")(function (x) {
      return x;
    })(addr));
    return createElement("div", {});
  } else {
    return viewOne(addr, matchValue, dispatchLocal, dispatchServer);
  }
}
export function viewAllWith(filter, model, dispatchLocal, dispatchServer) {
  return createElement.apply(undefined, ["div", {}].concat(_toConsumableArray(toList(map_3(function (tupledArg) {
    return viewOne(tupledArg[0], tupledArg[1], dispatchLocal, dispatchServer);
  }, filter_1(function (tupledArg_1) {
    return filter(tupledArg_1[0]);
  }, model))))));
}