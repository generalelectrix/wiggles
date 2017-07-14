var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { equals, toString, defaultArg, compareUnions, equalsUnions, Option, Tuple, makeGeneric, GenericParam, Array as _Array } from "fable-core/Util";
import { PatchServerRequest, PatchServerResponse, UnivWithPort, PatchItem } from "./PatchTypes";
import { view as view_2, update as update_1, initialModel as initialModel_1, Message as Message_2, Model as Model_1 } from "./PatchEdit";
import { view as view_3, update as update_2, initialModel as initialModel_2, Message as Message_1, Model as Model_2 } from "./NewPatch";
import { map, ofArray } from "fable-core/List";
import { CmdModule } from "fable-elmish/elmish";
import { map as map_1, singleton, append, delay, toList, fold, tryFind } from "fable-core/Seq";
import { sortInPlaceBy } from "fable-core/Array";
import { createElement } from "react";
import { Grid, Table } from "../core/Bootstrap";
import { view as view_1 } from "./Universes";
export var Model = function () {
  function Model(patches, universes, availablePorts, selected, editorModel, newPatchModel) {
    _classCallCheck(this, Model);

    this.patches = patches;
    this.universes = universes;
    this.availablePorts = availablePorts;
    this.selected = selected;
    this.editorModel = editorModel;
    this.newPatchModel = newPatchModel;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Patcher.Model",
        interfaces: ["FSharpRecord"],
        properties: {
          patches: _Array(makeGeneric(PatchItem, {
            s: GenericParam("s")
          })),
          universes: _Array(UnivWithPort),
          availablePorts: _Array(Tuple(["string", "string"])),
          selected: Option("number"),
          editorModel: makeGeneric(Model_1, {
            s: GenericParam("s")
          }),
          newPatchModel: Model_2
        }
      };
    }
  }]);

  return Model;
}();
setType("Patcher.Model", Model);
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
        type: "Patcher.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Create: [Message_1],
          Deselect: [],
          Edit: [makeGeneric(Message_2, {
            s: GenericParam("s")
          })],
          Response: [makeGeneric(PatchServerResponse, {
            s: GenericParam("s")
          })],
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

  return Message;
}();
setType("Patcher.Message", Message);
export function initCommands() {
  return ofArray([new PatchServerRequest("AvailablePorts", []), new PatchServerRequest("PatchState", []), new PatchServerRequest("GetKinds", [])]);
}
export function initialModel() {
  return new Model(new Array(0), new Array(0), new Array(0), null, initialModel_1(), initialModel_2());
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

function updateFromServerMessage(message, model) {
  if (message.Case === "NewPatches") {
    return [new Model(model.patches.concat(message.Fields[0]), model.universes, model.availablePorts, model.selected, model.editorModel, model.newPatchModel), CmdModule.none()];
  } else if (message.Case === "Update") {
    var newPatches = model.patches.map(function (existing) {
      return existing.id === message.Fields[0].id ? message.Fields[0] : existing;
    });
    return [new Model(newPatches, model.universes, model.availablePorts, model.selected, model.editorModel, model.newPatchModel), updateEditorState(newPatches, model.selected)];
  } else if (message.Case === "Remove") {
    var newPatches_1 = model.patches.filter(function (p) {
      return p.id !== message.Fields[0];
    });
    return [new Model(newPatches_1, model.universes, model.availablePorts, model.selected, model.editorModel, model.newPatchModel), updateEditorState(newPatches_1, model.selected)];
  } else if (message.Case === "Kinds") {
    return [model, CmdModule.ofMsg(new Message("Create", [new Message_1("UpdateKinds", [message.Fields[0]])]))];
  } else if (message.Case === "UpdateUniverse") {
    var found = false;
    var universes = model.universes.map(function (u) {
      if (u.universe === message.Fields[0].universe) {
        found = true;
        return message.Fields[0];
      } else {
        return u;
      }
    });
    var universes_1 = !found ? universes.concat([message.Fields[0]]) : universes;
    return [new Model(model.patches, universes_1, model.availablePorts, model.selected, model.editorModel, model.newPatchModel), CmdModule.none()];
  } else if (message.Case === "UniverseRemoved") {
    return [function () {
      var universes_2 = model.universes.filter(function (u_1) {
        return u_1.universe !== message.Fields[0];
      });
      return new Model(model.patches, universes_2, model.availablePorts, model.selected, model.editorModel, model.newPatchModel);
    }(), CmdModule.none()];
  } else if (message.Case === "AvailablePorts") {
    return [new Model(model.patches, model.universes, message.Fields[0], model.selected, model.editorModel, model.newPatchModel), CmdModule.none()];
  } else {
    sortInPlaceBy(function (p_1) {
      return p_1.id;
    }, message.Fields[0]);
    return [new Model(message.Fields[0], message.Fields[1], model.availablePorts, model.selected, model.editorModel, model.newPatchModel), updateEditorState(message.Fields[0], model.selected)];
  }
}

export function update(message, model) {
  if (message.Case === "SetSelected") {
    return [function () {
      var selected = message.Fields[0];
      return new Model(model.patches, model.universes, model.availablePorts, selected, model.editorModel, model.newPatchModel);
    }(), updateEditorState(model.patches, message.Fields[0])];
  } else if (message.Case === "Deselect") {
    return [function () {
      var selected_1 = null;
      return new Model(model.patches, model.universes, model.availablePorts, selected_1, model.editorModel, model.newPatchModel);
    }(), updateEditorState(model.patches, null)];
  } else if (message.Case === "Edit") {
    var patternInput = update_1(message.Fields[0], model.editorModel);
    return [new Model(model.patches, model.universes, model.availablePorts, model.selected, patternInput[0], model.newPatchModel), CmdModule.map(function (arg0) {
      return new Message("Edit", [arg0]);
    }, patternInput[1])];
  } else if (message.Case === "Create") {
    var patternInput_1 = update_2(message.Fields[0], model.newPatchModel);
    return [new Model(model.patches, model.universes, model.availablePorts, model.selected, model.editorModel, patternInput_1[0]), CmdModule.map(function (arg0_1) {
      return new Message("Create", [arg0_1]);
    }, patternInput_1[1])];
  } else {
    return updateFromServerMessage(message.Fields[0], model);
  }
}
export function viewPatchTableRow(dispatch, selectedId, item) {
  var td = function td(x) {
    return createElement("td", {}, toString(x));
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
    dispatch(new Message("SetSelected", [item.id]));
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

  return createElement("tr", rowAttrs, td(item.id), td(item.name), td(item.kind), td(patternInput[0]), td(patternInput[1]), td(item.channelCount));
}
export var patchTableHeader = createElement.apply(undefined, ["tr", {}].concat(_toConsumableArray(map(function (x) {
  return createElement("th", {}, x);
}, ofArray(["id", "name", "kind", "universe", "address", "channel count"])))));
export function viewPatchTable(dispatch, patches, selectedId) {
  return createElement("div", {}, createElement("h4", {}, "Fixtures"), createElement("table", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Table.Condensed]), createElement.apply(undefined, ["tbody", {}].concat(_toConsumableArray(toList(delay(function () {
    return append(singleton(patchTableHeader), delay(function () {
      return map_1(function (patch) {
        return viewPatchTableRow(dispatch, selectedId, patch);
      }, patches);
    }));
  })))))));
}
export function view(openModal, model, dispatch, dispatchServer) {
  return Grid.layout(ofArray([[8, ofArray([viewPatchTable(dispatch, model.patches, model.selected), view_1(model.universes, model.availablePorts, openModal, dispatchServer)])], [4, ofArray([Grid.fullRow(ofArray([view_2(model.editorModel, function ($var374) {
    return dispatch(function (arg0) {
      return new Message("Edit", [arg0]);
    }($var374));
  }, dispatchServer, openModal)])), Grid.fullRow(ofArray([view_3(model.newPatchModel, function ($var375) {
    return dispatch(function (arg0_1) {
      return new Message("Create", [arg0_1]);
    }($var375));
  }, dispatchServer)]))])]]));
}