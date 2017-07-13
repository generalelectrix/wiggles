var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { ClockId } from "./DataflowTypes";
import { Tuple, makeGeneric, compareUnions, equalsUnions, compareRecords, equalsRecords, Option } from "fable-core/Util";
import List from "fable-core/List";
export var SetInput = function () {
  function SetInput(clock, input, target) {
    _classCallCheck(this, SetInput);

    this.clock = clock;
    this.input = input;
    this.target = target;
  }

  _createClass(SetInput, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "ClockTypes.SetInput",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          clock: ClockId,
          input: "number",
          target: Option(ClockId)
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
setType("ClockTypes.SetInput", SetInput);
export var CreateClock = function () {
  function CreateClock(kind, name) {
    _classCallCheck(this, CreateClock);

    this.kind = kind;
    this.name = name;
  }

  _createClass(CreateClock, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "ClockTypes.CreateClock",
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

  return CreateClock;
}();
setType("ClockTypes.CreateClock", CreateClock);
export var RemoveClock = function () {
  function RemoveClock(id, force) {
    _classCallCheck(this, RemoveClock);

    this.id = id;
    this.force = force;
  }

  _createClass(RemoveClock, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "ClockTypes.RemoveClock",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          id: ClockId,
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

  return RemoveClock;
}();
setType("ClockTypes.RemoveClock", RemoveClock);
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
        type: "ClockTypes.Command",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Classes: [],
          Create: [CreateClock],
          PopInput: [ClockId],
          PushInput: [ClockId],
          Remove: [RemoveClock],
          Rename: [ClockId, "string"],
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
setType("ClockTypes.Command", Command);
export var ClockDescription = function () {
  function ClockDescription(name, kind, inputs) {
    _classCallCheck(this, ClockDescription);

    this.name = name;
    this.kind = kind;
    this.inputs = inputs;
  }

  _createClass(ClockDescription, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "ClockTypes.ClockDescription",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          name: "string",
          kind: "string",
          inputs: makeGeneric(List, {
            T: Option(ClockId)
          })
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

  return ClockDescription;
}();
setType("ClockTypes.ClockDescription", ClockDescription);
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
        type: "ClockTypes.Response",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Classes: [makeGeneric(List, {
            T: "string"
          })],
          New: [ClockId, ClockDescription],
          PopInput: [ClockId],
          PushInput: [ClockId],
          Removed: [ClockId],
          Renamed: [ClockId, "string"],
          SetInput: [SetInput],
          State: [makeGeneric(List, {
            T: Tuple([ClockId, ClockDescription])
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
setType("ClockTypes.Response", Response);