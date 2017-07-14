var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { $7C$Parsed$7C$_$7C$ as _Parsed___, view as view_1, update as update_1, Message as Message_1, initialModel, setFailed, Model as Model_1 } from "./EditBox";
import { equals, Option, compare, Tuple, compareUnions, equalsUnions, makeGeneric } from "fable-core/Util";
import { logError, transformMapItem, errorIfEmpty } from "./Util";
import { Button, Form, InputType } from "./Bootstrap";
import { slice, append, mapIndexed, ofArray, map as map_1 } from "fable-core/List";
import List from "fable-core/List";
import { createElement } from "react";
import { map as map_2, singleton, append as append_1, delay, toList, fold } from "fable-core/Seq";
import { SetInput, Response, ClockDescription, Command, CreateClock } from "./ClockTypes";
import { all } from "./Types";
import { remove, add, create as create_2 } from "fable-core/Map";
import _Map from "fable-core/Map";
import GenericComparer from "fable-core/GenericComparer";
import { fsFormat } from "fable-core/String";
import { ofJson, toJson } from "fable-core/Serialize";
import { viewAllWith } from "./Knobs";
export var NewClock = function (__exports) {
  var Model = __exports.Model = function () {
    function Model(name, selectedKind) {
      _classCallCheck(this, Model);

      this.name = name;
      this.selectedKind = selectedKind;
    }

    _createClass(Model, [{
      key: _Symbol.reflection,
      value: function () {
        return {
          type: "Clocks.NewClock.Model",
          interfaces: ["FSharpRecord"],
          properties: {
            name: makeGeneric(Model_1, {
              T: "string"
            }),
            selectedKind: "string"
          }
        };
      }
    }]);

    return Model;
  }();

  setType("Clocks.NewClock.Model", Model);

  var initModel = __exports.initModel = function () {
    return new Model(setFailed("", initialModel("Name:", errorIfEmpty, InputType.Text)), "");
  };

  var Message = __exports.Message = function () {
    function Message(caseName, fields) {
      _classCallCheck(this, Message);

      this.Case = caseName;
      this.Fields = fields;
    }

    _createClass(Message, [{
      key: _Symbol.reflection,
      value: function () {
        return {
          type: "Clocks.NewClock.Message",
          interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
          cases: {
            NameEdit: [makeGeneric(Message_1, {
              T: "string"
            })],
            SelectKind: ["string"]
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

  setType("Clocks.NewClock.Message", Message);

  var update = __exports.update = function (message, model) {
    if (message.Case === "SelectKind") {
      return new Model(model.name, message.Fields[0]);
    } else {
      return new Model(update_1(message.Fields[0], model.name), model.selectedKind);
    }
  };

  var viewKindSelector = function viewKindSelector(kinds, selected, dispatch) {
    var options = map_1(function (kind) {
      return createElement("option", {
        value: kind
      }, kind);
    }, kinds);
    return createElement("div", {}, createElement.apply(undefined, ["select", fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [["value", selected], ["onChange", function (e_1) {
      dispatch(new Message("SelectKind", [e_1.target.value]));
    }], Form.Control])].concat(_toConsumableArray(options))));
  };

  var view = __exports.view = function (kinds, model, dispatchLocal, dispatchServer) {
    return createElement("div", {}, createElement("h4", {}, "Create new clock"), view_1(null, "", model.name, function ($var220) {
      return dispatchLocal(function (arg0) {
        return new Message("NameEdit", [arg0]);
      }($var220));
    }), createElement("label", {}, "Kind:", viewKindSelector(kinds, model.selectedKind, dispatchLocal)), createElement("button", fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [["onClick", function (_arg1_1) {
      var activePatternResult1390_1 = _Parsed___(model.name);

      if (activePatternResult1390_1 != null) {
        var create_1 = new CreateClock(model.selectedKind, activePatternResult1390_1);
        dispatchServer(all(new Command("Create", [create_1])));
      }
    }], Button.Primary]), "Create"));
  };

  return __exports;
}({});
export var Model = function () {
  function Model(kinds, clocks, newClock) {
    _classCallCheck(this, Model);

    this.kinds = kinds;
    this.clocks = clocks;
    this.newClock = newClock;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Clocks.Model",
        interfaces: ["FSharpRecord"],
        properties: {
          kinds: makeGeneric(List, {
            T: "string"
          }),
          clocks: makeGeneric(_Map, {
            Key: Tuple(["number", "number"]),
            Value: ClockDescription
          }),
          newClock: NewClock.Model
        }
      };
    }
  }]);

  return Model;
}();
setType("Clocks.Model", Model);
export function initModel() {
  return new Model(new List(), create_2(null, new GenericComparer(compare)), NewClock.initModel());
}
export var initCommands = ofArray([new Command("Kinds", []), new Command("State", [])]);
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
        type: "Clocks.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          NewClock: [NewClock.Message],
          Response: [Response]
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
setType("Clocks.Message", Message);
export function updateFromServer(response, model) {
  var transformClock = function transformClock(id) {
    return function (f) {
      var clocks = function (map) {
        return transformMapItem(id, f, map);
      }(model.clocks);

      return new Model(model.kinds, clocks, model.newClock);
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
    var clocks_1 = create_2(response.Fields[0], new GenericComparer(compare));
    return new Model(model.kinds, clocks_1, model.newClock);
  } else if (response.Case === "New") {
    var clocks_2 = function (table) {
      return add(response.Fields[0], response.Fields[1], table);
    }(model.clocks);

    return new Model(model.kinds, clocks_2, model.newClock);
  } else if (response.Case === "Removed") {
    var clocks_3 = function (table_1) {
      return remove(response.Fields[0], table_1);
    }(model.clocks);

    return new Model(model.kinds, clocks_3, model.newClock);
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
        return mapIndexed(function ($var221, $var222) {
          return mapping($var221)($var222);
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
    var newClock = void 0;
    var selectedKind = response.Fields[0].head;
    newClock = new NewClock.Model(model.newClock.name, selectedKind);
    return new Model(response.Fields[0], model.clocks, newClock);
  }
}
export function update(message, model) {
  if (message.Case === "NewClock") {
    var newClock = NewClock.update(message.Fields[0], model.newClock);
    return new Model(model.kinds, model.clocks, newClock);
  } else {
    return updateFromServer(message.Fields[0], model);
  }
}

function inputSelector(clockId_0, clockId_1, inputId, currentValue, clocks, dispatchServer) {
  var clockId = [clockId_0, clockId_1];

  var option = function option(tupledArg) {
    return createElement("option", {
      value: toJson(tupledArg[0])
    }, tupledArg[1].name);
  };

  var options = toList(delay(function () {
    return append_1(singleton(createElement("option", {
      value: toJson(null)
    }, "{disconnected}")), delay(function () {
      return map_2(function (clock) {
        return option(clock);
      }, clocks);
    }));
  }));
  return createElement("div", {}, createElement.apply(undefined, ["select", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", toJson(currentValue)], ["onChange", function (e_1) {
    var selected_1 = ofJson(e_1.target.value, {
      T: Option(Tuple(["number", "number"]))
    });
    var cmd_1 = new SetInput(clockId, inputId, selected_1);
    dispatchServer(all(new Command("SetInput", [cmd_1])));
  }], Form.Control])].concat(_toConsumableArray(options))));
}

