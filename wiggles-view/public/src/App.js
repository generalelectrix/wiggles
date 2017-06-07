var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { equals, defaultArg, compareUnions, equalsUnions, compareRecords, equalsRecords, Array as _Array, Option, makeGeneric } from "fable-core/Util";
import { filter, map, ofArray } from "fable-core/List";
import List from "fable-core/List";
import { testPatches, testKinds, Cmd, ServerResponse, ServerRequest, PatchItem } from "./Types";
import { view as view_1, update as update_1, initialModel as initialModel_1, Message as Message_2, Model as Model_1 } from "./PatchEdit";
import { view as view_2, update as update_2, initialModel as initialModel_2, Message as Message_1, Model as Model_2 } from "./NewPatch";
import { ProgramModule, CmdModule } from "fable-elmish/elmish";
import { join, fsFormat } from "fable-core/String";
import { map as map_1, singleton, append, delay, toList, fold, tryFind } from "fable-core/Seq";
import { createElement } from "react";
import { Grid, Container, Button, Form, Table } from "./Bootstrap";
import { withReact } from "fable-elmish-react/react";
export function text(x) {
  return x;
}
export var Model = function () {
  function Model(patches, selected, editorModel, newPatchModel, consoleText) {
    _classCallCheck(this, Model);

    this.patches = patches;
    this.selected = selected;
    this.editorModel = editorModel;
    this.newPatchModel = newPatchModel;
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
          editorModel: Model_1,
          newPatchModel: Model_2,
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
  return new Model(model.patches, model.selected, model.editorModel, model.newPatchModel, consoleText);
}
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
          Create: [Message_1],
          Edit: [Message_2],
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
export function initialModel() {
  var m = new Model(new List(), null, initialModel_1(), initialModel_2(), new Array(0));
  var initCommands = CmdModule.map(function (arg0) {
    return new Message("Request", [arg0]);
  }, Cmd.ofMsgs(ofArray([new ServerRequest("PatchState", []), new ServerRequest("GetKinds", [])])));
  return [m, initCommands];
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
        }, model.patches), null, function ($var33) {
          return msgType(op($var33));
        }));
      };
    };
  };

  if (req.Case === "GetKinds") {
    return new ServerResponse("Kinds", [testKinds]);
  } else if (req.Case === "NewPatch") {
    return new ServerResponse("NewPatch", [req.Fields[0]]);
  } else if (req.Case === "Rename") {
    return maybeUpdatePatch(function (arg0) {
      return new ServerResponse("Update", [arg0]);
    })(function (p_1) {
      return new PatchItem(p_1.id, req.Fields[1], p_1.kind, p_1.address, p_1.channelCount);
    })(req.Fields[0]);
  } else if (req.Case === "Repatch") {
    return maybeUpdatePatch(function (arg0_1) {
      return new ServerResponse("Update", [arg0_1]);
    })(function (p_2) {
      return new PatchItem(p_2.id, p_2.name, p_2.kind, req.Fields[1], p_2.channelCount);
    })(req.Fields[0]);
  } else if (req.Case === "Remove") {
    return maybeUpdatePatch(function (arg0_2) {
      return new ServerResponse("Remove", [arg0_2]);
    })(function (_arg1_1) {
      return req.Fields[0];
    })(req.Fields[0]);
  } else {
    return new ServerResponse("PatchState", [model.patches.tail == null ? testPatches : model.patches]);
  }
}
export function updateEditorState(patches, selectedFixtureId) {
  return CmdModule.ofMsg(new Message("Edit", [new Message_2("SetState", [function (_arg1) {
    return _arg1 != null ? _arg1 : null;
  }(defaultArg(selectedFixtureId, null, function (fixtureId) {
    return tryFind(function (p) {
      return p.id === fixtureId;
    }, patches);
  }))])]));
}
export function update(message, model) {
  if (message.Case === "Response") {
    if (message.Fields[0].Case === "PatchState") {
      return [new Model(message.Fields[0].Fields[0], model.selected, model.editorModel, model.newPatchModel, model.consoleText), updateEditorState(message.Fields[0].Fields[0], model.selected)];
    } else if (message.Fields[0].Case === "NewPatch") {
      return [new Model(new List(message.Fields[0].Fields[0], model.patches), model.selected, model.editorModel, model.newPatchModel, model.consoleText), CmdModule.none()];
    } else if (message.Fields[0].Case === "Update") {
      var newPatches = map(function (existing) {
        return existing.id === message.Fields[0].Fields[0].id ? message.Fields[0].Fields[0] : existing;
      }, model.patches);
      return [new Model(newPatches, model.selected, model.editorModel, model.newPatchModel, model.consoleText), updateEditorState(newPatches, model.selected)];
    } else if (message.Fields[0].Case === "Remove") {
      var newPatches_1 = filter(function (p) {
        return p.id === message.Fields[0].Fields[0];
      }, model.patches);
      return [new Model(newPatches_1, model.selected, model.editorModel, model.newPatchModel, model.consoleText), updateEditorState(newPatches_1, model.selected)];
    } else if (message.Fields[0].Case === "Kinds") {
      return [model, CmdModule.ofMsg(new Message("Create", [new Message_1("UpdateKinds", [message.Fields[0].Fields[0]])]))];
    } else {
      return [function (model_1) {
        return withConsoleMessage(message.Fields[0].Fields[0], model_1);
      }(model), CmdModule.none()];
    }
  } else if (message.Case === "Action") {
    if (message.Fields[0].Case === "SetSelected") {
      return [function () {
        var selected = message.Fields[0].Fields[0];
        return new Model(model.patches, selected, model.editorModel, model.newPatchModel, model.consoleText);
      }(), updateEditorState(model.patches, message.Fields[0].Fields[0])];
    } else if (message.Fields[0].Case === "Deselect") {
      return [function () {
        var selected_1 = null;
        return new Model(model.patches, selected_1, model.editorModel, model.newPatchModel, model.consoleText);
      }(), updateEditorState(model.patches, null)];
    } else {
      return [function () {
        var consoleText = new Array(0);
        return new Model(model.patches, model.selected, model.editorModel, model.newPatchModel, consoleText);
      }(), CmdModule.none()];
    }
  } else if (message.Case === "Edit") {
    var patternInput = update_1(message.Fields[0], model.editorModel);
    return [new Model(model.patches, model.selected, patternInput[0], model.newPatchModel, model.consoleText), CmdModule.map(function (arg0) {
      return new Message("Edit", [arg0]);
    }, patternInput[1])];
  } else if (message.Case === "Create") {
    var patternInput_1 = update_2(message.Fields[0], model.newPatchModel);
    return [new Model(model.patches, model.selected, model.editorModel, patternInput_1[0], model.consoleText), CmdModule.map(function (arg0_1) {
      return new Message("Create", [arg0_1]);
    }, patternInput_1[1])];
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

  var rowAttrs = void 0;
  var onClick = ["onClick", function (_arg1) {
    dispatch(new Message("Action", [new UiAction("SetSelected", [item.id])]));
  }];

  if (equals(item.id, selectedId)) {
    rowAttrs = fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [Table.Row.Active, onClick]);
  } else {
    rowAttrs = fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [onClick]);
  }

  return createElement("tr", rowAttrs, td(item.id), td(item.kind), td(item.name), td(patternInput[0]), td(patternInput[1]), td(item.channelCount));
}
export var patchTableHeader = createElement.apply(undefined, ["tr", {}].concat(_toConsumableArray(map(function (x) {
  return createElement("th", {}, text(x));
}, ofArray(["id", "kind", "name", "universe", "address", "channel count"])))));
export function viewPatchTable(dispatch, patches, selectedId) {
  return createElement("table", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Table.Condensed]), createElement.apply(undefined, ["tbody", {}].concat(_toConsumableArray(toList(delay(function () {
    return append(singleton(patchTableHeader), delay(function () {
      return map_1(function (patch) {
        return viewPatchTableRow(dispatch, selectedId, patch);
      }, patches);
    }));
  }))))));
}
export function viewConsole(dispatch, lines) {
  return createElement("div", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Form.Group]), createElement("span", {}, text("Console"), createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", function (_arg1_1) {
    dispatch(new Message("Action", [new UiAction("ClearConsole", [])]));
  }], Button.Warning]), text("clear"))), createElement("div", {}, createElement("textarea", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["cols", 80], ["rows", 20], ["value", join("\n", lines)], ["style", {
    overflow: "scroll"
  }], Form.Control]))));
}
export function view(model, dispatch) {
  var dispatchServer = function dispatchServer($var34) {
    return dispatch(function (arg0) {
      return new Message("Request", [arg0]);
    }($var34));
  };

  return createElement("div", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Container.Fluid]), Grid.layout(ofArray([[8, ofArray([viewPatchTable(dispatch, model.patches, model.selected)])], [4, ofArray([Grid.fullRow(ofArray([view_1(model.editorModel, function ($var35) {
    return dispatch(function (arg0_1) {
      return new Message("Edit", [arg0_1]);
    }($var35));
  }, dispatchServer)])), Grid.fullRow(ofArray([view_2(model.newPatchModel, function ($var36) {
    return dispatch(function (arg0_2) {
      return new Message("Create", [arg0_2]);
    }($var36));
  }, dispatchServer)]))])]])), Grid.fullRow(ofArray([viewConsole(dispatch, model.consoleText)])));
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