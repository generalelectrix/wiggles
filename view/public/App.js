var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { makeGeneric, compareUnions, equalsUnions } from "fable-core/Util";
import { view as view_1, update as update_1, initCommands as initCommands_2, initialModel, Message as Message_1, Model } from "./patcher/Patcher";
import _Map from "fable-core/Map";
import { Model as Model_1 } from "./core/Knob";
import { viewOne, update as update_2, initModel as initModel_1, Message, ServerResponse, ServerCommand } from "./core/Knobs";
import { PatchServerResponse, PatchServerRequest } from "./patcher/PatchTypes";
import { Position, NavItem, Model as Model_2, Item } from "./core/Navbar";
import { view as view_2, update as update_3, liftResponseAndFilter, initCommands as initCommands_1, initModel as initModel_2, utilDropdown, Message as Message_2 } from "./core/Base";
import { concat, map, ofArray } from "fable-core/List";
import { ProgramModule, CmdModule } from "fable-elmish/elmish";
import { ServerResponse as ServerResponse_1, ServerCommand as ServerCommand_1, exclusive } from "./core/Types";
import { prompt, Message as Message_3 } from "./core/Modal";
import { map as map_1, toList } from "fable-core/Seq";
import { createElement } from "react";
import { openSocket } from "./core/Socket";
import { withReact } from "fable-elmish-react/react";
export var withConsoleTrace = true;
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
  function ShowModel(page, patcher, knobs) {
    _classCallCheck(this, ShowModel);

    this.page = page;
    this.patcher = patcher;
    this.knobs = knobs;
  }

  _createClass(ShowModel, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "App.ShowModel",
        interfaces: ["FSharpRecord"],
        properties: {
          page: Page,
          patcher: Model,
          knobs: makeGeneric(_Map, {
            Key: "number",
            Value: Model_1
          })
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
          Knob: [makeGeneric(ServerCommand, {
            a: "number"
          })],
          Patcher: [PatchServerRequest]
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
          Error: ["string"],
          Knob: [makeGeneric(ServerResponse, {
            a: "number"
          })],
          Patcher: [PatchServerResponse]
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
          Error: ["string"],
          Knob: [makeGeneric(Message, {
            a: "number"
          })],
          Patcher: [Message_1],
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
  dispatch(new Message_2("Inner", [new ShowMessage("SetPage", [new Page("Patcher", [])])]));
});
export var knobTestNavItem = new Item("Knobs", function (dispatch) {
  dispatch(new Message_2("Inner", [new ShowMessage("SetPage", [new Page("KnobTest", [])])]));
});
export var navbar = new Model_2(ofArray([new NavItem("Dropdown", [utilDropdown()]), new NavItem("Single", [knobTestNavItem])]), ofArray([new NavItem("Single", [patcherNavItem])]), new Position("Right", [0]));
export function initShowModel() {
  return new ShowModel(new Page("Patcher", []), initialModel(), initModel_1());
}
export function initModel() {
  return [initModel_2(navbar, initShowModel()), CmdModule.none()];
}
export var initCommands = CmdModule.batch(map(function (msg) {
  return CmdModule.ofMsg(msg);
}, map(function (message) {
  return exclusive(message);
}, concat(ofArray([initCommands_1(), map(function ($var288) {
  return new ServerCommand_1("Console", [new ShowServerCommand("Patcher", [$var288])]);
}, initCommands_2)])))));
export function wrapShowResponse(message) {
  if (message.Case === "Patcher") {
    return new ShowMessage("Patcher", [new Message_1("Response", [message.Fields[0]])]);
  } else if (message.Case === "Knob") {
    return new ShowMessage("Knob", [new Message("Response", [message.Fields[0]])]);
  } else {
    return new ShowMessage("Error", [message.Fields[0]]);
  }
}
export function updateShow(message, model) {
  if (message.Case === "SetPage") {
    return [new ShowModel(message.Fields[0], model.patcher, model.knobs), CmdModule.none()];
  } else if (message.Case === "Patcher") {
    var patternInput = update_1(message.Fields[0], model.patcher);
    return [new ShowModel(model.page, patternInput[0], model.knobs), CmdModule.map(function ($var289) {
      return new Message_2("Inner", [new ShowMessage("Patcher", [$var289])]);
    }, patternInput[1])];
  } else if (message.Case === "Knob") {
    var updatedKnobs = update_2(message.Fields[0], model.knobs);
    return [new ShowModel(model.page, model.patcher, updatedKnobs), CmdModule.none()];
  } else {
    return [model, CmdModule.ofMsg(new Message_2("Modal", [new Message_3("Open", [prompt(message.Fields[0])])]))];
  }
}
export function viewShow(openModal, model, dispatch, dispatchServer) {
  if (model.page.Case === "KnobTest") {
    var knobs = toList(map_1(function (tupledArg) {
      return viewOne(tupledArg[0], tupledArg[1], function ($var290) {
        return dispatch(function (arg0) {
          return new ShowMessage("Knob", [arg0]);
        }($var290));
      }, function ($var291) {
        return dispatchServer(function () {
          var f = function f(arg0_1) {
            return new ShowServerCommand("Knob", [arg0_1]);
          };

          return function (tupledArg_1) {
            return liftResponseAndFilter(f, tupledArg_1[0], tupledArg_1[1]);
          };
        }()($var291));
      });
    }, model.knobs));
    return createElement.apply(undefined, ["div", {}].concat(_toConsumableArray(knobs)));
  } else {
    return view_1(openModal, model.patcher, function ($var292) {
      return dispatch(function (arg0_2) {
        return new ShowMessage("Patcher", [arg0_2]);
      }($var292));
    }, function ($var293) {
      return dispatchServer(function () {
        var f_1 = function f_1(arg0_3) {
          return new ShowServerCommand("Patcher", [arg0_3]);
        };

        return function (tupledArg_2) {
          return liftResponseAndFilter(f_1, tupledArg_2[0], tupledArg_2[1]);
        };
      }()($var293));
    });
  }
}
var patternInput_141 = openSocket(function (arg0) {
  return new Message_2("Socket", [arg0]);
}, {
  rsp: makeGeneric(ServerResponse_1, {
    msg: ShowServerResponse
  }),
  msg: makeGeneric(Message_2, {
    cmd: ShowServerCommand,
    rsp: ShowServerResponse,
    msg: ShowMessage
  })
});
export var subscription = patternInput_141[0];
export var send = patternInput_141[1];
export function update(msg, model) {
  return update_3(initCommands, function (arg00) {
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
  return view_2(function (openModal) {
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
  return new Message_2("Response", [arg0]);
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