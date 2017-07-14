var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { makeGeneric, Tuple, compareUnions, equalsUnions } from "fable-core/Util";
import { view as view_4, update as update_1, initCommands as initCommands_2, initialModel, Message as Message_2, Model } from "./patcher/Patcher";
import _Map from "fable-core/Map";
import { KnobAddress } from "./core/DataflowTypes";
import { Model as Model_1 } from "./core/Knob";
import { view as view_2, update as update_3, initCommands as initCommands_4, initModel as initModel_2, Message, Model as Model_2 } from "./core/Clocks";
import { view as view_3, update as update_4, initCommands as initCommands_5, initModel as initModel_3, Message as Message_3, Model as Model_3 } from "./core/Wiggles";
import { Response, Command } from "./core/ClockTypes";
import { viewOne, update as update_2, initCommands as initCommands_3, initModel as initModel_1, Message as Message_1, ServerResponse, ServerCommand } from "./core/Knobs";
import { PatchServerResponse, PatchServerRequest } from "./patcher/PatchTypes";
import { Response as Response_1, Command as Command_1 } from "./core/WiggleTypes";
import { Position, NavItem, Model as Model_4, Item } from "./core/Navbar";
import { view as view_5, update as update_5, liftResponseAndFilter, initCommands as initCommands_1, initModel as initModel_4, utilDropdown, Message as Message_4 } from "./core/Base";
import { concat, map, ofArray } from "fable-core/List";
import { ProgramModule, CmdModule } from "fable-elmish/elmish";
import { ServerResponse as ServerResponse_1, ServerCommand as ServerCommand_1, exclusive } from "./core/Types";
import { prompt, Message as Message_5 } from "./core/Modal";
import { view as view_1 } from "./patcher/Controls";
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
          Controls: [],
          KnobTest: [],
          Patcher: [],
          WiggleTest: []
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
  function ShowModel(page, patcher, knobs, clocks, wiggles) {
    _classCallCheck(this, ShowModel);

    this.page = page;
    this.patcher = patcher;
    this.knobs = knobs;
    this.clocks = clocks;
    this.wiggles = wiggles;
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
            s: Tuple([Tuple(["number", "number"]), "number"])
          }),
          knobs: makeGeneric(_Map, {
            Key: KnobAddress,
            Value: Model_1
          }),
          clocks: Model_2,
          wiggles: Model_3
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
            s: Tuple([Tuple(["number", "number"]), "number"])
          })],
          Wiggle: [Command_1]
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
            s: Tuple([Tuple(["number", "number"]), "number"])
          })],
          Wiggle: [Response_1]
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
            s: Tuple([Tuple(["number", "number"]), "number"])
          })],
          SetPage: [Page],
          Wiggle: [Message_3]
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
export function pageNavItem(name, page) {
  return new Item(name, function (dispatch) {
    dispatch(new Message_4("Inner", [new ShowMessage("SetPage", [page])]));
  });
}
export var navbar = new Model_4(ofArray([new NavItem("Dropdown", [utilDropdown()]), new NavItem("Single", [pageNavItem("Knobs", new Page("KnobTest", []))]), new NavItem("Single", [pageNavItem("Clocks", new Page("ClockTest", []))]), new NavItem("Single", [pageNavItem("Wiggles", new Page("WiggleTest", []))])]), ofArray([new NavItem("Single", [pageNavItem("Controls", new Page("Controls", []))]), new NavItem("Single", [pageNavItem("Patch", new Page("Patcher", []))])]), new Position("Right", [1]));
export function initShowModel() {
  return new ShowModel(new Page("Patcher", []), initialModel(), initModel_1(), initModel_2(), initModel_3());
}
export function initModel() {
  return [initModel_4(navbar, initShowModel()), CmdModule.none()];
}
export var initCommands = CmdModule.batch(map(function (msg) {
  return CmdModule.ofMsg(msg);
}, map(function (message) {
  return exclusive(message);
}, concat(ofArray([initCommands_1(), map(function ($var400) {
  return new ServerCommand_1("Console", [new ShowServerCommand("Patcher", [$var400])]);
}, initCommands_2()), map(function ($var401) {
  return new ServerCommand_1("Console", [new ShowServerCommand("Knob", [$var401])]);
}, initCommands_3()), map(function ($var402) {
  return new ServerCommand_1("Console", [new ShowServerCommand("Clock", [$var402])]);
}, initCommands_4), map(function ($var403) {
  return new ServerCommand_1("Console", [new ShowServerCommand("Wiggle", [$var403])]);
}, initCommands_5)])))));
export function wrapShowResponse(message) {
  if (message.Case === "Patcher") {
    return new ShowMessage("Patcher", [new Message_2("Response", [message.Fields[0]])]);
  } else if (message.Case === "Knob") {
    return new ShowMessage("Knob", [new Message_1("Response", [message.Fields[0]])]);
  } else if (message.Case === "Clock") {
    return new ShowMessage("Clock", [new Message("Response", [message.Fields[0]])]);
  } else if (message.Case === "Wiggle") {
    return new ShowMessage("Wiggle", [new Message_3("Response", [message.Fields[0]])]);
  } else {
    return new ShowMessage("Error", [message.Fields[0]]);
  }
}
export function updateShow(message, model) {
  if (message.Case === "SetPage") {
    return [new ShowModel(message.Fields[0], model.patcher, model.knobs, model.clocks, model.wiggles), CmdModule.none()];
  } else if (message.Case === "Patcher") {
    var patternInput = update_1(message.Fields[0], model.patcher);
    return [new ShowModel(model.page, patternInput[0], model.knobs, model.clocks, model.wiggles), CmdModule.map(function ($var404) {
      return new Message_4("Inner", [new ShowMessage("Patcher", [$var404])]);
    }, patternInput[1])];
  } else if (message.Case === "Knob") {
    var updatedKnobs = update_2(message.Fields[0], model.knobs);
    return [new ShowModel(model.page, model.patcher, updatedKnobs, model.clocks, model.wiggles), CmdModule.none()];
  } else if (message.Case === "Clock") {
    var updatedClocks = update_3(message.Fields[0], model.clocks);
    return [new ShowModel(model.page, model.patcher, model.knobs, updatedClocks, model.wiggles), CmdModule.none()];
  } else if (message.Case === "Wiggle") {
    var updatedWiggles = update_4(message.Fields[0], model.wiggles);
    return [new ShowModel(model.page, model.patcher, model.knobs, model.clocks, updatedWiggles), CmdModule.none()];
  } else {
    return [model, CmdModule.ofMsg(new Message_4("Modal", [new Message_5("Open", [prompt(message.Fields[0])])]))];
  }
}
export function viewShow(openModal, model, dispatch, dispatchServer) {
  if (model.page.Case === "Controls") {
    return view_1(model.patcher.patches, model.wiggles.wiggles, function ($var405) {
      return dispatchServer(function () {
        var f = function f(arg0) {
          return new ShowServerCommand("Patcher", [arg0]);
        };

        return function (tupledArg) {
          return liftResponseAndFilter(f, tupledArg[0], tupledArg[1]);
        };
      }()($var405));
    });
  } else if (model.page.Case === "KnobTest") {
    var knobs = toList(map_1(function (tupledArg_1) {
      return viewOne(tupledArg_1[0], tupledArg_1[1], function ($var406) {
        return dispatch(function (arg0_1) {
          return new ShowMessage("Knob", [arg0_1]);
        }($var406));
      }, function ($var407) {
        return dispatchServer(function () {
          var f_1 = function f_1(arg0_2) {
            return new ShowServerCommand("Knob", [arg0_2]);
          };

          return function (tupledArg_2) {
            return liftResponseAndFilter(f_1, tupledArg_2[0], tupledArg_2[1]);
          };
        }()($var407));
      });
    }, model.knobs));
    return createElement.apply(undefined, ["div", {}].concat(_toConsumableArray(knobs)));
  } else if (model.page.Case === "ClockTest") {
    return view_2(model.knobs, model.clocks, function ($var408) {
      return dispatch(function (arg0_3) {
        return new ShowMessage("Knob", [arg0_3]);
      }($var408));
    }, function ($var409) {
      return dispatch(function (arg0_4) {
        return new ShowMessage("Clock", [arg0_4]);
      }($var409));
    }, function ($var410) {
      return dispatchServer(function () {
        var f_2 = function f_2(arg0_5) {
          return new ShowServerCommand("Knob", [arg0_5]);
        };

        return function (tupledArg_3) {
          return liftResponseAndFilter(f_2, tupledArg_3[0], tupledArg_3[1]);
        };
      }()($var410));
    }, function ($var411) {
      return dispatchServer(function () {
        var f_3 = function f_3(arg0_6) {
          return new ShowServerCommand("Clock", [arg0_6]);
        };

        return function (tupledArg_4) {
          return liftResponseAndFilter(f_3, tupledArg_4[0], tupledArg_4[1]);
        };
      }()($var411));
    });
  } else if (model.page.Case === "WiggleTest") {
    return view_3(model.knobs, model.clocks.clocks, model.wiggles, function ($var412) {
      return dispatch(function (arg0_7) {
        return new ShowMessage("Knob", [arg0_7]);
      }($var412));
    }, function ($var413) {
      return dispatch(function (arg0_8) {
        return new ShowMessage("Wiggle", [arg0_8]);
      }($var413));
    }, function ($var414) {
      return dispatchServer(function () {
        var f_4 = function f_4(arg0_9) {
          return new ShowServerCommand("Knob", [arg0_9]);
        };

        return function (tupledArg_5) {
          return liftResponseAndFilter(f_4, tupledArg_5[0], tupledArg_5[1]);
        };
      }()($var414));
    }, function ($var415) {
      return dispatchServer(function () {
        var f_5 = function f_5(arg0_10) {
          return new ShowServerCommand("Wiggle", [arg0_10]);
        };

        return function (tupledArg_6) {
          return liftResponseAndFilter(f_5, tupledArg_6[0], tupledArg_6[1]);
        };
      }()($var415));
    });
  } else {
    return view_4(openModal, model.patcher, function ($var416) {
      return dispatch(function (arg0_11) {
        return new ShowMessage("Patcher", [arg0_11]);
      }($var416));
    }, function ($var417) {
      return dispatchServer(function () {
        var f_6 = function f_6(arg0_12) {
          return new ShowServerCommand("Patcher", [arg0_12]);
        };

        return function (tupledArg_7) {
          return liftResponseAndFilter(f_6, tupledArg_7[0], tupledArg_7[1]);
        };
      }()($var417));
    });
  }
}
var patternInput_195 = openSocket(logSocketTraffic, function (arg0) {
  return new Message_4("Socket", [arg0]);
}, {
  rsp: makeGeneric(ServerResponse_1, {
    msg: ShowServerResponse
  }),
  msg: makeGeneric(Message_4, {
    cmd: ShowServerCommand,
    rsp: ShowServerResponse,
    msg: ShowMessage
  })
});
export var subscription = patternInput_195[0];
export var send = patternInput_195[1];
export function update(msg, model) {
  return update_5(initCommands, function (arg00) {
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
  return view_5(function (openModal) {
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
  return new Message_4("Response", [arg0]);
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