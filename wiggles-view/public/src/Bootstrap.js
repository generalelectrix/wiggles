function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

import { singleton, collect, delay, toList, fold } from "fable-core/Seq";
import { createElement } from "react";
import { fsFormat } from "fable-core/String";
import { map, ofArray } from "fable-core/List";

function cn(arg0) {
  return ["className", arg0];
}

function elementWithClass(elemFunc, cls, elems) {
  return elemFunc(fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [cn(cls)]))(elems);
}

export var Container = function (__exports) {
  var Fixed = __exports.Fixed = cn("container");
  var Fluid = __exports.Fluid = cn("container-fluid");
  return __exports;
}({});
export var Grid = function (__exports) {
  var divWithClass = function divWithClass(cls) {
    return function (elems) {
      return elementWithClass(function (b) {
        return function (c) {
          return createElement.apply(undefined, ["div", b].concat(_toConsumableArray(c)));
        };
      }, cls, elems);
    };
  };

  var row = __exports.row = divWithClass("row");

  var col = __exports.col = function (num, elems) {
    return divWithClass(fsFormat("col-sm-%d")(function (x) {
      return x;
    })(num))(elems);
  };

  var fullRow = __exports.fullRow = function (elems) {
    return row(ofArray([col(12, elems)]));
  };

  var layout = __exports.layout = function (elementsWithWidths) {
    return row(toList(delay(function () {
      return collect(function (matchValue) {
        return singleton(col(matchValue[0], matchValue[1]));
      }, elementsWithWidths);
    })));
  };

  var distribute = __exports.distribute = function (elements) {
    var width = ~~(12 / elements.length);
    return layout(map(function (elements_1) {
      return [width, elements_1];
    }, elements));
  };

  return __exports;
}({});
export var Form = function (__exports) {
  var Control = __exports.Control = cn("form-control");
  var Group = __exports.Group = cn("form-group");
  return __exports;
}({});
export var Table = function (__exports) {
  var Basic = __exports.Basic = cn("table");
  var Striped = __exports.Striped = cn("table table-striped");
  var Bordered = __exports.Bordered = cn("table table-bordered");
  var Hover = __exports.Hover = cn("table table-hover");
  var Condensed = __exports.Condensed = cn("table table-condensed");
  var Responsive = __exports.Responsive = cn("table table-responsive");

  var Row = __exports.Row = function (__exports) {
    var Active = __exports.Active = cn("active");
    var Success = __exports.Success = cn("success");
    var Info = __exports.Info = cn("info");
    var Warning = __exports.Warning = cn("warning");
    var Danger = __exports.Danger = cn("danger");
    return __exports;
  }({});

  return __exports;
}({});
export var Button = function (__exports) {
  var Basic = __exports.Basic = cn("btn");
  var Warning = __exports.Warning = cn("btn btn-warning");
  return __exports;
}({});