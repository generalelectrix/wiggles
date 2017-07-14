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
import { collect, singleton, append as append_1, toList, range, map as map_2, delay, fold } from "fable-core/Seq";
import { SetInput, UsesClock, Response, WiggleDescription, Command, CreateWiggle } from "./WiggleTypes";
import { all } from "./Types";
import { remove, add, create as create_2 } from "fable-core/Map";
import _Map from "fable-core/Map";
import GenericComparer from "fable-core/GenericComparer";
import { fsFormat } from "fable-core/String";
import { ofJson, toJson } from "fable-core/Serialize";
import { viewAllWith } from "./Knobs";
export var NewWiggle = function (__exports) {
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
          type: "Wiggles.NewWiggle.Model",
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

  setType("Wiggles.NewWiggle.Model", Model);

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
          type: "Wiggles.NewWiggle.Message",
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

  setType("Wiggles.NewWiggle.Message", Message);

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
    return createElement("div", {}, createElement("h4", {}, "Create new wiggle"), view_1(null, "", model.name, function ($var261) {
      return dispatchLocal(function (arg0) {
        return new Message("NameEdit", [arg0]);
      }($var261));
    }), createElement("label", {}, "Kind:", viewKindSelector(kinds, model.selectedKind, dispatchLocal)), createElement("button", fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [["onClick", function (_arg1_1) {
      var activePatternResult1488_1 = _Parsed___(model.name);

      if (activePatternResult1488_1 != null) {
        var create_1 = new CreateWiggle(model.selectedKind, activePatternResult1488_1);
        dispatchServer(all(new Command("Create", [create_1])));
      }
    }], Button.Primary]), "Create"));
  };

  return __exports;
}({});
export var Model = function () {
  function Model(kinds, wiggles, newWiggle) {
    _classCallCheck(this, Model);

    this.kinds = kinds;
    this.wiggles = wiggles;
    this.newWiggle = newWiggle;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Wiggles.Model",
        interfaces: ["FSharpRecord"],
        properties: {
          kinds: makeGeneric(List, {
            T: "string"
          }),
          wiggles: makeGeneric(_Map, {
            Key: Tuple(["number", "number"]),
            Value: WiggleDescription
          }),
          newWiggle: NewWiggle.Model
        }
      };
    }
  }]);

  return Model;
}();
setType("Wiggles.Model", Model);
export function initModel() {
  return new Model(new List(), create_2(null, new GenericComparer(compare)), NewWiggle.initModel());
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
        type: "Wiggles.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          NewWiggle: [NewWiggle.Message],
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
setType("Wiggles.Message", Message);
export function updateFromServer(response, model) {
  var transformWiggle = function transformWiggle(id) {
    return function (f) {
      var wiggles = function (map) {
        return transformMapItem(id, f, map);
      }(model.wiggles);

      return new Model(model.kinds, wiggles, model.newWiggle);
    };
  };

  var transformInputs = function transformInputs(id_1) {
    return function (f_1) {
      var transform = function transform(wiggle) {
        var inputs = f_1(wiggle.inputs);
        return new WiggleDescription(wiggle.name, wiggle.kind, inputs, wiggle.outputs, wiggle.clock);
      };

      return transformWiggle(id_1)(transform);
    };
  };

  if (response.Case === "State") {
    var wiggles_1 = create_2(response.Fields[0], new GenericComparer(compare));
    return new Model(model.kinds, wiggles_1, model.newWiggle);
  } else if (response.Case === "New") {
    var wiggles_2 = function (table) {
      return add(response.Fields[0], response.Fields[1], table);
    }(model.wiggles);

    return new Model(model.kinds, wiggles_2, model.newWiggle);
  } else if (response.Case === "Removed") {
    var wiggles_3 = function (table_1) {
      return remove(response.Fields[0], table_1);
    }(model.wiggles);

    return new Model(model.kinds, wiggles_3, model.newWiggle);
  } else if (response.Case === "Renamed") {
    return transformWiggle(response.Fields[0])(function (wiggle_1) {
      return new WiggleDescription(response.Fields[1], wiggle_1.kind, wiggle_1.inputs, wiggle_1.outputs, wiggle_1.clock);
    });
  } else if (response.Case === "SetInput") {
    return transformInputs(response.Fields[0].wiggle)(function () {
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
        return mapIndexed(function ($var262, $var263) {
          return mapping($var262)($var263);
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
        logError(fsFormat("Got a command to pop an input from wiggle %+A but it already has no inputs.")(function (x) {
          return x;
        })(response.Fields[0]));
        return inputs_2;
      } else {
        return slice(null, inputs_2.length - 1, inputs_2);
      }
    });
  } else if (response.Case === "PushOutput") {
    return transformWiggle(response.Fields[0])(function (wiggle_2) {
      var outputs = wiggle_2.outputs + 1;
      return new WiggleDescription(wiggle_2.name, wiggle_2.kind, wiggle_2.inputs, outputs, wiggle_2.clock);
    });
  } else if (response.Case === "PopOutput") {
    return transformWiggle(response.Fields[0])(function (wiggle_3) {
      if (wiggle_3.outputs === 0) {
        logError(fsFormat("Got a command to pop an output from wiggle %+A but it already has no outputs.")(function (x) {
          return x;
        })(response.Fields[0]));
        return wiggle_3;
      } else {
        var outputs_1 = wiggle_3.outputs - 1;
        return new WiggleDescription(wiggle_3.name, wiggle_3.kind, wiggle_3.inputs, outputs_1, wiggle_3.clock);
      }
    });
  } else if (response.Case === "SetClock") {
    return transformWiggle(response.Fields[0])(function (wiggle_4) {
      var clock = new UsesClock("Yes", [response.Fields[1]]);
      return new WiggleDescription(wiggle_4.name, wiggle_4.kind, wiggle_4.inputs, wiggle_4.outputs, clock);
    });
  } else {
    var newWiggle = void 0;
    var selectedKind = response.Fields[0].head;
    newWiggle = new NewWiggle.Model(model.newWiggle.name, selectedKind);
    return new Model(response.Fields[0], model.wiggles, newWiggle);
  }
}
export function update(message, model) {
  if (message.Case === "NewWiggle") {
    var newWiggle = NewWiggle.update(message.Fields[0], model.newWiggle);
    return new Model(model.kinds, model.wiggles, newWiggle);
  } else {
    return updateFromServer(message.Fields[0], model);
  }
}

function inputSelector(wiggleId_0, wiggleId_1, inputId, currentValue, wiggles, dispatchServer) {
  var wiggleId = [wiggleId_0, wiggleId_1];

  var options = function options(tupledArg) {
    return delay(function () {
      return map_2(function (outputId) {
        return createElement("option", {
          value: toJson([tupledArg[0], outputId])
        }, fsFormat("%s (output %d)")(function (x) {
          return x;
        })(tupledArg[1].name)(outputId));
      }, range(0, tupledArg[1].outputs - 1));
    });
  };

  var options_1 = toList(delay(function () {
    return append_1(singleton(createElement("option", {
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
  }, {}, [["value", toJson(currentValue)], ["onChange", function (e_1) {
    var selected_1 = ofJson(e_1.target.value, {
      T: Option(Tuple([Tuple(["number", "number"]), "number"]))
    });
    var cmd_1 = new SetInput(wiggleId, inputId, selected_1);
    dispatchServer(all(new Command("SetInput", [cmd_1])));
  }], Form.Control])].concat(_toConsumableArray(options_1))));
}

function clockSelector(wiggleId_0, wiggleId_1, currentValue, clocks, dispatchServer) {
  var wiggleId = [wiggleId_0, wiggleId_1];

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
    dispatchServer(all(function (tupledArg_2) {
      return new Command("SetClock", [tupledArg_2[0], tupledArg_2[1]]);
    }([wiggleId, selected_1])));
  }], Form.Control])].concat(_toConsumableArray(options))));
}

