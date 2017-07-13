var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compareUnions, equalsUnions, Tuple } from "fable-core/Util";
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
          Clock: [Tuple([Tuple(["number", "number"]), "number"])],
          Wiggle: [Tuple([Tuple(["number", "number"]), "number"])]
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