var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compareUnions, equalsUnions, makeGeneric, compareRecords, equalsRecords, Tuple, Option } from "fable-core/Util";
import { map, ofArray } from "fable-core/List";
import List from "fable-core/List";
import { createElement } from "react";
import { map as map_1, singleton, append, delay, toList } from "fable-core/Seq";
import { ProgramModule } from "fable-elmish/elmish";
import { withReact } from "fable-elmish-react/react";
export function text(x) {
  return x;
}
export var PatchItem = function () {
  function PatchItem(id, name, address, channelCount) {
    _classCallCheck(this, PatchItem);

    this.id = id;
    this.name = name;
    this.address = address;
    this.channelCount = channelCount;
  }

  _createClass(PatchItem, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "App.PatchItem",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          id: "number",
          name: "string",
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
  }]);

  return PatchItem;
}();
setType("App.PatchItem", PatchItem);
export var Model = function () {
  function Model(patches) {
    _classCallCheck(this, Model);

    this.patches = patches;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "App.Model",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          patches: makeGeneric(List, {
            T: PatchItem
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

  return Model;
}();
setType("App.Model", Model);
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
        type: "App.ServerRequest",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          NewPatch: [PatchItem],
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
setType("App.ServerRequest", ServerRequest);
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
        type: "App.ServerResponse",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Error: ["string"],
          NewPatch: [PatchItem],
          PatchState: [makeGeneric(List, {
            T: PatchItem
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
setType("App.ServerResponse", ServerResponse);
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
        type: "App.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Request: [ServerRequest],
          Response: [ServerResponse]
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
setType("App.Message", Message);
export var testPatches = ofArray([new PatchItem(0, "foo", null, 2), new PatchItem(1, "charlie", [0, 27], 1)]);
export function initialModel() {
  return new Model(testPatches);
}
export function update(message, model) {
  return model;
}
export function viewPatchItem(item) {
  var td = function td(x) {
    return createElement("td", {}, text(x));
  };

  var patternInput = void 0;

  if (item.address == null) {
    patternInput = ["", ""];
  } else {
    var u = item.address[0];
    var a = item.address[1];
    patternInput = [String(u), String(a)];
  }

  return createElement("tr", {}, td(item.id), td(item.name), td(patternInput[0]), td(patternInput[1]), td(item.channelCount));
}
export var patchTableHeader = createElement.apply(undefined, ["tr", {}].concat(_toConsumableArray(map(function (x) {
  return createElement("th", {}, text(x));
}, ofArray(["id", "name", "universe", "address", "channel count"])))));
export function viewPatchTable(patches) {
  return createElement.apply(undefined, ["table", {}].concat(_toConsumableArray(toList(delay(function () {
    return append(singleton(patchTableHeader), delay(function () {
      return map_1(function (patch) {
        return viewPatchItem(patch);
      }, patches);
    }));
  })))));
}
export function view(model, dispatch) {
  return createElement("div", {}, viewPatchTable(model.patches));
}
ProgramModule.run(withReact("app", ProgramModule.mkSimple(function () {
  return initialModel();
}, function (message) {
  return function (model) {
    return update(message, model);
  };
}, function (model_1) {
  return function (dispatch) {
    return view(model_1, dispatch);
  };
})));