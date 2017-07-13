var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { view as view_1, update as update_1, initialModel, Model as Model_1 } from "./EditBox";
import { makeGeneric, GenericParam } from "fable-core/Util";
import { createElement } from "react";
import { fold } from "fable-core/Seq";
import { Grid, Button } from "./Bootstrap";
import { ofArray } from "fable-core/List";
export var Model = function () {
  function Model(editBox, okText) {
    _classCallCheck(this, Model);

    this.editBox = editBox;
    this.okText = okText;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "SimpleEditor.Model",
        interfaces: ["FSharpRecord"],
        properties: {
          editBox: makeGeneric(Model_1, {
            T: GenericParam("a")
          }),
          okText: "string"
        }
      };
    }
  }]);

  return Model;
}();
setType("SimpleEditor.Model", Model);
export function initModel(okText, label, parser, inputType) {
  return new Model(initialModel(label, parser, inputType), okText);
}
export function update(message, model) {
  return new Model(update_1(message, model.editBox), model.okText);
}

function okButton(model, onOk, onComplete) {
  var onClick = function onClick(_arg1) {
    var matchValue = model.editBox.value;
    var $var87 = matchValue != null ? matchValue.Case === "Ok" ? [0, matchValue.Fields[0]] : [1] : [1];

    switch ($var87[0]) {
      case 0:
        onOk($var87[1]);
        onComplete(null);
        break;

      case 1:
        break;
    }
  };

  return createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", onClick], Button.Primary]), model.okText);
}

export function cancelButton(onComplete) {
  var onClick = function onClick(_arg1) {
    onComplete(null);
  };

  return createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", onClick], Button.Default]), "Cancel");
}
export function view(defaultVal, model, onOk, onComplete, dispatch) {
  var okButton_1 = okButton(model, onOk, onComplete);
  return createElement("div", {}, Grid.fullRow(ofArray([view_1(null, defaultVal, model.editBox, dispatch)])), Grid.layout(ofArray([[1, ofArray([okButton_1])], [1, ofArray([cancelButton(onComplete)])]])));
}