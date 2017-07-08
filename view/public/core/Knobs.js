var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compare, compareUnions, equalsUnions, makeGeneric, compareRecords, equalsRecords, GenericParam } from "fable-core/Util";
import { view as view_1, updateFromValueChange, fromDesc, update as update_1, Message as Message_1, KnobDescription, Data } from "./Knob";
import { remove, add, tryFind, create } from "fable-core/Map";
import GenericComparer from "fable-core/GenericComparer";
import { logError } from "./Util";
import { fsFormat } from "fable-core/String";
import { ResponseFilter } from "./Types";
import { createElement } from "react";
export var ValueChange = function () {
  function ValueChange(addr, value) {
    _classCallCheck(this, ValueChange);

    this.addr = addr;
    this.value = value;
  }

  _createClass(ValueChange, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knobs.ValueChange",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          addr: GenericParam("a"),
          value: Data
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

  return ValueChange;
}();
setType("Knobs.ValueChange", ValueChange);
export var KnobAdded = function () {
  function KnobAdded(addr, desc) {
    _classCallCheck(this, KnobAdded);

    this.addr = addr;
    this.desc = desc;
  }

  _createClass(KnobAdded, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knobs.KnobAdded",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          addr: GenericParam("a"),
          desc: KnobDescription
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

  return KnobAdded;
}();
setType("Knobs.KnobAdded", KnobAdded);
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
          Set: [makeGeneric(ValueChange, {
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
          KnobAdded: [makeGeneric(KnobAdded, {
            a: GenericParam("a")
          })],
          KnobRemoved: [GenericParam("a")],
          ValueChange: [makeGeneric(ValueChange, {
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

  return ServerResponse;
}();
setType("Knobs.ServerResponse", ServerResponse);
export function initModel() {
  return create(null, new GenericComparer(compare));
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
      var op = function op(model_1) {
        return update_1(message.Fields[1], model_1);
      };

      return function (model_2) {
        return operateOnKnob(message.Fields[0], op, model_2);
      };
    }()(model);
  } else if (message.Fields[0].Case === "KnobAdded") {
    return add(message.Fields[0].Fields[0].addr, fromDesc(message.Fields[0].Fields[0].desc), model);
  } else if (message.Fields[0].Case === "KnobRemoved") {
    return function (table) {
      return remove(message.Fields[0].Fields[0], table);
    }(model);
  } else {
    return operateOnKnob(message.Fields[0].Fields[0].addr, function (model_3) {
      return updateFromValueChange(message.Fields[0].Fields[0].value, model_3);
    }, model);
  }
}
export function viewOne(addr, knob, dispatchLocal, dispatchServer) {
  var dispatchChange = function dispatchChange(data) {
    dispatchServer([new ResponseFilter("AllButSelf", []), new ServerCommand("Set", [new ValueChange(addr, data)])]);
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