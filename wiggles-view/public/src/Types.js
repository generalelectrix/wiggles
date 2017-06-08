var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { map, delay, toList } from "fable-core/Seq";
import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compareUnions, equalsUnions, makeGeneric, defaultArg, Tuple, Option, compareRecords, equalsRecords } from "fable-core/Util";
import { ofArray } from "fable-core/List";
import List from "fable-core/List";
export var Cmd = function (__exports) {
  var ofMsgs = __exports.ofMsgs = function (msgs) {
    return toList(delay(function () {
      return map(function (msg) {
        return function (dispatch) {
          dispatch(msg);
        };
      }, msgs);
    }));
  };

  return __exports;
}({});
export var FixtureKind = function () {
  function FixtureKind(name, channelCount) {
    _classCallCheck(this, FixtureKind);

    this.name = name;
    this.channelCount = channelCount;
  }

  _createClass(FixtureKind, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Types.FixtureKind",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          name: "string",
          channelCount: "number"
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

  return FixtureKind;
}();
setType("Types.FixtureKind", FixtureKind);
export var PatchRequest = function () {
  function PatchRequest(name, kind, address) {
    _classCallCheck(this, PatchRequest);

    this.name = name;
    this.kind = kind;
    this.address = address;
  }

  _createClass(PatchRequest, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Types.PatchRequest",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          name: "string",
          kind: "string",
          address: Option(Tuple(["number", "number"]))
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

  return PatchRequest;
}();
setType("Types.PatchRequest", PatchRequest);
export var PatchItem = function () {
  function PatchItem(id, name, kind, address, channelCount) {
    _classCallCheck(this, PatchItem);

    this.id = id;
    this.name = name;
    this.kind = kind;
    this.address = address;
    this.channelCount = channelCount;
  }

  _createClass(PatchItem, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Types.PatchItem",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          id: "number",
          name: "string",
          kind: "string",
          address: Option(Tuple(["number", "number"])),
          channelCount: "number"
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
    key: "universe",
    get: function () {
      return defaultArg(this.address, null, function (tuple) {
        return tuple[0];
      });
    }
  }, {
    key: "dmxAddress",
    get: function () {
      return defaultArg(this.address, null, function (tuple) {
        return tuple[1];
      });
    }
  }]);

  return PatchItem;
}();
setType("Types.PatchItem", PatchItem);
export var testPatches = ofArray([new PatchItem(0, "foo", "dimmer", null, 2), new PatchItem(1, "charlie", "roto", [0, 27], 1)]);
export var testKinds = ofArray([new FixtureKind("dimmer", 1), new FixtureKind("roto", 2)]);
export var ServerRequest = function () {
  function ServerRequest(caseName, fields) {
    _classCallCheck(this, ServerRequest);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ServerRequest, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Types.ServerRequest",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          GetKinds: [],
          NewPatches: [makeGeneric(List, {
            T: PatchRequest
          })],
          PatchState: [],
          Remove: ["number"],
          Rename: ["number", "string"],
          Repatch: ["number", Option(Tuple(["number", "number"]))]
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

  return ServerRequest;
}();
setType("Types.ServerRequest", ServerRequest);
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
        type: "Types.ServerResponse",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Error: ["string"],
          Kinds: [makeGeneric(List, {
            T: FixtureKind
          })],
          NewPatches: [makeGeneric(List, {
            T: PatchItem
          })],
          PatchState: [makeGeneric(List, {
            T: PatchItem
          })],
          Remove: ["number"],
          Update: [PatchItem]
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
setType("Types.ServerResponse", ServerResponse);