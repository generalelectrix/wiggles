var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { Option, compareRecords, equalsRecords, makeGeneric } from "fable-core/Util";
import { slice, ofArray, append, mapIndexed } from "fable-core/List";
import List from "fable-core/List";
import { remove, add, create } from "fable-core/Map";
import _Map from "fable-core/Map";
import { ClockId } from "./DataflowTypes";
import { Command, SetInput, ClockDescription } from "./ClockTypes";
import { logError, transformMapItem } from "./Util";
import GenericComparer from "fable-core/GenericComparer";
import { fsFormat } from "fable-core/String";
import { createElement } from "react";
import { ofJson, toJson } from "fable-core/Serialize";
import { fold, map as map_1, toList } from "fable-core/Seq";
import { all } from "./Types";
import { Form } from "./Bootstrap";
import { viewAllWith } from "./Knobs";
export var Model = function () {
  function Model(classes, clocks) {
    _classCallCheck(this, Model);

    this.classes = classes;
    this.clocks = clocks;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Clocks.Model",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          classes: makeGeneric(List, {
            T: "string"
          }),
          clocks: makeGeneric(_Map, {
            Key: ClockId,
            Value: ClockDescription
          })
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
setType("Clocks.Model", Model);
export function updateFromServer(response, model) {
  var transformClock = function transformClock(id) {
    return function (f) {
      var clocks = function (map) {
        return transformMapItem(id, f, map);
      }(model.clocks);

      return new Model(model.classes, clocks);
    };
  };

  var transformInputs = function transformInputs(id_1) {
    return function (f_1) {
      var transform = function transform(clock) {
        var inputs = f_1(clock.inputs);
        return new ClockDescription(clock.name, clock.kind, inputs);
      };

      return transformClock(id_1)(transform);
    };
  };

  if (response.Case === "State") {
    var clocks_1 = create(response.Fields[0], new GenericComparer(function (x, y) {
      return x.CompareTo(y);
    }));
    return new Model(model.classes, clocks_1);
  } else if (response.Case === "New") {
    var clocks_2 = function (table) {
      return add(response.Fields[0], response.Fields[1], table);
    }(model.clocks);

    return new Model(model.classes, clocks_2);
  } else if (response.Case === "Removed") {
    var clocks_3 = function (table_1) {
      return remove(response.Fields[0], table_1);
    }(model.clocks);

    return new Model(model.classes, clocks_3);
  } else if (response.Case === "Renamed") {
    return transformClock(response.Fields[0])(function (clock_1) {
      return new ClockDescription(response.Fields[1], clock_1.kind, clock_1.inputs);
    });
  } else if (response.Case === "SetInput") {
    return transformInputs(response.Fields[0].clock)(function () {
      var mapping = function mapping(inputId) {
        return function (input) {
          if (response.Fields[0].input === inputId) {
            return response.Fields[0].target;
          } else {
            return input;
          }
        };
      };

      return function (list) {
        return mapIndexed(function ($var219, $var220) {
          return mapping($var219)($var220);
        }, list);
      };
    }());
  } else if (response.Case === "PushInput") {
    return transformInputs(response.Fields[0])(function (inputs_1) {
      return append(inputs_1, ofArray([null]));
    });
  } else if (response.Case === "PopInput") {
    return transformInputs(response.Fields[0])(function (inputs_2) {
      if (inputs_2.tail == null) {
        logError(fsFormat("Got a command to pop an input from clock %+A but it already has no inputs.")(function (x) {
          return x;
        })(response.Fields[0]));
        return inputs_2;
      } else {
        return slice(null, inputs_2.length - 1, inputs_2);
      }
    });
  } else {
    return new Model(response.Fields[0], model.clocks);
  }
}

function inputSelector(clockId, inputId, currentValue, clocks, dispatchServer) {
  var option = function option(tupledArg) {
    return createElement("option", {
      value: toJson(tupledArg[0])
    }, tupledArg[1].name);
  };

  var options = toList(function (source) {
    return map_1(option, source);
  }(clocks));
  return createElement("div", {}, createElement.apply(undefined, ["select", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", toJson(currentValue)], ["onChange", function (e_1) {
    var selected_1 = ofJson(e_1.target.value, {
      T: Option(ClockId)
    });
    var cmd_1 = new SetInput(clockId, inputId, selected_1);
    dispatchServer(all(new Command("SetInput", [cmd_1])));
  }], Form.Control])].concat(_toConsumableArray(options))));
}

export function viewClock(clockId, clock, clocks, knobs, dispatchKnobLocal, dispatchKnobServer, dispatchClockServer) {
  var inputSelectors = createElement.apply(undefined, ["div", {}].concat(_toConsumableArray(mapIndexed(function (inputId, source) {
    return inputSelector(clockId, inputId, source, clocks, dispatchClockServer);
  }, clock.inputs))));

  var addrFilter = function addrFilter(addr) {
    var $var221 = addr.Case === "Clock" ? function () {
      var id = addr.Fields[0][0];
      return clockId.Equals(id);
    }() ? [0, addr.Fields[0][0]] : [1] : [1];

    switch ($var221[0]) {
      case 0:
        return true;

      case 1:
        return false;
    }
  };

  return createElement("div", {}, fsFormat("%s (%s)")(function (x) {
    return x;
  })(clock.name)(clock.kind), inputSelectors, viewAllWith(addrFilter, knobs, dispatchKnobLocal, dispatchKnobServer));
}