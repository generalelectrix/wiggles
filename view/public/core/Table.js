var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { equals, compareUnions, equalsUnions, compareRecords, equalsRecords, Option, makeGeneric } from "fable-core/Util";
import { mapIndexed, map } from "fable-core/List";
import List from "fable-core/List";
import { fold } from "fable-core/Seq";
import { Table } from "./Bootstrap";
import { createElement } from "react";
export var Model = function () {
  function Model(height, header, selected) {
    _classCallCheck(this, Model);

    this.height = height;
    this.header = header;
    this.selected = selected;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Table.Model",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          height: "number",
          header: makeGeneric(List, {
            T: "string"
          }),
          selected: Option("number")
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
setType("Table.Model", Model);
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
        type: "Table.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Deselect: [],
          Select: ["number"]
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
setType("Table.Message", Message);
export function update(message, model) {
  if (message.Case === "Deselect") {
    var selected = null;
    return new Model(model.height, model.header, selected);
  } else {
    var selected_1 = message.Fields[0];
    return new Model(model.height, model.header, selected_1);
  }
}

function viewRow(dispatch, selected, index, row) {
  var rowAttrs = void 0;
  var onClick = ["onClick", function (_arg1) {
    dispatch(new Message("Select", [index]));
  }];

  if (equals(index, selected)) {
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

  var rowItems = map(function (x) {
    return createElement("td", {}, x);
  }, row.ToStrings());
  return createElement.apply(undefined, ["tr", rowAttrs].concat(_toConsumableArray(rowItems)));
}

function viewHeader(header) {
  return createElement.apply(undefined, ["tr", {}].concat(_toConsumableArray(map(function (x) {
    return createElement("th", {}, x);
  }, header))));
}

export function view(rows, model, dispatch) {
  var styles = {
    height: model.height,
    overflow: "scroll"
  };
  return createElement("div", {
    style: styles
  }, createElement("table", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Table.Condensed]), createElement("thead", {}, viewHeader(model.header)), createElement.apply(undefined, ["tbody", {}].concat(_toConsumableArray(mapIndexed(function (index, row) {
    return viewRow(dispatch, model.selected, index, row);
  }, rows))))));
}