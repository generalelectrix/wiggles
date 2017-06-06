var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { defaultArg, compareUnions, equalsUnions, compareRecords, equalsRecords, Option } from "fable-core/Util";
import { ServerRequest, PatchItem } from "./Types";
import { CmdModule } from "fable-elmish/elmish";
import { createElement } from "react";
import { fsFormat } from "fable-core/String";
export function text(x) {
  return x;
}
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
          nameEdit: Option("string"),
          addressEdit: Option("number"),
          universeEdit: Option("number")
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
          AddressEdit: [Option("number")],
          NameEdit: [Option("string")],
          SetState: [Option(PatchItem)],
          UniverseEdit: [Option("number")]
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
  return new Model(null, null, null, null);
}
export function update(message, model) {
  return function (m) {
    return [m, CmdModule.none()];
  }(message.Case === "NameEdit" ? new Model(model.selected, message.Fields[0], model.addressEdit, model.universeEdit) : message.Case === "AddressEdit" ? new Model(model.selected, model.nameEdit, message.Fields[0], model.universeEdit) : message.Case === "UniverseEdit" ? new Model(model.selected, model.nameEdit, model.addressEdit, message.Fields[0]) : function () {
    var clearBuffers = void 0;
    var matchValue = [model.selected, message.Fields[0]];
    var $var1 = matchValue[0] != null ? matchValue[1] != null ? [0, matchValue[0], matchValue[1]] : [1] : [1];

    switch ($var1[0]) {
      case 0:
        if ($var1[1].id !== $var1[2].id) {
          clearBuffers = true;
        } else {
          clearBuffers = false;
        }

        break;

      case 1:
        clearBuffers = true;
        break;
    }

    return new Model(message.Fields[0], clearBuffers ? null : model.nameEdit, clearBuffers ? null : model.addressEdit, model.universeEdit);
  }());
}
export var enterKeyCode = 13;
export function nameEditBox(fixtureId, name, dispatchLocal, dispatchServer) {
  var clearNameEdit = function clearNameEdit() {
    dispatchLocal(new Message("NameEdit", [null]));
  };

  return createElement("input", {
    type: "text",
    onChange: function onChange(e_1) {
      dispatchLocal(new Message("NameEdit", [e_1.target.value]));
    },
    onBlur: function onBlur(_arg1) {
      clearNameEdit(null);
    },
    onKeyDown: function onKeyDown(e) {
      if (e.keyCode === enterKeyCode) {
        clearNameEdit(null);
        dispatchServer(new ServerRequest("Rename", [fixtureId, name]));
      }
    },
    value: name
  });
}
export function view(model, dispatchLocal, dispatchServer) {
  if (model.selected != null) {
    return createElement("div", {}, createElement("div", {}, text(fsFormat("Fixture id: %d")(function (x) {
      return x;
    })(model.selected.id))), createElement("div", {}, text(fsFormat("Fixture type: %s")(function (x) {
      return x;
    })(model.selected.kind))), nameEditBox(model.selected.id, defaultArg(model.nameEdit, model.selected.name), dispatchLocal, dispatchServer));
  } else {
    return createElement("div", {}, text(fsFormat("No fixture selected.")(function (x) {
      return x;
    })));
  }
}