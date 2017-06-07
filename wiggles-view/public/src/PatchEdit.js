var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compareRecords, equalsRecords, makeGeneric, Option, compareUnions, equalsUnions, GenericParam } from "fable-core/Util";
import { ServerRequest, PatchItem } from "./Types";
import { CmdModule } from "fable-elmish/elmish";
import { createElement } from "react";
import { fold } from "fable-core/Seq";
import { Grid, Button, Form } from "./Bootstrap";
import { ofArray } from "fable-core/List";
import { fsFormat } from "fable-core/String";
export function text(x) {
  return x;
}
export var EditField = function () {
  function EditField(caseName, fields) {
    _classCallCheck(this, EditField);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(EditField, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "PatchEdit.EditField",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Absent: [],
          Present: [GenericParam("T")]
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

  return EditField;
}();
setType("PatchEdit.EditField", EditField);
export var Model = function () {
  function Model(selected, nameEdit, addressEdit, universeEdit) {
    _classCallCheck(this, Model);

    this.selected = selected;
    this.nameEdit = nameEdit;
    this.addressEdit = addressEdit;
    this.universeEdit = universeEdit;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "PatchEdit.Model",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          selected: Option(PatchItem),
          nameEdit: makeGeneric(EditField, {
            T: "string"
          }),
          addressEdit: makeGeneric(EditField, {
            T: Option("number")
          }),
          universeEdit: makeGeneric(EditField, {
            T: Option("number")
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
setType("PatchEdit.Model", Model);
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
        type: "PatchEdit.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          AddressEdit: [makeGeneric(EditField, {
            T: Option("number")
          })],
          NameEdit: [makeGeneric(EditField, {
            T: "string"
          })],
          SetState: [Option(PatchItem)],
          UniverseEdit: [makeGeneric(EditField, {
            T: Option("number")
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

  return Message;
}();
setType("PatchEdit.Message", Message);
export function initialModel() {
  return new Model(null, new EditField("Absent", []), new EditField("Absent", []), new EditField("Absent", []));
}
export function update(message, model) {
  return function (m) {
    return [m, CmdModule.none()];
  }(message.Case === "NameEdit" ? new Model(model.selected, message.Fields[0], model.addressEdit, model.universeEdit) : message.Case === "AddressEdit" ? new Model(model.selected, model.nameEdit, message.Fields[0], model.universeEdit) : message.Case === "UniverseEdit" ? new Model(model.selected, model.nameEdit, model.addressEdit, message.Fields[0]) : function () {
    var clearBuffers = void 0;
    var matchValue = [model.selected, message.Fields[0]];
    var $var4 = matchValue[0] != null ? matchValue[1] != null ? [0, matchValue[0], matchValue[1]] : [1] : [1];

    switch ($var4[0]) {
      case 0:
        if ($var4[1].id !== $var4[2].id) {
          clearBuffers = true;
        } else {
          clearBuffers = false;
        }

        break;

      case 1:
        clearBuffers = true;
        break;
    }

    return new Model(message.Fields[0], clearBuffers ? new EditField("Absent", []) : model.nameEdit, clearBuffers ? new EditField("Absent", []) : model.addressEdit, clearBuffers ? new EditField("Absent", []) : model.universeEdit);
  }());
}
export var EnterKey = 13;
export var EscapeKey = 27;
export function nameEditBox(fixtureId, name, dispatchLocal, dispatchServer) {
  var clear = function clear() {
    dispatchLocal(new Message("NameEdit", [new EditField("Absent", [])]));
  };

  return createElement("div", {}, text("Name:"), createElement("input", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", name], ["onKeyDown", function (e_2) {
    var matchValue_1 = e_2.keyCode;

    switch (matchValue_1) {
      case 13:
        clear(null);
        dispatchServer(new ServerRequest("Rename", [fixtureId, name]));
        break;

      case 27:
        clear(null);
        break;

      default:}
  }], ["onBlur", function (_arg1_1) {
    clear(null);
  }], ["onChange", function (e_3) {
    dispatchLocal(new Message("NameEdit", [new EditField("Present", [e_3.target.value])]));
  }], ["type", "text"], Form.Control])));
}
export function withDefault(value, editField) {
  if (editField.Case === "Absent") {
    return value;
  } else {
    return editField.Fields[0];
  }
}
export function addressPieceEditBox(label, cmd, addr, dispatchLocal) {
  var displayAddr = function (_arg1) {
    return _arg1 == null ? "" : String(_arg1);
  }(addr);

  return createElement("label", {}, text(label), createElement("input", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", displayAddr], ["onChange", function (e_1) {
    dispatchLocal(cmd(new EditField("Present", [function () {
      var matchValue_1 = e_1.target.value;

      if (matchValue_1 === "") {
        return null;
      } else {
        return matchValue_1;
      }
    }()])));
  }], ["type", "number"], Form.Control])));
}
export function addressEditor(selected, model, dispatchLocal, dispatchServer) {
  var displayUniv = withDefault(selected.universe, model.universeEdit);
  var displayAddr = withDefault(selected.dmxAddress, model.addressEdit);

  var clear = function clear(msg) {
    dispatchLocal(msg(new EditField("Absent", [])));
  };

  return createElement("div", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Form.Group]), addressPieceEditBox("Universe:", function (arg0) {
    return new Message("UniverseEdit", [arg0]);
  }, displayUniv, dispatchLocal), addressPieceEditBox("Address:", function (arg0_1) {
    return new Message("AddressEdit", [arg0_1]);
  }, displayAddr, dispatchLocal), createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", function (_arg1_1) {
    clear(function (arg0_4) {
      return new Message("UniverseEdit", [arg0_4]);
    });
    clear(function (arg0_5) {
      return new Message("AddressEdit", [arg0_5]);
    });
    var matchValue_1 = [displayUniv, displayAddr];
    var $var6 = matchValue_1[0] != null ? matchValue_1[1] != null ? [0, matchValue_1[1], matchValue_1[0]] : [1] : [1];

    switch ($var6[0]) {
      case 0:
        dispatchServer(new ServerRequest("Repatch", [selected.id, [$var6[2], $var6[1]]]));
        break;

      case 1:
        break;
    }
  }], Button.Warning]), text("Repatch")));
}
export function view(model, dispatchLocal, dispatchServer) {
  var header = createElement("h3", {}, text("Edit patch"));
  var editor = model.selected != null ? createElement("div", {}, Grid.layout(ofArray([[3, ofArray([text(fsFormat("Id: %d")(function (x) {
    return x;
  })(model.selected.id))])], [9, ofArray([text(fsFormat("Type: %s")(function (x) {
    return x;
  })(model.selected.kind))])]])), nameEditBox(model.selected.id, withDefault(model.selected.name, model.nameEdit), dispatchLocal, dispatchServer), addressEditor(model.selected, model, dispatchLocal, dispatchServer)) : text(fsFormat("No fixture selected.")(function (x) {
    return x;
  }));
  return createElement("div", {}, header, editor);
}