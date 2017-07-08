var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { defaultArg, equals, GenericParam, compareRecords, equalsRecords, makeGeneric, compareUnions, equalsUnions } from "fable-core/Util";
import { map, ofArray } from "fable-core/List";
import List from "fable-core/List";
import { ResponseFilter } from "./Types";
import { createElement } from "react";
import { update as update_1, Message as Message_1, Model as Model_1, initModel as initModel_1, view as view_1 } from "./Slider";
import { exists, fold } from "fable-core/Seq";
import { Form, Button as Button_1 } from "./Bootstrap";
import { logError } from "./Util";
import { fsFormat } from "fable-core/String";
export var Wiggle = function (__exports) {
  var Data = __exports.Data = function () {
    function Data(caseName, fields) {
      _classCallCheck(this, Data);

      this.Case = caseName;
      this.Fields = fields;
    }

    _createClass(Data, [{
      key: _Symbol.reflection,
      value: function () {
        return {
          type: "Knob.Wiggle.Data",
          interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
          cases: {
            Bipolar: ["number"],
            Unipolar: ["number"]
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

    return Data;
  }();

  setType("Knob.Wiggle.Data", Data);

  var Datatype = __exports.Datatype = function () {
    function Datatype(caseName, fields) {
      _classCallCheck(this, Datatype);

      this.Case = caseName;
      this.Fields = fields;
    }

    _createClass(Datatype, [{
      key: _Symbol.reflection,
      value: function () {
        return {
          type: "Knob.Wiggle.Datatype",
          interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
          cases: {
            Bipolar: [],
            Unipolar: []
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

    return Datatype;
  }();

  setType("Knob.Wiggle.Datatype", Datatype);

  var datatype = __exports.datatype = function (data, _arg1) {
    if (_arg1.Case === "Bipolar") {
      return new Datatype("Bipolar", []);
    } else {
      return new Datatype("Unipolar", []);
    }
  };

  return __exports;
}({});
export var Rate = function () {
  function Rate(caseName, fields) {
    _classCallCheck(this, Rate);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(Rate, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knob.Rate",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Bpm: ["number"],
          Hz: ["number"],
          Period: ["number"]
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
  }, {
    key: "inHz",
    value: function () {
      if (this.Case === "Bpm") {
        return this.Fields[0] / 60;
      } else if (this.Case === "Period") {
        return 1 / this.Fields[0];
      } else {
        return this.Fields[0];
      }
    }
  }, {
    key: "inBpm",
    value: function () {
      if (this.Case === "Bpm") {
        return this.Fields[0];
      } else if (this.Case === "Period") {
        return 60 / this.Fields[0];
      } else {
        return this.Fields[0] * 60;
      }
    }
  }]);

  return Rate;
}();
setType("Knob.Rate", Rate);
export var Datatype = function () {
  function Datatype(caseName, fields) {
    _classCallCheck(this, Datatype);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(Datatype, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knob.Datatype",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Button: [],
          Picker: [makeGeneric(List, {
            T: "string"
          })],
          Rate: [],
          UFloat: [],
          Wiggle: [Wiggle.Datatype]
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

  return Datatype;
}();
setType("Knob.Datatype", Datatype);
export var Data = function () {
  function Data(caseName, fields) {
    _classCallCheck(this, Data);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(Data, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knob.Data",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Button: ["boolean"],
          Picker: ["string"],
          Rate: [Rate],
          UFloat: ["number"],
          Wiggle: [Wiggle.Data]
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

  return Data;
}();
setType("Knob.Data", Data);
export var KnobDescription = function () {
  function KnobDescription(name, datatype) {
    _classCallCheck(this, KnobDescription);

    this.name = name;
    this.datatype = datatype;
  }

  _createClass(KnobDescription, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knob.KnobDescription",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          name: "string",
          datatype: Datatype
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

  return KnobDescription;
}();
setType("Knob.KnobDescription", KnobDescription);
export var ValueChange = function () {
  function ValueChange(addr, value) {
    _classCallCheck(this, ValueChange);

    this.addr = addr;
    this.value = value;
  }

  _createClass(ValueChange, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knob.ValueChange",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          addr: GenericParam("a"),
          value: Data
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

  return ValueChange;
}();
setType("Knob.ValueChange", ValueChange);
export var KnobAdded = function () {
  function KnobAdded(addr, desc) {
    _classCallCheck(this, KnobAdded);

    this.addr = addr;
    this.desc = desc;
  }

  _createClass(KnobAdded, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knob.KnobAdded",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          addr: GenericParam("a"),
          desc: KnobDescription
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

  return KnobAdded;
}();
setType("Knob.KnobAdded", KnobAdded);
export var KnobServerCommand = function () {
  function KnobServerCommand(caseName, fields) {
    _classCallCheck(this, KnobServerCommand);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(KnobServerCommand, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knob.KnobServerCommand",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Get: [GenericParam("a")],
          Set: [makeGeneric(ValueChange, {
            a: GenericParam("a")
          })]
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

  return KnobServerCommand;
}();
setType("Knob.KnobServerCommand", KnobServerCommand);
export function createSetCommand(addr, wrapper, value) {
  var v = wrapper(value);
  return [new ResponseFilter("AllButSelf", []), new KnobServerCommand("Set", [new ValueChange(addr, v)])];
}
export var KnobServerResponse = function () {
  function KnobServerResponse(caseName, fields) {
    _classCallCheck(this, KnobServerResponse);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(KnobServerResponse, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knob.KnobServerResponse",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          KnobAdded: [makeGeneric(KnobAdded, {
            a: GenericParam("a")
          })],
          KnobRemoved: [GenericParam("a")],
          ValueChange: [makeGeneric(ValueChange, {
            a: GenericParam("a")
          })]
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

  return KnobServerResponse;
}();
setType("Knob.KnobServerResponse", KnobServerResponse);
export function viewSlider(dataWrapper, name, addr, model, dispatchLocal, dispatchServer) {
  var onValueChange = function onValueChange(v) {
    dispatchServer(createSetCommand(addr, dataWrapper, v));
  };

  return createElement("div", {}, name, view_1(onValueChange, model, dispatchLocal));
}
export var Unipolar = function (__exports) {
  var initModel = __exports.initModel = function () {
    return initModel_1(0, 0, 1, 0.0001, ofArray([0, 1]));
  };

  var view = __exports.view = function (name, addr, model, dispatchLocal, dispatchServer) {
    return viewSlider(function ($var124) {
      return new Data("Wiggle", [new Wiggle.Data("Unipolar", [$var124])]);
    }, name, addr, model, dispatchLocal, dispatchServer);
  };

  return __exports;
}({});
export var Bipolar = function (__exports) {
  var initModel = __exports.initModel = function () {
    return initModel_1(0, -1, 1, 0.0001, ofArray([-1, 0, 1]));
  };

  var view = __exports.view = function (name, addr, model, dispatchLocal, dispatchServer) {
    return viewSlider(function ($var125) {
      return new Data("Wiggle", [new Wiggle.Data("Bipolar", [$var125])]);
    }, name, addr, model, dispatchLocal, dispatchServer);
  };

  return __exports;
}({});
export var RateModule = function (__exports) {
  var initModel = __exports.initModel = function () {
    return initModel_1(60, 0, 200, 0.01, new List());
  };

  var view = __exports.view = function (name, addr, model, dispatchLocal, dispatchServer) {
    return viewSlider(function ($var126) {
      return new Data("Rate", [new Rate("Bpm", [$var126])]);
    }, name, addr, model, dispatchLocal, dispatchServer);
  };

  return __exports;
}({});
export var UFloat = function (__exports) {
  var initModel = __exports.initModel = function () {
    return initModel_1(1, 0, 4, 0.001, ofArray([0, 0.25, 0.5, 1, 2, 4]));
  };

  var view = __exports.view = function (name, addr, model, dispatchLocal, dispatchServer) {
    return viewSlider(function (arg0) {
      return new Data("UFloat", [arg0]);
    }, name, addr, model, dispatchLocal, dispatchServer);
  };

  return __exports;
}({});
export var Button = function (__exports) {
  var initModel = __exports.initModel = function () {
    return false;
  };

  var update = __exports.update = function (message, _arg1) {
    return message;
  };

  var view = __exports.view = function (name, addr, state, dispatchLocal, dispatchServer) {
    return createElement("button", fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [["onClick", function (_arg1_1) {
      dispatchLocal(!state);
      dispatchServer(createSetCommand(addr, function (arg0_1) {
        return new Data("Button", [arg0_1]);
      }, !state));
    }], state ? Button_1.Info : Button_1.Default]), name);
  };

  return __exports;
}({});
export var Picker = function (__exports) {
  var Model = __exports.Model = function () {
    function Model(selected, options) {
      _classCallCheck(this, Model);

      this.selected = selected;
      this.options = options;
    }

    _createClass(Model, [{
      key: _Symbol.reflection,
      value: function () {
        return {
          type: "Knob.Picker.Model",
          interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
          properties: {
            selected: "string",
            options: makeGeneric(List, {
              T: "string"
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

  setType("Knob.Picker.Model", Model);

  var initModel = __exports.initModel = function (options) {
    var selected = void 0;

    if (options.tail == null) {
      logError("Constructing a picker knob with an empty option list.");
      selected = "";
    } else {
      selected = options.head;
    }

    return new Model(selected, options);
  };

  var update = __exports.update = function (message, model) {
    if (function (source) {
      return exists(function (x) {
        return equals(message, x);
      }, source);
    }(model.options)) {
      return new Model(message, model.options);
    } else {
      logError(fsFormat("Picker knob got a bad value: '%s', expected one of %O.")(function (x) {
        return x;
      })(message)(model.options));
      return model;
    }
  };

  var view = __exports.view = function (name, addr, model, dispatchLocal, dispatchServer) {
    return createElement("div", {}, name, createElement.apply(undefined, ["select", fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [["value", model.selected], ["onChange", function (e_1) {
      var selected_1 = e_1.target.value;
      dispatchLocal(selected_1);
      dispatchServer(createSetCommand(addr, function (arg0_1) {
        return new Data("Picker", [arg0_1]);
      }, selected_1));
    }], Form.Control])].concat(_toConsumableArray(map(function (s) {
      return createElement("option", {
        value: s
      }, s);
    }, model.options)))));
  };

  return __exports;
}({});
export var ViewModel = function () {
  function ViewModel(caseName, fields) {
    _classCallCheck(this, ViewModel);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ViewModel, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knob.ViewModel",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Bipolar: [Model_1],
          Button: ["boolean"],
          Picker: [Picker.Model],
          Rate: [Model_1],
          UFloat: [Model_1],
          Unipolar: [Model_1]
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

  return ViewModel;
}();
setType("Knob.ViewModel", ViewModel);
export var Model = function () {
  function Model(name, data) {
    _classCallCheck(this, Model);

    this.name = name;
    this.data = data;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Knob.Model",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          name: "string",
          data: ViewModel
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
setType("Knob.Model", Model);
export function fromDesc(d) {
  var initData = d.datatype.Case === "Rate" ? new ViewModel("Rate", [RateModule.initModel()]) : d.datatype.Case === "Button" ? new ViewModel("Button", [Button.initModel()]) : d.datatype.Case === "UFloat" ? new ViewModel("UFloat", [UFloat.initModel()]) : d.datatype.Case === "Picker" ? new ViewModel("Picker", [Picker.initModel(d.datatype.Fields[0])]) : d.datatype.Fields[0].Case === "Bipolar" ? new ViewModel("Bipolar", [Bipolar.initModel()]) : new ViewModel("Unipolar", [Unipolar.initModel()]);
  return new Model(d.name, initData);
}
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
        type: "Knob.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Button: ["boolean"],
          Picker: ["string"],
          Slider: [Message_1],
          ValueChange: [Data]
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
setType("Knob.Message", Message);
export function update(message, model) {
  if (message.Case === "Slider") {
    return function (_arg1) {
      if (_arg1 == null) {
        logError(fsFormat("Knob %s ignored a slider message.")(function (x) {
          return x;
        })(model.name));
        return model;
      } else {
        return _arg1;
      }
    }(defaultArg(model.data.Case === "Unipolar" ? [function (arg0) {
      return new ViewModel("Unipolar", [arg0]);
    }, model.data.Fields[0]] : model.data.Case === "Bipolar" ? [function (arg0_1) {
      return new ViewModel("Bipolar", [arg0_1]);
    }, model.data.Fields[0]] : model.data.Case === "Rate" ? [function (arg0_2) {
      return new ViewModel("Rate", [arg0_2]);
    }, model.data.Fields[0]] : model.data.Case === "UFloat" ? [function (arg0_3) {
      return new ViewModel("UFloat", [arg0_3]);
    }, model.data.Fields[0]] : null, null, function (tupledArg) {
      var updatedSlider = update_1(message.Fields[0], tupledArg[1]);
      var data = tupledArg[0](updatedSlider);
      return new Model(model.name, data);
    }));
  } else if (message.Case === "Button") {
    if (model.data.Case === "Button") {
      var data_1 = new ViewModel("Button", [message.Fields[0]]);
      return new Model(model.name, data_1);
    } else {
      logError(fsFormat("Knob %s ignored a button message.")(function (x) {
        return x;
      })(model.name));
      return model;
    }
  } else if (message.Case === "Picker") {
    if (model.data.Case === "Picker") {
      var data_2 = new ViewModel("Picker", [Picker.update(message.Fields[0], model.data.Fields[0])]);
      return new Model(model.name, data_2);
    } else {
      logError(fsFormat("Knob %s ignored a picker message.")(function (x) {
        return x;
      })(model.name));
      return model;
    }
  } else {
    var matchValue = [message.Fields[0], model.data];
    var $var127 = matchValue[0].Case === "Rate" ? matchValue[1].Case === "Rate" ? [2, matchValue[0].Fields[0], matchValue[1].Fields[0]] : [6] : matchValue[0].Case === "Button" ? matchValue[1].Case === "Button" ? [3, matchValue[0].Fields[0]] : [6] : matchValue[0].Case === "UFloat" ? matchValue[1].Case === "UFloat" ? [4, matchValue[0].Fields[0], matchValue[1].Fields[0]] : [6] : matchValue[0].Case === "Picker" ? matchValue[1].Case === "Picker" ? [5, matchValue[0].Fields[0], matchValue[1].Fields[0]] : [6] : matchValue[0].Fields[0].Case === "Bipolar" ? matchValue[1].Case === "Bipolar" ? [1, matchValue[0].Fields[0].Fields[0], matchValue[1].Fields[0]] : [6] : matchValue[1].Case === "Unipolar" ? [0, matchValue[0].Fields[0].Fields[0], matchValue[1].Fields[0]] : [6];

    switch ($var127[0]) {
      case 0:
        var newDat = new Model_1($var127[2].uniqueId, $var127[1], $var127[2].min, $var127[2].max, $var127[2].step, $var127[2].detents, $var127[2].inputEventHasFired);
        var data_3 = new ViewModel("Unipolar", [newDat]);
        return new Model(model.name, data_3);

      case 1:
        var newDat_1 = new Model_1($var127[2].uniqueId, $var127[1], $var127[2].min, $var127[2].max, $var127[2].step, $var127[2].detents, $var127[2].inputEventHasFired);
        var data_4 = new ViewModel("Bipolar", [newDat_1]);
        return new Model(model.name, data_4);

      case 2:
        var newDat_2 = void 0;
        var value = $var127[1].inBpm();
        newDat_2 = new Model_1($var127[2].uniqueId, value, $var127[2].min, $var127[2].max, $var127[2].step, $var127[2].detents, $var127[2].inputEventHasFired);
        var data_5 = new ViewModel("Rate", [newDat_2]);
        return new Model(model.name, data_5);

      case 3:
        var data_6 = new ViewModel("Button", [$var127[1]]);
        return new Model(model.name, data_6);

      case 4:
        var newDat_3 = new Model_1($var127[2].uniqueId, $var127[1], $var127[2].min, $var127[2].max, $var127[2].step, $var127[2].detents, $var127[2].inputEventHasFired);
        var data_7 = new ViewModel("UFloat", [newDat_3]);
        return new Model(model.name, data_7);

      case 5:
        var data_8 = new ViewModel("Picker", [Picker.update($var127[1], $var127[2])]);
        return new Model(model.name, data_8);

      case 6:
        logError(fsFormat("Invalid knob value change message for knob %s.  Current data: %+A")(function (x) {
          return x;
        })(model.name)(model.data));
        return model;
    }
  }
}
export function view(addr, model, dispatchLocal, dispatchServer) {
  if (model.data.Case === "Bipolar") {
    return Bipolar.view(model.name, addr, model.data.Fields[0], function ($var128) {
      return dispatchLocal(function (arg0) {
        return new Message("Slider", [arg0]);
      }($var128));
    }, dispatchServer);
  } else if (model.data.Case === "Rate") {
    return RateModule.view(model.name, addr, model.data.Fields[0], function ($var129) {
      return dispatchLocal(function (arg0_1) {
        return new Message("Slider", [arg0_1]);
      }($var129));
    }, dispatchServer);
  } else if (model.data.Case === "UFloat") {
    return UFloat.view(model.name, addr, model.data.Fields[0], function ($var130) {
      return dispatchLocal(function (arg0_2) {
        return new Message("Slider", [arg0_2]);
      }($var130));
    }, dispatchServer);
  } else if (model.data.Case === "Button") {
    return Button.view(model.name, addr, model.data.Fields[0], function ($var131) {
      return dispatchLocal(function (arg0_3) {
        return new Message("Button", [arg0_3]);
      }($var131));
    }, dispatchServer);
  } else if (model.data.Case === "Picker") {
    return Picker.view(model.name, addr, model.data.Fields[0], function ($var132) {
      return dispatchLocal(function (arg0_4) {
        return new Message("Picker", [arg0_4]);
      }($var132));
    }, dispatchServer);
  } else {
    return Unipolar.view(model.name, addr, model.data.Fields[0], function ($var133) {
      return dispatchLocal(function (arg0_5) {
        return new Message("Slider", [arg0_5]);
      }($var133));
    }, dispatchServer);
  }
}