var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { makeGeneric, compareRecords, equalsRecords, Option, Tuple, compareUnions, equalsUnions } from "fable-core/Util";
import List from "fable-core/List";
export var Data = function () {
  function Data(caseName, fields) {
    _classCallCheck(this, Data);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(Data, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WiggleTypes.Data",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Bipolar: ["number"],
          Unipolar: ["number"]
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

  return Data;
}();
setType("WiggleTypes.Data", Data);
export var Datatype = function () {
  function Datatype(caseName, fields) {
    _classCallCheck(this, Datatype);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(Datatype, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WiggleTypes.Datatype",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Bipolar: [],
          Unipolar: []
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
  }, {
    key: "ToString",
    value: function () {
      if (this.Case === "Bipolar") {
        return "bipolar";
      } else {
        return "unipolar";
      }
    }
  }]);

  return Datatype;
}();
setType("WiggleTypes.Datatype", Datatype);
export function datatype(data, _arg1) {
  if (_arg1.Case === "Bipolar") {
    return new Datatype("Bipolar", []);
  } else {
    return new Datatype("Unipolar", []);
  }
}
export var SetInput = function () {
  function SetInput(wiggle, input, target) {
    _classCallCheck(this, SetInput);

    this.wiggle = wiggle;
    this.input = input;
    this.target = target;
  }

  _createClass(SetInput, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WiggleTypes.SetInput",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          wiggle: Tuple(["number", "number"]),
          input: "number",
          target: Option(Tuple([Tuple(["number", "number"]), "number"]))
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

  return SetInput;
}();
setType("WiggleTypes.SetInput", SetInput);
export var CreateWiggle = function () {
  function CreateWiggle(kind, name) {
    _classCallCheck(this, CreateWiggle);

    this.kind = kind;
    this.name = name;
  }

  _createClass(CreateWiggle, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WiggleTypes.CreateWiggle",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          kind: "string",
          name: "string"
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

  return CreateWiggle;
}();
setType("WiggleTypes.CreateWiggle", CreateWiggle);
export var RemoveWiggle = function () {
  function RemoveWiggle(id, force) {
    _classCallCheck(this, RemoveWiggle);

    this.id = id;
    this.force = force;
  }

  _createClass(RemoveWiggle, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WiggleTypes.RemoveWiggle",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          id: Tuple(["number", "number"]),
          force: "boolean"
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

  return RemoveWiggle;
}();
setType("WiggleTypes.RemoveWiggle", RemoveWiggle);
export var Command = function () {
  function Command(caseName, fields) {
    _classCallCheck(this, Command);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(Command, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WiggleTypes.Command",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Create: [CreateWiggle],
          Kinds: [],
          PopInput: [Tuple(["number", "number"])],
          PopOutput: [Tuple(["number", "number"])],
          PushInput: [Tuple(["number", "number"])],
          PushOutput: [Tuple(["number", "number"])],
          Remove: [RemoveWiggle],
          Rename: [Tuple(["number", "number"]), "string"],
          SetClock: [Tuple(["number", "number"]), Option(Tuple(["number", "number"]))],
          SetInput: [SetInput],
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

  return Command;
}();
setType("WiggleTypes.Command", Command);
export var UsesClock = function () {
  function UsesClock(caseName, fields) {
    _classCallCheck(this, UsesClock);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(UsesClock, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WiggleTypes.UsesClock",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          No: [],
          Yes: [Option(Tuple(["number", "number"]))]
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

  return UsesClock;
}();
setType("WiggleTypes.UsesClock", UsesClock);
export var WiggleDescription = function () {
  function WiggleDescription(name, kind, inputs, outputs, clock) {
    _classCallCheck(this, WiggleDescription);

    this.name = name;
    this.kind = kind;
    this.inputs = inputs;
    this.outputs = outputs;
    this.clock = clock;
  }

  _createClass(WiggleDescription, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WiggleTypes.WiggleDescription",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          name: "string",
          kind: "string",
          inputs: makeGeneric(List, {
            T: Option(Tuple([Tuple(["number", "number"]), "number"]))
          }),
          outputs: "number",
          clock: UsesClock
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

  return WiggleDescription;
}();
setType("WiggleTypes.WiggleDescription", WiggleDescription);
export var Response = function () {
  function Response(caseName, fields) {
    _classCallCheck(this, Response);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(Response, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WiggleTypes.Response",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Kinds: [makeGeneric(List, {
            T: "string"
          })],
          New: [Tuple(["number", "number"]), WiggleDescription],
          PopInput: [Tuple(["number", "number"])],
          PopOutput: [Tuple(["number", "number"])],
          PushInput: [Tuple(["number", "number"])],
          PushOutput: [Tuple(["number", "number"])],
          Removed: [Tuple(["number", "number"])],
          Renamed: [Tuple(["number", "number"]), "string"],
          SetClock: [Tuple(["number", "number"]), Option(Tuple(["number", "number"]))],
          SetInput: [SetInput],
          State: [makeGeneric(List, {
            T: Tuple([Tuple(["number", "number"]), WiggleDescription])
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

  return Response;
}();
setType("WiggleTypes.Response", Response);