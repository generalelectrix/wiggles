var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { makeGeneric, compareUnions, equalsUnions, Array as _Array, defaultArg, Tuple, Option, compareRecords, equalsRecords } from "fable-core/Util";
import { Result } from "fable-elmish/result";
import { parseOptionalNumber } from "../core/Util";
import List from "fable-core/List";
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
        type: "PatchTypes.FixtureKind",
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
setType("PatchTypes.FixtureKind", FixtureKind);
export function validUniverse(u) {
  if (u >= 0) {
    return new Result("Ok", [u]);
  } else {
    return new Result("Error", [null]);
  }
}
export function validDmxAddress(a) {
  if (a > 0 ? a < 513 : false) {
    return new Result("Ok", [a]);
  } else {
    return new Result("Error", [null]);
  }
}
export var parseDmxAddress = function parseDmxAddress(v) {
  return parseOptionalNumber(function (a) {
    return validDmxAddress(a);
  }, v);
};
export var parseUniverseId = function parseUniverseId(v) {
  return parseOptionalNumber(function (u) {
    return validUniverse(u);
  }, v);
};
export function globalAddressFromOptionals(univOpt, addrOpt) {
  var matchValue = [univOpt, addrOpt];
  var $var159 = matchValue[0].Case === "Absent" ? matchValue[1].Case === "Absent" ? [1] : [2] : matchValue[1].Case === "Present" ? [0, matchValue[1].Fields[0], matchValue[0].Fields[0]] : [2];

  switch ($var159[0]) {
    case 0:
      return new Result("Ok", [[$var159[2], $var159[1]]]);

    case 1:
      return new Result("Ok", [null]);

    case 2:
      return new Result("Error", [null]);
  }
}
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
        type: "PatchTypes.PatchRequest",
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
setType("PatchTypes.PatchRequest", PatchRequest);
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
        type: "PatchTypes.PatchItem",
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
setType("PatchTypes.PatchItem", PatchItem);
export var PortAttachment = function () {
  function PortAttachment(universe, portNamespace, portName) {
    _classCallCheck(this, PortAttachment);

    this.universe = universe;
    this.portNamespace = portNamespace;
    this.portName = portName;
  }

  _createClass(PortAttachment, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "PatchTypes.PortAttachment",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          universe: "number",
          portNamespace: "string",
          portName: "string"
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

  return PortAttachment;
}();
setType("PatchTypes.PortAttachment", PortAttachment);
export var PatchServerRequest = function () {
  function PatchServerRequest(caseName, fields) {
    _classCallCheck(this, PatchServerRequest);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(PatchServerRequest, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "PatchTypes.PatchServerRequest",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          AddUniverse: [],
          AttachPort: [PortAttachment],
          AvailablePorts: [],
          GetKinds: [],
          NewPatches: [_Array(PatchRequest)],
          PatchState: [],
          Remove: ["number"],
          RemoveUniverse: ["number", "boolean"],
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

  return PatchServerRequest;
}();
setType("PatchTypes.PatchServerRequest", PatchServerRequest);
export var PatchServerResponse = function () {
  function PatchServerResponse(caseName, fields) {
    _classCallCheck(this, PatchServerResponse);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(PatchServerResponse, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "PatchTypes.PatchServerResponse",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          AvailablePorts: [makeGeneric(List, {
            T: Tuple(["string", "string"])
          })],
          Kinds: [_Array(FixtureKind)],
          NewPatches: [_Array(PatchItem)],
          NewUniverse: ["number"],
          PatchState: [_Array(PatchItem)],
          PortAttached: [PortAttachment],
          Remove: ["number"],
          UniverseRemoved: ["number"],
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

  return PatchServerResponse;
}();
setType("PatchTypes.PatchServerResponse", PatchServerResponse);