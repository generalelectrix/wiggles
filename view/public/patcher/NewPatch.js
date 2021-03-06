var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { defaultArg, compare, compareUnions, equalsUnions, makeGeneric, Option, Array as _Array } from "fable-core/Util";
import { PatchServerRequest, globalAddressFromOptionals, PatchRequest, parseDmxAddress, parseUniverseId, FixtureKind } from "./PatchTypes";
import { view as view_1, $7C$Parsed$7C$_$7C$ as _Parsed___, update as update_1, setParsed, initialModel as initialModel_1, setFailed, Message as Message_1, Model as Model_1 } from "../core/EditBox";
import { errorIfEmpty, parseInt, Result as Result_1, Optional } from "../core/Util";
import { range, map, fold, sortWith, tryFind } from "fable-core/Seq";
import { ResultModule, Result } from "fable-elmish/result";
import { Grid, Button, Form, InputType } from "../core/Bootstrap";
import { CmdModule } from "fable-elmish/elmish";
import { createElement } from "react";
import { trim, fsFormat } from "fable-core/String";
import { ofArray } from "fable-core/List";
import { all } from "../core/Types";
export var Model = function () {
  function Model(kinds, selectedKind, name, universe, address, quantity) {
    _classCallCheck(this, Model);

    this.kinds = kinds;
    this.selectedKind = selectedKind;
    this.name = name;
    this.universe = universe;
    this.address = address;
    this.quantity = quantity;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "NewPatch.Model",
        interfaces: ["FSharpRecord"],
        properties: {
          kinds: _Array(FixtureKind),
          selectedKind: Option(FixtureKind),
          name: makeGeneric(Model_1, {
            T: "string"
          }),
          universe: makeGeneric(Model_1, {
            T: makeGeneric(Optional, {
              T: "number"
            })
          }),
          address: makeGeneric(Model_1, {
            T: makeGeneric(Optional, {
              T: "number"
            })
          }),
          quantity: makeGeneric(Model_1, {
            T: "number"
          })
        }
      };
    }
  }, {
    key: "TryGetNamedKind",
    value: function (name) {
      return tryFind(function (k) {
        return k.name === name;
      }, this.kinds);
    }
  }]);

  return Model;
}();
setType("NewPatch.Model", Model);
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
        type: "NewPatch.Message",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          AddrEdit: [makeGeneric(Message_1, {
            T: makeGeneric(Optional, {
              T: "number"
            })
          })],
          AdvanceAddress: [],
          NameEdit: [makeGeneric(Message_1, {
            T: "string"
          })],
          QuantEdit: [makeGeneric(Message_1, {
            T: "number"
          })],
          SetSelected: ["string"],
          UnivEdit: [makeGeneric(Message_1, {
            T: makeGeneric(Optional, {
              T: "number"
            })
          })],
          UpdateKinds: [_Array(FixtureKind)]
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
setType("NewPatch.Message", Message);

var parsePositiveInt = function parsePositiveInt($var353) {
  return function () {
    var f = function f(number) {
      if (number < 1) {
        return new Result("Error", [null]);
      } else {
        return new Result("Ok", [number]);
      }
    };

    return function (r) {
      return ResultModule.bind(f, r);
    };
  }()(function ($var352) {
    return Result_1.ofOption(parseInt($var352));
  }($var353));
};

