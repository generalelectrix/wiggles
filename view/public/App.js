var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { makeGeneric, Tuple, compareUnions, equalsUnions } from "fable-core/Util";
import { view as view_2, update as update_1, initCommands as initCommands_2, initialModel, Message as Message_2, Model } from "./patcher/Patcher";
import _Map from "fable-core/Map";
import { KnobAddress } from "./core/DataflowTypes";
import { Model as Model_1 } from "./core/Knob";
import { view as view_1, update as update_3, initCommands as initCommands_4, initModel as initModel_2, Message, Model as Model_2 } from "./core/Clocks";
import { Response, Command } from "./core/ClockTypes";
import { viewOne, update as update_2, initCommands as initCommands_3, initModel as initModel_1, Message as Message_1, ServerResponse, ServerCommand } from "./core/Knobs";
import { PatchServerResponse, PatchServerRequest } from "./patcher/PatchTypes";
import { Position, NavItem, Model as Model_3, Item } from "./core/Navbar";
import { view as view_3, update as update_4, liftResponseAndFilter, initCommands as initCommands_1, initModel as initModel_3, utilDropdown, Message as Message_3 } from "./core/Base";
import { concat, map, ofArray } from "fable-core/List";
import { ProgramModule, CmdModule } from "fable-elmish/elmish";
import { ServerResponse as ServerResponse_1, ServerCommand as ServerCommand_1, exclusive } from "./core/Types";
import { prompt, Message as Message_4 } from "./core/Modal";
import { map as map_1, toList } from "fable-core/Seq";
import { createElement } from "react";
import { openSocket } from "./core/Socket";
import { withReact } from "fable-elmish-react/react";
export var withConsoleTrace = false;
export var logSocketTraffic = false;
export var Page = function () {
  function Page(caseName, fields) {
    _classCallCheck(this, Page);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(Page, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "App.Page",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          ClockTest: [],
          KnobTest: [],
          Patcher: []
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

  return Page;
}();
setType("App.Page", Page);
export var ShowModel = function () {
  function ShowModel(page, patcher, knobs, clocks) {
    _classCallCheck(this, ShowModel);

    this.page = page;
    this.patcher = patcher;
    this.knobs = knobs;
    this.clocks = clocks;
  }

  _createClass(ShowModel, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "App.ShowModel",
        interfaces: ["FSharpRecord"],
        properties: {
          page: Page,
          patcher: makeGeneric(Model, {
            s: Tuple(["number", "number"])
          }),
          knobs: makeGeneric(_Map, {
            Key: KnobAddress,
            Value: Model_1
          }),
          clocks: Model_2
        }
      };
    }
  }]);

  return ShowModel;
}();
setType("App.ShowModel", ShowModel);
export var ShowServerCommand = function () {
  function ShowServerCommand(caseName, fields) {
    _classCallCheck(this, ShowServerCommand);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ShowServerCommand, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "App.ShowServerCommand",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Clock: [Command],
          Knob: [makeGeneric(ServerCommand, {
            a: KnobAddress
          })],
          Patcher: [makeGeneric(PatchServerRequest, {
            s: Tuple(["number", "number"])
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

  return ShowServerCommand;
}();
setType("App.ShowServerCommand", ShowServerCommand);
export var ShowServerResponse = function () {
  function ShowServerResponse(caseName, fields) {
    _classCallCheck(this, ShowServerResponse);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ShowServerResponse, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "App.ShowServerResponse",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Clock: [Response],
          Error: ["string"],
          Knob: [makeGeneric(ServerResponse, {
            a: KnobAddress
          })],
          Patcher: [makeGeneric(PatchServerResponse, {
            s: Tuple(["number", "number"])
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

  return ShowServerResponse;
}();
setType("App.ShowServerResponse", ShowServerResponse);
export var ShowMessage = function () {
  function ShowMessage(caseName, fields) {
    _classCallCheck(this, ShowMessage);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ShowMessage, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "App.ShowMessage",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Clock: [Message],
          Error: ["string"],
          Knob: [makeGeneric(Message_1, {
            a: KnobAddress
          })],
          Patcher: [makeGeneric(Message_2, {
            s: Tuple(["number", "number"])
          })],
          SetPage: [Page]
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

  return ShowMessage;
}();
setType("App.ShowMessage", ShowMessage);
export var patcherNavItem = new Item("Patch", function (dispatch) {
  dispatch(new Message_3("Inner", [new ShowMessage("SetPage", [new Page("Patcher", [])])]));
});
export var knobTestNavItem = new Item("Knobs", function (dispatch) {
  dispatch(new Message_3("Inner", [new ShowMessage("SetPage", [new Page("KnobTest", [])])]));
});
export var clockTestNavItem = new Item("Clocks", function (dispatch) {
  dispatch(new Message_3("Inner", [new ShowMessage("SetPage", [new Page("ClockTest", [])])]));
});
export var navbar = new Model_3(ofArray([new NavItem("Dropdown", [utilDropdown()]), new NavItem("Single", [knobTestNavItem]), new NavItem("Single", [clockTestNavItem])]), ofArray([new NavItem("Single", [patcherNavItem])]), new Position("Right", [0]));
export function initShowModel() {
  return new ShowModel(new Page("Patcher", []), initialModel(), initModel_1(), initModel_2());
}
export function initModel() {
  return [initModel_3(navbar, initShowModel()), CmdModule.none()];
}
export var initCommands = CmdModule.batch(map(function (msg) {
  return CmdModule.ofMsg(msg);
}, map(function (message) {
  return exclusive(message);
}, concat(ofArray([initCommands_1(), map(function ($var337) {
  return new ServerCommand_1("Console", [new ShowServerCommand("Patcher", [$var337])]);
}, initCommands_2()), map(function ($var338) {
  return new ServerCommand_1("Console", [new ShowServerCommand("Knob", [$var338])]);
}, initCommands_3()), map(function ($var339) {
  return new ServerCommand_1("Console", [new ShowServerCommand("Clock", [$var339])]);
}, initCommands_4)])))));
export function wrapShowResponse(message) {
  if (message.Case === "Patcher") {
    return new ShowMessage("Patcher", [new Message_2("Response", [message.Fields[0]])]);
  } else if (message.Case === "Knob") {
    return new ShowMessage("Knob", [new Message_1("Response", [message.Fields[0]])]);
  } else if (message.Case === "Clock") {
    return new ShowMessage("Clock", [new Message("Response", [message.Fields[0]])]);
  } else {
    return new ShowMessage("Error", [message.Fields[0]]);
  }
}
export function updateShow(message, model) {
  if (message.Case === "SetPage") {
    return [new ShowModel(message.Fields[0], model.patcher, model.knobs, model.clocks), CmdModule.none()];
  } else if (message.Case === "Patcher") {
    var patternInput = update_1(message.Fields[0], model.patcher);
    return [new ShowModel(model.page, patternInput[0], model.knobs, model.clocks), CmdModule.map(function ($var340) {
      return new Message_3("Inner", [new ShowMessage("Patcher", [$var340])]);
    }, patternInput[1])];
  } else if (message.Case === "Knob") {
    var updatedKnobs = update_2(message.Fields[0], model.knobs);
    return [new ShowModel(model.page, model.patcher, updatedKnobs, model.clocks), CmdModule.none()];
  } else if (message.Case === "Clock") {
    var updatedClocks = update_3(message.Fields[0], model.clocks);
    return [new ShowModel(model.page, model.patcher, model.knobs, updatedClocks), CmdModule.none()];
  } else {
    return [model, CmdModule.ofMsg(new Message_3("Modal", [new Message_4("Open", [prompt(message.Fields[0])])]))];
  }
}
export function viewShow(openModal, model, dispatch, dispatchServer) {
  if (model.page.Case === "KnobTest") {
    var knobs = toList(map_1(function (tupledArg) {
      return viewOne(tupledArg[0], tupledArg[1], function ($var341) {
        return dispatch(function (arg0) {
          return new ShowMessage("Knob", [arg0]);
        }($var341));
      }, function ($var342) {
        return dispatchServer(function () {
          var f = function f(arg0_1) {
            return new ShowServerCommand("Knob", [arg0_1]);
          };

          return function (tupledArg_1) {
            return liftResponseAndFilter(f, tupledArg_1[0], tupledArg_1[1]);
          };
        }()($var342));
      });
    }, model.knobs));
    return createElement.apply(undefined, ["div", {}].concat(_toConsumableArray(knobs)));
  } else if (model.page.Case === "ClockTest") {
    return view_1(model.knobs, model.clocks, function ($var343) {
      return dispatch(function (arg0_2) {
        return new ShowMessage("Knob", [arg0_2]);
      }($var343));
    }, function ($var344) {
      return dispatch(function (arg0_3) {
        return new ShowMessage("Clock", [arg0_3]);
      }($var344));
    }, function ($var345) {
      return dispatchServer(function () {
        var f_1 = function f_1(arg0_4) {
          return new ShowServerCommand("Knob", [arg0_4]);
        };

        return function (tupledArg_2) {
          return liftResponseAndFilter(f_1, tupledArg_2[0], tupledArg_2[1]);
        };
      }()($var345));
    }, function ($var346) {
      return dispatchServer(function () {
        var f_2 = function f_2(arg0_5) {
          return new ShowServerCommand("Clock", [arg0_5]);
        };

        return function (tupledArg_3) {
          return liftResponseAndFilter(f_2, tupledArg_3[0], tupledArg_3[1]);
        };
      }()($var346));
    });
  } else {
    return view_2(openModal, model.patcher, function ($var347) {
      return dispatch(function (arg0_6) {
        return new ShowMessage("Patcher", [arg0_6]);
      }($var347));
    }, function ($var348) {
      return dispatchServer(function () {
        var f_3 = function f_3(arg0_7) {
          return new ShowServerCommand("Patcher", [arg0_7]);
        };

        return function (tupledArg_4) {
          return liftResponseAndFilter(f_3, tupledArg_4[0], tupledArg_4[1]);
        };
      }()($var348));
    });
  }
}
var patternInput_177 = openSocket(logSocketTraffic, function (arg0) {
  return new Message_3("Socket", [arg0]);
}, {
  rsp: makeGeneric(ServerResponse_1, {
    msg: ShowServerResponse
  }),
  msg: makeGeneric(Message_3, {
    cmd: ShowServerCommand,
    rsp: ShowServerResponse,
    msg: ShowMessage
  })
});
export var subscription = patternInput_177[0];
export var send = patternInput_177[1];
export function update(msg, model) {
  return update_4(initCommands, function (arg00) {
    send(arg00);
  }, function (message) {
    return wrapShowResponse(message);
  }, function (message_1) {
    return function (model_1) {
      return updateShow(message_1, model_1);
    };
  }, msg, model);
}
export function view(model, dispatch) {
  return view_3(function (openModal) {
    return function (model_1) {
      return function (dispatch_1) {
        return function (dispatchServer) {
          return viewShow(openModal, model_1, dispatch_1, dispatchServer);
        };
      };
    };
  }, model, dispatch);
}
ProgramModule.run((withConsoleTrace ? function (program) {
  return ProgramModule.withConsoleTrace(program);
} : function (x) {
  return x;
})(withReact("app", ProgramModule.withSubscription(function (arg00) {
  var clo1 = subscription(arg00);
  return function (arg10) {
    return clo1(arg10);
  };
}(function (arg0) {
  return new Message_3("Response", [arg0]);
}), ProgramModule.mkProgram(function () {
  return initModel();
}, function (msg) {
  return function (model) {
    return update(msg, model);
  };
}, function (model_1) {
  return function (dispatch) {
    return view(model_1, dispatch);
  };
})))));