export function viewWiggle(wiggleId_0, wiggleId_1, wiggle, wiggles, knobs, clocks, dispatchKnobLocal, dispatchKnobServer, dispatchWiggleServer) {
  var wiggleId = [wiggleId_0, wiggleId_1];
  var inputSelectors = createElement.apply(undefined, ["div", {}].concat(_toConsumableArray(mapIndexed(function (inputId, source) {
    return inputSelector(wiggleId[0], wiggleId[1], inputId, source, wiggles, dispatchWiggleServer);
  }, wiggle.inputs))));
  var clockSelector_1 = void 0;

  if (wiggle.clock.Case === "No") {
    clockSelector_1 = createElement("div", {});
  } else {
    var selector = clockSelector(wiggleId[0], wiggleId[1], wiggle.clock.Fields[0], clocks, dispatchWiggleServer);
    clockSelector_1 = createElement("div", {}, "Clock Source:", selector);
  }

  var addrFilter = function addrFilter(addr) {
    var $var264 = addr.Case === "Wiggle" ? function () {
      var id = addr.Fields[0][0];
      return equals(wiggleId, id);
    }() ? [0, addr.Fields[0][0]] : [1] : [1];

    switch ($var264[0]) {
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
  })(wiggle.name)(wiggle.kind), clockSelector_1, createElement("div", {}, "Inputs:", inputSelectors), viewAllWith(addrFilter, knobs, dispatchKnobLocal, dispatchKnobServer));
}
export function viewAllWiggles(wiggles, knobs, clocks, dispatchKnobLocal, dispatchKnobServer, dispatchWiggleServer) {
  return createElement.apply(undefined, ["div", {}].concat(_toConsumableArray(toList(map_2(function (tupledArg) {
    return viewWiggle(tupledArg[0][0], tupledArg[0][1], tupledArg[1], wiggles, knobs, clocks, dispatchKnobLocal, dispatchKnobServer, dispatchWiggleServer);
  }, wiggles)))));
}
export function view(knobs, clocks, model, dispatchKnob, dispatchWiggle, dispatchKnobServer, dispatchWiggleServer) {
  return createElement("div", {}, NewWiggle.view(model.kinds, model.newWiggle, function ($var265) {
    return dispatchWiggle(function (arg0) {
      return new Message("NewWiggle", [arg0]);
    }($var265));
  }, dispatchWiggleServer), viewAllWiggles(model.wiggles, knobs, clocks, dispatchKnob, dispatchKnobServer, dispatchWiggleServer));
}