export function initialModel() {
  return new Model([], null, setFailed("", initialModel_1("Name:", errorIfEmpty, InputType.Text)), setParsed(new Optional("Absent", []), initialModel_1("Universe:", parseUniverseId, InputType.Number)), setParsed(new Optional("Absent", []), initialModel_1("Address:", parseDmxAddress, InputType.Number)), setParsed(1, initialModel_1("Quantity:", parsePositiveInt, InputType.Number)));
}
export function update(message, model) {
  return function (m) {
    return [m, CmdModule.none()];
  }(message.Case === "SetSelected" ? function () {
    var matchValue = model.TryGetNamedKind(message.Fields[0]);

    if (matchValue == null) {
      return model;
    } else {
      var selectedKind = matchValue;
      return new Model(model.kinds, selectedKind, model.name, model.universe, model.address, model.quantity);
    }
  }() : message.Case === "NameEdit" ? function () {
    var name = update_1(message.Fields[0], model.name);
    return new Model(model.kinds, model.selectedKind, name, model.universe, model.address, model.quantity);
  }() : message.Case === "UnivEdit" ? function () {
    var universe = update_1(message.Fields[0], model.universe);
    return new Model(model.kinds, model.selectedKind, model.name, universe, model.address, model.quantity);
  }() : message.Case === "AddrEdit" ? function () {
    var address = update_1(message.Fields[0], model.address);
    return new Model(model.kinds, model.selectedKind, model.name, model.universe, address, model.quantity);
  }() : message.Case === "QuantEdit" ? function () {
    var quantity = update_1(message.Fields[0], model.quantity);
    return new Model(model.kinds, model.selectedKind, model.name, model.universe, model.address, quantity);
  }() : message.Case === "AdvanceAddress" ? function () {
    var matchValue_1 = [model.address, model.quantity, model.selectedKind];
    var $var354 = void 0;

    var activePatternResult1850 = _Parsed___(matchValue_1[0]);

    if (activePatternResult1850 != null) {
      if (activePatternResult1850.Case === "Present") {
        var activePatternResult1851 = _Parsed___(matchValue_1[1]);

        if (activePatternResult1851 != null) {
          if (matchValue_1[2] != null) {
            $var354 = [0, activePatternResult1850.Fields[0], matchValue_1[2], activePatternResult1851];
          } else {
            $var354 = [1];
          }
        } else {
          $var354 = [1];
        }
      } else {
        $var354 = [1];
      }
    } else {
      $var354 = [1];
    }

    switch ($var354[0]) {
      case 0:
        var newStartAddress = 512 < $var354[1] + $var354[3] * $var354[2].channelCount ? 512 : $var354[1] + $var354[3] * $var354[2].channelCount;
        var address_1 = setParsed(new Optional("Present", [newStartAddress]), model.address);
        return new Model(model.kinds, model.selectedKind, model.name, model.universe, address_1, model.quantity);

      case 1:
        return model;
    }
  }() : function () {
    var sortedKinds = Array.from(sortWith(function (x, y) {
      return compare(function (k) {
        return k.name;
      }(x), function (k) {
        return k.name;
      }(y));
    }, message.Fields[0]));
    return new Model(sortedKinds, sortedKinds.length === 0 ? null : sortedKinds[0], model.name, model.universe, model.address, model.quantity);
  }());
}
export var EnterKey = 13;
export var EscapeKey = 27;

function typeSelector(kinds, selectedKind, dispatchLocal) {
  var option = function option(kind) {
    return createElement("option", {
      value: kind.name
    }, fsFormat("%s (%d ch)")(function (x) {
      return x;
    })(kind.name)(kind.channelCount));
  };

  var selected = selectedKind != null ? selectedKind : kinds[0];
  return createElement("div", {}, createElement.apply(undefined, ["select", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["value", selected.name], ["onChange", function (e_1) {
    dispatchLocal(new Message("SetSelected", [e_1.target.value]));
  }], Form.Control])].concat(_toConsumableArray(ofArray(function (array) {
    return array.map(option);
  }(kinds))))));
}

function newPatchesSequential(name, kind, n, startAddress) {
  var trimmedName = trim(name, "both");
  var name_1 = trimmedName === "" ? kind.name : trimmedName;

  if (n < 1) {
    return new Result("Error", [null]);
  } else if (n === 1) {
    return new Result("Ok", [[new PatchRequest(name_1, kind.name, startAddress)]]);
  } else {
    var makeOne = function makeOne(i) {
      var nameWithCount = fsFormat("%s %d")(function (x) {
        return x;
      })(name_1)(i + 1);
      var addr = defaultArg(startAddress, null, function (tupledArg) {
        return [tupledArg[0], tupledArg[1] + kind.channelCount * i];
      });
      return new PatchRequest(nameWithCount, kind.name, addr);
    };

    return new Result("Ok", [function (array) {
      return Array.from(map(makeOne, array));
    }(Int32Array.from(range(0, n - 1)))]);
  }
}

