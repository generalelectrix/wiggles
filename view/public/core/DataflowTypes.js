var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { Tuple, compareUnions, equalsUnions } from "fable-core/Util";
export var ClockId = function () {
  function ClockId(caseName, fields) {
    _classCallCheck(this, ClockId);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ClockId, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "DataflowTypes.ClockId",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          ClockId: ["number", "number"]
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

  return ClockId;
}();
setType("DataflowTypes.ClockId", ClockId);
export var WiggleId = function () {
  function WiggleId(caseName, fields) {
    _classCallCheck(this, WiggleId);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(WiggleId, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "DataflowTypes.WiggleId",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          WiggleId: ["number", "number"]
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

  return WiggleId;
}();
setType("DataflowTypes.WiggleId", WiggleId);
export var KnobAddress = function () {
  function KnobAddress(caseName, fields) {
    _classCallCheck(this, KnobAddress);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(KnobAddress, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "DataflowTypes.KnobAddress",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Clock: [Tuple([ClockId, "number"])],
          Wiggle: [Tuple([WiggleId, "number"])]
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

  return KnobAddress;
}();
setType("DataflowTypes.KnobAddress", KnobAddress);