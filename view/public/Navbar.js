var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { Array as _Array, makeGeneric, GenericParam, compareRecords, equalsRecords, compareUnions, equalsUnions } from "fable-core/Util";
import { createElement } from "react";
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
          Left: [],
          Right: []
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
export var ItemId = function () {
  function ItemId(caseName, fields) {
    _classCallCheck(this, ItemId);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ItemId, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.ItemId",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Dropdown: ["number", "number"],
          Single: ["number"]
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

  return ItemId;
}();
setType("Navbar.ItemId", ItemId);
export var ItemAddress = function () {
  function ItemAddress(position, id) {
    _classCallCheck(this, ItemAddress);

    this.position = position;
    this.id = id;
  }

  _createClass(ItemAddress, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.ItemAddress",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          position: Position,
          id: ItemId
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

  return ItemAddress;
}();
setType("Navbar.ItemAddress", ItemAddress);
export var ItemState = function () {
  function ItemState(caseName, fields) {
    _classCallCheck(this, ItemState);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ItemState, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.ItemState",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Active: [],
          AlwaysInactive: [],
          Inactive: []
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

  return ItemState;
}();
setType("Navbar.ItemState", ItemState);
export var Item = function () {
  function Item(text, onClick, state) {
    _classCallCheck(this, Item);

    this.text = text;
    this.onClick = onClick;
    this.state = state;
  }

  _createClass(Item, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.Item",
        interfaces: ["FSharpRecord"],
        properties: {
          text: "string",
          onClick: "function",
          state: ItemState
        }
      };
    }
  }, {
    key: "active",
    get: function () {
      return this.state.Equals(new ItemState("Active", []));
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
  function DropdownModel(items, dropped) {
    _classCallCheck(this, DropdownModel);

    this.items = items;
    this.dropped = dropped;
  }

  _createClass(DropdownModel, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.DropdownModel",
        interfaces: ["FSharpRecord"],
        properties: {
          items: _Array(makeGeneric(DropdownItem, {
            msg: GenericParam("msg")
          })),
          dropped: "boolean"
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
export var Model = function () {
  function Model(leftItems, rightItems) {
    _classCallCheck(this, Model);

    this.leftItems = leftItems;
    this.rightItems = rightItems;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Navbar.Model",
        interfaces: ["FSharpRecord"],
        properties: {
          leftItems: _Array(makeGeneric(NavItem, {
            msg: GenericParam("msg")
          })),
          rightItems: _Array(makeGeneric(NavItem, {
            msg: GenericParam("msg")
          }))
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
          SetActive: [ItemAddress, "boolean"],
          SetDropped: [ItemAddress, "boolean"]
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
export function sendMessageSetState(address, model, dispatch) {
  model.onClick(dispatch);
  var $var77 = model.state.Case === "Active" ? [0] : model.state.Case === "Inactive" ? [0] : [1];

  switch ($var77[0]) {
    case 0:
      dispatch(new Message("SetActive", [address, true]));
      break;

    case 1:
      break;
  }
}
export function viewSingle(address, model, dispatch) {
  var onClick = function onClick(e) {
    sendMessageSetState(address, model, dispatch);
  };

  return createElement("li", model.active ? {
    className: "active"
  } : {}, createElement("a", {
    href: "#",
    onClick: onClick
  }, model.text));
}
export function viewDropItem(model, dispatch) {
  var onClick = ["onClick", function (_arg1) {
    model.onClick(dispatch);
  }];
}
export function viewDropdown(model) {}
export function view(model, dispatch) {}