export function addInput(clockId_0, clockId_1, dispatchServer) {
  var clockId = [clockId_0, clockId_1];
  return createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", function (_arg1_1) {
    dispatchServer(all(new Command("PushInput", [clockId])));
  }], Button.Primary]), "Add Input");
}
export function dropInput(clockId_0, clockId_1, dispatchServer) {
  var clockId = [clockId_0, clockId_1];
  return createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", function (_arg1_1) {
    dispatchServer(all(new Command("PopInput", [clockId])));
  }], Button.Default]), "Drop Input");
}
export function viewClock(clockId_0, clockId_1, clock, clocks, knobs, dispatchKnobLocal, dispatchKnobServer, dispatchClockServer) {
  var clockId = [clockId_0, clockId_1];
  var inputSelectors = createElement.apply(undefined, ["div", {}].concat(_toConsumableArray(mapIndexed(function (inputId, source) {
    return inputSelector(clockId[0], clockId[1], inputId, source, clocks, dispatchClockServer);
  }, clock.inputs))));

  var addrFilter = function addrFilter(addr) {
    var $var223 = addr.Case === "Clock" ? function () {
      var id = addr.Fields[0][0];
      return equals(clockId, id);
    }() ? [0, addr.Fields[0][0]] : [1] : [1];

    switch ($var223[0]) {
      case 0:
        return true;

      case 1:
        return false;
    }
  };

  return createElement("div", {
    style: {
      width: "200px"
    }
  }, fsFormat("%s (%s)")(function (x) {
    return x;
  })(clock.name)(clock.kind), addInput(clockId[0], clockId[1], dispatchClockServer), dropInput(clockId[0], clockId[1], dispatchClockServer), inputSelectors, viewAllWith(addrFilter, knobs, dispatchKnobLocal, dispatchKnobServer));
}
export function viewAllClocks(clocks, knobs, dispatchKnobLocal, dispatchKnobServer, dispatchClockServer) {
  return createElement.apply(undefined, ["div", {}].concat(_toConsumableArray(toList(map_2(function (tupledArg) {
    return viewClock(tupledArg[0][0], tupledArg[0][1], tupledArg[1], clocks, knobs, dispatchKnobLocal, dispatchKnobServer, dispatchClockServer);
  }, clocks)))));
}
export function view(knobs, model, dispatchKnob, dispatchClock, dispatchKnobServer, dispatchClockServer) {
  return createElement("div", {}, NewClock.view(model.kinds, model.newClock, function ($var224) {
    return dispatchClock(function (arg0) {
      return new Message("NewClock", [arg0]);
    }($var224));
  }, dispatchClockServer), viewAllClocks(model.clocks, knobs, dispatchKnob, dispatchKnobServer, dispatchClockServer));
}