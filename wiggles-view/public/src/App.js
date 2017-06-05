var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { equals, defaultArg, compareUnions, equalsUnions, Array as _Array, makeGeneric, compareRecords, equalsRecords, Tuple, Option } from "fable-core/Util";
import { filter, map, ofArray } from "fable-core/List";
import List from "fable-core/List";
import { ProgramModule, CmdModule } from "fable-elmish/elmish";
import { join, fsFormat } from "fable-core/String";
import { map as map_1, singleton, append, delay, toList, fold, tryFind } from "fable-core/Seq";
import { createElement } from "react";
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
  function Model(patches, selected, consoleText) {
    _classCallCheck(this, Model);

    this.patches = patches;
    this.selected = selected;
    this.consoleText = consoleText;
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
          }),
          selected: Option("number"),
          consoleText: _Array("string")
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
export function withConsoleMessage(msg, model) {
  var consoleText = model.consoleText.concat([msg]);
  return new Model(model.patches, model.selected, consoleText);
}
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
setType("App.ServerResponse", ServerResponse);
export var UiAction = function () {
  function UiAction(caseName, fields) {
    _classCallCheck(this, UiAction);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(UiAction, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "App.UiAction",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          ClearConsole: [],
          Deselect: [],
          SetSelected: ["number"]
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

  return UiAction;
}();
setType("App.UiAction", UiAction);
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
          Action: [UiAction],
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
  return [new Model(testPatches, null, new Array(0)), CmdModule.none()];
}
export function mockServer(model, req) {
  var maybeUpdatePatch = function maybeUpdatePatch(msgType) {
    return function (op) {
      return function (patchId) {
        return function (_arg1) {
          return _arg1 == null ? new ServerResponse("Error", [fsFormat("Unknown fixture id %d")(function (x) {
            return x;
          })(patchId)]) : _arg1;
        }(defaultArg(tryFind(function (p) {
          return p.id === patchId;
        }, model.patches), null, function ($var1) {
          return msgType(op($var1));
        }));
      };
    };
  };

  if (req.Case === "NewPatch") {
    return new ServerResponse("NewPatch", [req.Fields[0]]);
  } else if (req.Case === "Rename") {
    return maybeUpdatePatch(function (arg0) {
      return new ServerResponse("Update", [arg0]);
    })(function (p_1) {
      return new PatchItem(p_1.id, req.Fields[1], p_1.address, p_1.channelCount);
    })(req.Fields[0]);
  } else if (req.Case === "Repatch") {
    return maybeUpdatePatch(function (arg0_1) {
      return new ServerResponse("Update", [arg0_1]);
    })(function (p_2) {
      return new PatchItem(p_2.id, p_2.name, req.Fields[1], p_2.channelCount);
    })(req.Fields[0]);
  } else if (req.Case === "Remove") {
    return maybeUpdatePatch(function (arg0_2) {
      return new ServerResponse("Remove", [arg0_2]);
    })(function (_arg1_1) {
      return req.Fields[0];
    })(req.Fields[0]);
  } else {
    return new ServerResponse("PatchState", [model.patches]);
  }
}
export var purple = ["color", "#6600ff"];
export var cyan = ["color", "#00ccff"];
export function update(message, model) {
  if (message.Case === "Response") {
    var newModel = void 0;

    if (message.Fields[0].Case === "PatchState") {
      newModel = new Model(message.Fields[0].Fields[0], model.selected, model.consoleText);
    } else if (message.Fields[0].Case === "NewPatch") {
      newModel = new Model(new List(message.Fields[0].Fields[0], model.patches), model.selected, model.consoleText);
    } else if (message.Fields[0].Case === "Update") {
      var newPatches = map(function (existing) {
        return existing.id === message.Fields[0].Fields[0].id ? message.Fields[0].Fields[0] : existing;
      }, model.patches);
      newModel = new Model(newPatches, model.selected, model.consoleText);
    } else if (message.Fields[0].Case === "Remove") {
      newModel = new Model(filter(function (p) {
        return p.id === message.Fields[0].Fields[0];
      }, model.patches), model.selected, model.consoleText);
    } else {
      newModel = function (model_1) {
        return withConsoleMessage(message.Fields[0].Fields[0], model_1);
      }(model);
    }

    return [newModel, CmdModule.none()];
  } else if (message.Case === "Action") {
    return function (m) {
      return [m, CmdModule.none()];
    }(function (_arg1) {
      if (_arg1.Case === "SetSelected") {
        var selected = _arg1.Fields[0];
        return new Model(model.patches, selected, model.consoleText);
      } else if (_arg1.Case === "Deselect") {
        var selected_1 = null;
        return new Model(model.patches, selected_1, model.consoleText);
      } else {
        var consoleText = new Array(0);
        return new Model(model.patches, model.selected, consoleText);
      }
    }(message.Fields[0]));
  } else {
    return [model, CmdModule.ofMsg(new Message("Response", [mockServer(model, message.Fields[0])]))];
  }
}
export function updateAndLog(message, model) {
  var patternInput = update(message, model);
  return [withConsoleMessage(fsFormat("%+A")(function (x) {
    return x;
  })(message), patternInput[0]), patternInput[1]];
}
export function viewPatchTableRow(dispatch, selectedId, item) {
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

  return createElement("tr", {}, createElement("td", {}, createElement("button", {
    onClick: function onClick(_arg1) {
      dispatch(new Message("Action", [new UiAction("SetSelected", [item.id])]));
    },
    style: fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [equals(item.id, selectedId) ? purple : cyan])
  })), td(item.id), td(item.name), td(patternInput[0]), td(patternInput[1]), td(item.channelCount));
}
export var patchTableHeader = createElement.apply(undefined, ["tr", {}].concat(_toConsumableArray(map(function (x) {
  return createElement("th", {}, text(x));
}, ofArray(["selected", "id", "name", "universe", "address", "channel count"])))));
export function viewPatchTable(dispatch, patches, selectedId) {
  return createElement("table", {}, createElement.apply(undefined, ["tbody", {}].concat(_toConsumableArray(toList(delay(function () {
    return append(singleton(patchTableHeader), delay(function () {
      return map_1(function (patch) {
        return viewPatchTableRow(dispatch, selectedId, patch);
      }, patches);
    }));
  }))))));
}
export function viewConsole(dispatch, lines) {
  return createElement("div", {}, createElement("span", {}, text("Console")), createElement("div", {}, createElement("textarea", {
    style: {
      overflow: "scroll"
    },
    value: join("\n", lines)
  }), createElement("button", {
    onClick: function onClick(_arg1) {
      dispatch(new Message("Action", [new UiAction("ClearConsole", [])]));
    }
  }, text("clear"))));
}
export function view(model, dispatch) {
  return createElement("div", {}, viewPatchTable(dispatch, model.patches, model.selected), viewConsole(dispatch, model.consoleText));
}
ProgramModule.run(withReact("app", ProgramModule.mkProgram(function () {
  return initialModel();
}, function (message) {
  return function (model) {
    return updateAndLog(message, model);
  };
}, function (model_1) {
  return function (dispatch) {
    return view(model_1, dispatch);
  };
})));