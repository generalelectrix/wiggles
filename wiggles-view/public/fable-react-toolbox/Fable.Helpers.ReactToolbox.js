var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import * as commons_scss from "react-toolbox/lib/commons.scss";
import app_bar from "react-toolbox/lib/app_bar";
import autocomplete from "react-toolbox/lib/autocomplete";
import avatar from "react-toolbox/lib/avatar";
import { IconButton as IconButton_1, Button as Button_1 } from "react-toolbox/lib/button";
import { CardTitle as CardTitle_1, CardText as CardText_1, CardMedia as CardMedia_1, CardActions as CardActions_1, Card as Card_1 } from "react-toolbox/lib/card";
import checkbox from "react-toolbox/lib/checkbox";
import chip from "react-toolbox/lib/chip";
import date_picker from "react-toolbox/lib/date_picker";
import dialog from "react-toolbox/lib/dialog";
import drawer from "react-toolbox/lib/drawer";
import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compareUnions, equalsUnions, Any, compareRecords, equalsRecords, GenericParam } from "fable-core/Util";
import dropdown from "react-toolbox/lib/dropdown";
import font_icon from "react-toolbox/lib/font_icon";
import input from "react-toolbox/lib/input";
import { Sidebar as Sidebar_1, Panel as Panel_1, NavDrawer as NavDrawer_1, Layout as Layout_1 } from "react-toolbox/lib/layout";
import link from "react-toolbox/lib/link";
import { ListSubHeader as ListSubHeader_1, ListItem as ListItem_1, ListDivider as ListDivider_1, ListCheckbox as ListCheckbox_1, List as List_1 } from "react-toolbox/lib/list";
import { MenuItem as MenuItem_1, MenuDivider as MenuDivider_1, IconMenu as IconMenu_1, Menu as Menu_1 } from "react-toolbox/lib/menu";
import navigation from "react-toolbox/lib/navigation";
import progress_bar from "react-toolbox/lib/progress_bar";
import radio from "react-toolbox/lib/radio";
import ripple from "react-toolbox/lib/ripple";
import slider from "react-toolbox/lib/slider";
import snackbar from "react-toolbox/lib/snackbar";
import _switch from "react-toolbox/lib/switch";
import table from "react-toolbox/lib/table";
import { Tab as Tab_1, Tabs as Tabs_1 } from "react-toolbox/lib/tabs";
import time_picker from "react-toolbox/lib/time_picker";
import tooltip from "react-toolbox/lib/tooltip";
export var styles = commons_scss;
export var AppBar = app_bar;
export var Autocomplete = autocomplete;
export var Avatar = avatar;
export var Button = Button_1;
export var IconButton = IconButton_1;
export var Card = Card_1;
export var CardActions = CardActions_1;
export var CardMedia = CardMedia_1;
export var CardText = CardText_1;
export var CardTitle = CardTitle_1;
export var Checkbox = checkbox;
export var Chip = chip;
export var DatePicker = date_picker;
export var Dialog = dialog;
export var Drawer = drawer;
export var DropdownWrapper = function () {
  function DropdownWrapper(item, value) {
    _classCallCheck(this, DropdownWrapper);

    this.item = item;
    this.value = value;
  }

  _createClass(DropdownWrapper, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Fable.Helpers.ReactToolbox.DropdownWrapper",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          item: GenericParam("T"),
          value: GenericParam("TVal")
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

  return DropdownWrapper;
}();
setType("Fable.Helpers.ReactToolbox.DropdownWrapper", DropdownWrapper);
export var Dropdown = dropdown;
export var FontIcon = font_icon;
export var Input = input;
export var Layout = Layout_1;
export var NavDrawer = NavDrawer_1;
export var Panel = Panel_1;
export var Sidebar = Sidebar_1;
export var Link = link;
export var List = List_1;
export var ListCheckbox = ListCheckbox_1;
export var ListDividerProps = function () {
  function ListDividerProps(caseName, fields) {
    _classCallCheck(this, ListDividerProps);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ListDividerProps, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Fable.Helpers.ReactToolbox.ListDividerProps",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable", "Fable.Helpers.ReactToolbox.Props.IReactToolboxProp"],
        cases: {
          Inset: ["boolean"],
          Theme: [Any]
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

  return ListDividerProps;
}();
setType("Fable.Helpers.ReactToolbox.ListDividerProps", ListDividerProps);
export var ListDivider = ListDivider_1;
export var ListItem = ListItem_1;
export var ListSubHeader = ListSubHeader_1;
export var Menu = Menu_1;
export var IconMenu = IconMenu_1;
export var MenuDivider = MenuDivider_1;
export var MenuItem = MenuItem_1;
export var Navigation = navigation;
export var ProgressBar = progress_bar;
export var RadioGroup = radio;
export var RadioButton = radio;
export var Ripple = ripple;
export var Slider = slider;
export var Snackbar = snackbar;
export var Switch = _switch;
export var Table = table;
export var Tabs = Tabs_1;
export var Tab = Tab_1;
export var TimePicker = time_picker;
export var Tooltip = tooltip;