function patchButton(model, dispatchLocal, dispatchServer) {
  return createElement("button", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [["onClick", function (_arg1_1) {
    fsFormat("%+A")(function (x) {
      console.log(x);
    })(model);
    var matchValue_2 = [model.selectedKind, model.name, model.universe, model.address, model.quantity];
    var $var356 = void 0;

    if (matchValue_2[0] != null) {
      var activePatternResult1869_1 = _Parsed___(matchValue_2[1]);

      if (activePatternResult1869_1 != null) {
        var activePatternResult1870_1 = _Parsed___(matchValue_2[2]);

        if (activePatternResult1870_1 != null) {
          var activePatternResult1871_1 = _Parsed___(matchValue_2[3]);

          if (activePatternResult1871_1 != null) {
            var activePatternResult1872_1 = _Parsed___(matchValue_2[4]);

            if (activePatternResult1872_1 != null) {
              $var356 = [0, activePatternResult1871_1, matchValue_2[0], activePatternResult1869_1, activePatternResult1872_1, activePatternResult1870_1];
            } else {
              $var356 = [1];
            }
          } else {
            $var356 = [1];
          }
        } else {
          $var356 = [1];
        }
      } else {
        $var356 = [1];
      }
    } else {
      $var356 = [1];
    }

    switch ($var356[0]) {
      case 0:
        var matchValue_3 = globalAddressFromOptionals($var356[5], $var356[1]);

        if (matchValue_3.Case === "Ok") {
          fsFormat("Addr: %+A")(function (x) {
            console.log(x);
          })(matchValue_3.Fields[0]);
          var newPatchResult_1 = newPatchesSequential($var356[3], $var356[2], $var356[4], matchValue_3.Fields[0]);

          if (newPatchResult_1.Case === "Ok") {
            dispatchServer(all(new PatchServerRequest("NewPatches", [newPatchResult_1.Fields[0]])));
            dispatchLocal(new Message("AdvanceAddress", []));
          }
        }

        break;

      case 1:
        fsFormat("%+A")(function (x) {
          console.log(x);
        })(matchValue_2);
        break;
    }
  }], Button.Primary]), "Patch");
}

export function view(model, dispatchLocal, dispatchServer) {
  if (model.kinds.length === 0) {
    return createElement("div", {}, "No patch types available.");
  } else {
    var nameEntry = view_1(null, "", model.name, function ($var357) {
      return dispatchLocal(function (arg0) {
        return new Message("NameEdit", [arg0]);
      }($var357));
    });
    var universeEntry = view_1(null, "", model.universe, function ($var358) {
      return dispatchLocal(function (arg0_1) {
        return new Message("UnivEdit", [arg0_1]);
      }($var358));
    });
    var addressEntry = view_1(null, "", model.address, function ($var359) {
      return dispatchLocal(function (arg0_2) {
        return new Message("AddrEdit", [arg0_2]);
      }($var359));
    });
    var quantityEntry = view_1(null, "", model.quantity, function ($var360) {
      return dispatchLocal(function (arg0_3) {
        return new Message("QuantEdit", [arg0_3]);
      }($var360));
    });
    return createElement("div", fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [Form.Group]), createElement("span", {}, createElement("h4", {}, "Create new patch")), typeSelector(model.kinds, model.selectedKind, dispatchLocal), nameEntry, Grid.distribute(ofArray([ofArray([universeEntry]), ofArray([addressEntry])])), Grid.distribute(ofArray([ofArray([quantityEntry]), ofArray([patchButton(model, dispatchLocal, dispatchServer)])])));
  }
}