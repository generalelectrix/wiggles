function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

import { createElement } from "react";
import { map as map_1, toList, mapIndexed, tryItem, tryFindIndex, fold } from "fable-core/Seq";
import { all } from "../core/Types";
import { UnivWithPort, PatchServerRequest } from "./PatchTypes";
import { Table, Form, Button } from "../core/Bootstrap";
import { confirm } from "../core/Modal";
import { fsFormat } from "fable-core/String";
import { parseInt, logError, withDefault } from "../core/Util";
import { defaultArg } from "fable-core/Util";
import { map, singleton, ofArray } from "fable-core/List";

function addButton(dispatchServer) {
  return createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", function (_arg1_1) {
    dispatchServer(all(new PatchServerRequest("AddUniverse", [])));
  }], Button.Primary]), "Add universe");
}

function refreshPortsButton(dispatchServer) {
  return createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", function (_arg1_1) {
    dispatchServer(all(new PatchServerRequest("AvailablePorts", [])));
  }], Button.Default]), "Refresh ports");
}

function deleteButton(universeId, openModal, dispatchServer) {
  return createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", function (_arg2_1) {
    openModal(confirm(fsFormat("Are you sure you want to delete universe %d?")(function (x) {
      return x;
    })(universeId), function (_arg1_1) {
      dispatchServer(all(new PatchServerRequest("RemoveUniverse", [universeId, false])));
    }));
  }], Button.Default]), "Delete");
}

function portSelector(universe, ports, dispatchServer) {
  var portOption = function portOption(portIndex) {
    return function (_arg1) {
      return createElement("option", {
        value: String(portIndex)
      }, fsFormat("%s: %s")(function (x) {
        return x;
      })(_arg1[0])(_arg1[1]));
    };
  };

  var selected = withDefault("", defaultArg(tryFindIndex(function (tupledArg) {
    return universe.portNamespace === tupledArg[0] ? universe.portId === tupledArg[1] : false;
  }, ports), null, function (value) {
    return String(value);
  }));

  if (selected === "") {
    logError(fsFormat("Could not find port entry for universe %d (%s: %s).")(function (x) {
      return x;
    })(universe.universe)(universe.portNamespace)(universe.portId));
  }

  return createElement("div", {}, createElement.apply(undefined, ["select", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", selected], ["onChange", function (e_1) {
    var selectedValue_1 = e_1.target.value;
    var matchValue_2 = parseInt(selectedValue_1);

    if (matchValue_2 != null) {
      var matchValue_3 = function (array_1) {
        return tryItem(matchValue_2, array_1);
      }(ports);

      if (matchValue_3 != null) {
        var portNamespace_1 = matchValue_3[0];
        var portId_1 = matchValue_3[1];
        dispatchServer(all(new PatchServerRequest("AttachPort", [new UnivWithPort(universe.universe, portNamespace_1, portId_1)])));
      } else {
        logError(fsFormat("Port index %d is out of bounds.")(function (x) {
          return x;
        })(matchValue_2));
      }
    } else {
      logError(fsFormat("Could not reparse universe index: %s")(function (x) {
        return x;
      })(selectedValue_1));
    }
  }], Form.Control])].concat(_toConsumableArray(ofArray(function (array_2) {
    return Array.from(mapIndexed(function ($var303, $var304) {
      return portOption($var303)($var304);
    }, array_2));
  }(ports))))));
}

export var tableHeader = createElement.apply(undefined, ["thead", {}].concat(_toConsumableArray(singleton(createElement.apply(undefined, ["tr", {}].concat(_toConsumableArray(map(function (item) {
  return createElement("th", {}, item);
}, ofArray(["id", "port", ""])))))))));

function universeRow(ports, openModal, dispatchServer, universe) {
  var td = function td(item) {
    return createElement("td", {}, item);
  };

  return createElement("tr", {}, td(String(universe.universe)), td(portSelector(universe, ports, dispatchServer)), td(deleteButton(universe.universe, openModal, dispatchServer)));
}

function viewTable(ports, openModal, dispatchServer, universes) {
  return createElement("table", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Table.Condensed]), tableHeader, createElement.apply(undefined, ["tbody", {}].concat(_toConsumableArray(toList(map_1(function (universe) {
    return universeRow(ports, openModal, dispatchServer, universe);
  }, universes))))));
}

export function view(universes, ports, openModal, dispatchServer) {
  return createElement("div", {}, createElement("h4", {}, "Universes"), viewTable(ports, openModal, dispatchServer, universes), addButton(dispatchServer), refreshPortsButton(dispatchServer));
}