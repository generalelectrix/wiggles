function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

import { ofArray, fold, collect, singleton, append, toList, range, map, delay } from "fable-core/Seq";
import { createElement } from "react";
import { ofJson, toJson } from "fable-core/Serialize";
import { fsFormat } from "fable-core/String";
import { Tuple, Option } from "fable-core/Util";
import { all } from "../core/Types";
import { PatchServerRequest } from "./PatchTypes";
import { Form } from "../core/Bootstrap";
import { mapIndexed } from "fable-core/List";

function sourceSelector(fixtureId, controlId, wiggles, selected, dispatchServer) {
  var options = function options(tupledArg) {
    return delay(function () {
      return map(function (outputId) {
        return createElement("option", {
          value: toJson([tupledArg[0], outputId])
        }, fsFormat("%s (output %d)")(function (x) {
          return x;
        })(tupledArg[1].name)(outputId));
      }, range(0, tupledArg[1].outputs - 1));
    });
  };

  var options_1 = toList(delay(function () {
    return append(singleton(createElement("option", {
      value: toJson(null)
    }, "{disconnected}")), delay(function () {
      return collect(function (wiggle) {
        return options(wiggle);
      }, wiggles);
    }));
  }));
  return createElement("div", {}, createElement.apply(undefined, ["select", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", toJson(selected)], ["onChange", function (e_1) {
    var selected_2 = ofJson(e_1.target.value, {
      T: Option(Tuple([Tuple(["number", "number"]), "number"]))
    });
    dispatchServer(all(function (tupledArg_2) {
      return new PatchServerRequest("SetControlSource", [tupledArg_2[0], tupledArg_2[1], tupledArg_2[2]]);
    }([fixtureId, controlId, selected_2])));
  }], Form.Control])].concat(_toConsumableArray(options_1))));
}

function viewSources(item, wiggles, dispatchServer) {
  return createElement.apply(undefined, ["div", {}].concat(_toConsumableArray(mapIndexed(function (controlId, control) {
    return createElement("div", {}, fsFormat("%s (%O)")(function (x) {
      return x;
    })(control.name)(control.dataType), sourceSelector(item.id, controlId, wiggles, control.source, dispatchServer));
  }, item.controlSources))));
}

function viewPatchItem(wiggles, dispatchServer, item) {
  return createElement("div", {}, createElement("h4", {}, fsFormat("%s (fixture %d)")(function (x) {
    return x;
  })(item.name)(item.id)), viewSources(item, wiggles, dispatchServer));
}

export function view(patches, wiggles, dispatchServer) {
  return createElement.apply(undefined, ["div", {}].concat(_toConsumableArray(toList(map(function (item) {
    return viewPatchItem(wiggles, dispatchServer, item);
  }, ofArray(patches))))));
}