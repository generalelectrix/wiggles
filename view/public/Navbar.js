var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _defineProperty(obj, key, value) { if (key in obj) { Object.defineProperty(obj, key, { value: value, enumerable: true, configurable: true, writable: true }); } else { obj[key] = value; } return obj; }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { equals, compareUnions, equalsUnions, makeGeneric, GenericParam } from "fable-core/Util";
import { mapIndexed, map } from "fable-core/List";
import List from "fable-core/List";
import { createElement } from "react";
export var Item = function () {
  function Item(text, onClick) {
    _classCallCheck(this, Item);

    this.text = text;
    this.onClick = onClick;
  }

  _createClass(Item, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.Item",
        interfaces: ["FSharpRecord"],
        properties: {
          text: "string",
          onClick: "function"
        }
      };
    }
  }]);

  return Item;
}();
setType("Navbar.Item", Item);
export var DropdownItem = function () {
  function DropdownItem(caseName, fields) {
    _classCallCheck(this, DropdownItem);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(DropdownItem, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.DropdownItem",
        interfaces: ["FSharpUnion"],
        cases: {
          Selection: [makeGeneric(Item, {
            msg: GenericParam("msg")
          })],
          Separator: []
        }
      };
    }
  }]);

  return DropdownItem;
}();
setType("Navbar.DropdownItem", DropdownItem);
export var DropdownModel = function () {
  function DropdownModel(text, items, isOpen) {
    _classCallCheck(this, DropdownModel);

    this.text = text;
    this.items = items;
    this.isOpen = isOpen;
  }

  _createClass(DropdownModel, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.DropdownModel",
        interfaces: ["FSharpRecord"],
        properties: {
          text: "string",
          items: makeGeneric(List, {
            T: makeGeneric(DropdownItem, {
              msg: GenericParam("msg")
            })
          }),
          isOpen: "boolean"
        }
      };
    }
  }]);

  return DropdownModel;
}();
setType("Navbar.DropdownModel", DropdownModel);
export var NavItem = function () {
  function NavItem(caseName, fields) {
    _classCallCheck(this, NavItem);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(NavItem, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.NavItem",
        interfaces: ["FSharpUnion"],
        cases: {
          Dropdown: [makeGeneric(DropdownModel, {
            msg: GenericParam("msg")
          })],
          Single: [makeGeneric(Item, {
            msg: GenericParam("msg")
          })]
        }
      };
    }
  }]);

  return NavItem;
}();
setType("Navbar.NavItem", NavItem);
export var Position = function () {
  function Position(caseName, fields) {
    _classCallCheck(this, Position);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(Position, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.Position",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Left: ["number"],
          Right: ["number"]
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

  return Position;
}();
setType("Navbar.Position", Position);
export var Model = function () {
  function Model(leftItems, rightItems, activeItem) {
    _classCallCheck(this, Model);

    this.leftItems = leftItems;
    this.rightItems = rightItems;
    this.activeItem = activeItem;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.Model",
        interfaces: ["FSharpRecord"],
        properties: {
          leftItems: makeGeneric(List, {
            T: makeGeneric(NavItem, {
              msg: GenericParam("msg")
            })
          }),
          rightItems: makeGeneric(List, {
            T: makeGeneric(NavItem, {
              msg: GenericParam("msg")
            })
          }),
          activeItem: Position
        }
      };
    }
  }]);

  return Model;
}();
setType("Navbar.Model", Model);
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
        type: "Navbar.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          CloseDropdown: [Position],
          OpenDropdown: [Position],
          SetActive: [Position],
          ToggleDropdown: [Position]
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
setType("Navbar.Message", Message);

function viewSingle(active, position, model, dispatch, dispatchLocal) {
  var onClick = function onClick(e) {
    model.onClick(dispatch);
    dispatchLocal(new Message("SetActive", [position]));
  };

  return createElement("li", active ? {
    className: "active"
  } : {}, createElement("a", {
    href: "#",
    onClick: onClick
  }, model.text));
}

function viewItemAsDropdownEntry(position, dispatch, dispatchLocal, model) {
  var onClick = function onClick(e) {
    model.onClick(dispatch);
    dispatchLocal(new Message("CloseDropdown", [position]));
  };

  return createElement("li", {}, createElement("a", {
    href: "#",
    onClick: onClick
  }, model.text));
}

export function viewDropdownItem(position, dispatch, dispatchLocal, model) {
  if (model.Case === "Separator") {
    return createElement("li", {
      role: "separator",
      className: "divider"
    });
  } else {
    return viewItemAsDropdownEntry(position, dispatch, dispatchLocal, model.Fields[0]);
  }
}

function viewDropdown(position, model, dispatch, dispatchLocal) {
  var _createElement;

  var dropdownItem = createElement("a", (_createElement = {
    href: "#",
    className: "dropdown-toggle",
    role: "button"
  }, _defineProperty(_createElement, "aria-haspopup", true), _defineProperty(_createElement, "aria-expanded", model.isOpen), _defineProperty(_createElement, "onClick", function (_arg1) {
    dispatchLocal(new Message("ToggleDropdown", [position]));
  }), _createElement), model.text, createElement("span", {
    className: "caret"
  }));
  var subitems = map(function (model_1) {
    return viewDropdownItem(position, dispatch, dispatchLocal, model_1);
  }, model.items);
  return createElement("li", {
    className: model.isOpen ? "dropdown open" : "dropdown"
  }, createElement.apply(undefined, ["ul", {
    className: "dropdown-menu",
    onBlur: function onBlur(_arg2) {
      dispatchLocal(new Message("CloseDropdown", [position]));
    }
  }].concat(_toConsumableArray(subitems))));
}

function viewNavSection(leftSide, active, items, dispatch, dispatchLocal) {
  var viewItem = function viewItem(index) {
    return function (item) {
      var position = leftSide ? new Position("Left", [index]) : new Position("Right", [index]);

      if (item.Case === "Dropdown") {
        return viewDropdown(position, item.Fields[0], dispatch, dispatchLocal);
      } else {
        var isActive = equals(index, active);
        return viewSingle(isActive, position, item.Fields[0], dispatch, dispatchLocal);
      }
    };
  };

  return createElement.apply(undefined, ["ul", {
    className: leftSide ? "nav navbar-nav" : "nav navbar-nav navbar-right"
  }].concat(_toConsumableArray(function (list) {
    return mapIndexed(function ($var77, $var78) {
      return viewItem($var77)($var78);
    }, list);
  }(items))));
}

export function view(model, dispatch, dispatchLocal) {
  var patternInput = model.activeItem.Case === "Right" ? [null, model.activeItem.Fields[0]] : [model.activeItem.Fields[0], null];
  var divLeftRight = createElement("div", {}, viewNavSection(true, patternInput[0], model.leftItems, dispatch, dispatchLocal), viewNavSection(false, patternInput[1], model.rightItems, dispatch, dispatchLocal));
  return createElement("nav", {
    className: "navbar navbar-default navbar-fixed-top"
  }, createElement("div", {
    className: "container"
  }, divLeftRight));
}