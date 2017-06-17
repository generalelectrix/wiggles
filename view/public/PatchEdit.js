var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compareUnions, equalsUnions, makeGeneric, Option } from "fable-core/Util";
import { globalAddressFromOptionals, ServerRequest, parseUniverseId, parseDmxAddress, PatchItem } from "./Types";
import { view as view_1, update as update_1, initialModel as initialModel_1, Message as Message_1, Model as Model_1 } from "./EditBox";
import { OptionalModule, emptyIfNone, Optional } from "./Util";
import { Result } from "fable-elmish/result";
import { CmdModule } from "fable-elmish/elmish";
import { createElement } from "react";
import { fold } from "fable-core/Seq";
import { Grid, Form, Button } from "./Bootstrap";
import { fsFormat } from "fable-core/String";
import { confirm } from "./Modal";
import { ofArray } from "fable-core/List";
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
        interfaces: ["FSharpRecord"],
        properties: {
          selected: Option(PatchItem),
          nameEdit: makeGeneric(Model_1, {
            T: "string"
          }),
          addressEdit: makeGeneric(Model_1, {
            T: makeGeneric(Optional, {
              T: "number"
            })
          }),
          universeEdit: makeGeneric(Model_1, {
            T: makeGeneric(Optional, {
              T: "number"
            })
          })
        }
      };
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
          AddressEdit: [makeGeneric(Message_1, {
            T: makeGeneric(Optional, {
              T: "number"
            })
          })],
          NameEdit: [makeGeneric(Message_1, {
            T: "string"
          })],
          SetState: [Option(PatchItem)],
          UniverseEdit: [makeGeneric(Message_1, {
            T: makeGeneric(Optional, {
              T: "number"
            })
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
  return new Model(null, initialModel_1("Name:", function (s) {
    return new Result("Ok", [s]);
  }, "text"), initialModel_1("Address:", parseDmxAddress, "number"), initialModel_1("Universe:", parseUniverseId, "number"));
}
export function update(message, model) {
  var clear = function clear(submodel) {
    return update_1(new Message_1("Clear", []), submodel);
  };

  return function (m) {
    return [m, CmdModule.none()];
  }(message.Case === "NameEdit" ? function () {
    var nameEdit = update_1(message.Fields[0], model.nameEdit);
    return new Model(model.selected, nameEdit, model.addressEdit, model.universeEdit);
  }() : message.Case === "AddressEdit" ? function () {
    var addressEdit = update_1(message.Fields[0], model.addressEdit);
    return new Model(model.selected, model.nameEdit, addressEdit, model.universeEdit);
  }() : message.Case === "UniverseEdit" ? function () {
    var universeEdit = update_1(message.Fields[0], model.universeEdit);
    return new Model(model.selected, model.nameEdit, model.addressEdit, universeEdit);
  }() : function () {
    var clearBuffers = void 0;
    var matchValue = [model.selected, message.Fields[0]];
    var $var34 = matchValue[0] != null ? matchValue[1] != null ? [0, matchValue[0], matchValue[1]] : [1] : [1];

    switch ($var34[0]) {
      case 0:
        if ($var34[1].id !== $var34[2].id) {
          clearBuffers = true;
        } else {
          clearBuffers = false;
        }

        break;

      case 1:
        clearBuffers = true;
        break;
    }

    var updatedModel = new Model(message.Fields[0], model.nameEdit, model.addressEdit, model.universeEdit);

    if (clearBuffers) {
      var nameEdit_1 = clear(model.nameEdit);
      var addressEdit_1 = clear(model.addressEdit);
      var universeEdit_1 = clear(model.universeEdit);
      return new Model(updatedModel.selected, nameEdit_1, addressEdit_1, universeEdit_1);
    } else {
      return updatedModel;
    }
  }());
}

function nameEditOnKeyDown(fixtureId, dispatchLocal, dispatchServer, nameEditModel) {
  var clear = function clear() {
    dispatchLocal(new Message("NameEdit", [new Message_1("Clear", [])]));
  };

  return ["onKeyDown", function (event) {
    var matchValue = event.keyCode;

    switch (matchValue) {
      case 13:
        var $var35 = nameEditModel.value != null ? nameEditModel.value.Case === "Ok" ? [0, nameEditModel.value.Fields[0]] : [1] : [1];

        switch ($var35[0]) {
          case 0:
            clear(null);
            dispatchServer(new ServerRequest("Rename", [fixtureId, $var35[1]]));
            break;

          case 1:
            break;
        }

        break;

      case 27:
        clear(null);
        break;

      default:}
  }];
}

function nameEditBox(selected, model, dispatchLocal, dispatchServer) {
  var onKeyDown = function onKeyDown(nameEditModel) {
    return nameEditOnKeyDown(selected.id, dispatchLocal, dispatchServer, nameEditModel);
  };

  return view_1(onKeyDown, selected.name, model.nameEdit, function ($var36) {
    return dispatchLocal(function (arg0) {
      return new Message("NameEdit", [arg0]);
    }($var36));
  });
}

function addressEditor(selected, model, dispatchLocal, dispatchServer, openModal) {
  var universeBox = view_1(null, emptyIfNone(selected.universe), model.universeEdit, function ($var37) {
    return dispatchLocal(function (arg0) {
      return new Message("UniverseEdit", [arg0]);
    }($var37));
  });
  var addressBox = view_1(null, emptyIfNone(selected.dmxAddress), model.addressEdit, function ($var38) {
    return dispatchLocal(function (arg0_1) {
      return new Message("AddressEdit", [arg0_1]);
    }($var38));
  });

  var clear = function clear(msg) {
    dispatchLocal(msg(new Message_1("Clear", [])));
  };

  var clearAll = function clearAll() {
    clear(function (arg0_2) {
      return new Message("UniverseEdit", [arg0_2]);
    });
    clear(function (arg0_3) {
      return new Message("AddressEdit", [arg0_3]);
    });
  };

  var handleRepatchButtonClick = function handleRepatchButtonClick(_arg1) {
    if (!model.addressEdit.IsOk ? true : !model.universeEdit.IsOk) {} else {
      var univ = model.universeEdit.ParsedValueOr(OptionalModule.ofOption(selected.universe));
      var addr = model.addressEdit.ParsedValueOr(OptionalModule.ofOption(selected.dmxAddress));
      var matchValue = globalAddressFromOptionals(univ, addr);

      if (matchValue.Case === "Ok") {
        dispatchServer(new ServerRequest("Repatch", [selected.id, matchValue.Fields[0]]));
        clearAll(null);
      }
    }
  };

  var repatchButton = createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", handleRepatchButtonClick], Button.Warning]), "Repatch");
  var removeButton = createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", function (e_1) {
    e_1.currentTarget.blur();
    var confirmMessage_1 = fsFormat("Are you sure you want to delete fixture %d (%s)?")(function (x) {
      return x;
    })(selected.id)(selected.name);

    var removeAction_1 = function removeAction_1(_arg2_1) {
      dispatchServer(new ServerRequest("Remove", [selected.id]));
    };

    openModal(confirm(confirmMessage_1, removeAction_1));
  }], Button.Danger]), "Remove");
  return createElement("div", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Form.Group]), universeBox, addressBox, repatchButton, removeButton);
}

export function view(model, dispatchLocal, dispatchServer, openModal) {
  var header = createElement("h3", {}, "Edit patch");
  var editor = model.selected != null ? createElement("div", {}, Grid.layout(ofArray([[3, ofArray([fsFormat("Id: %d")(function (x) {
    return x;
  })(model.selected.id)])], [9, ofArray([fsFormat("Type: %s")(function (x) {
    return x;
  })(model.selected.kind)])]])), nameEditBox(model.selected, model, dispatchLocal, dispatchServer), addressEditor(model.selected, model, dispatchLocal, dispatchServer, openModal)) : fsFormat("No fixture selected.")(function (x) {
    return x;
  });
  return createElement("div", {}, header, editor);
}