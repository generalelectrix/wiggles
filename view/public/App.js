var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { makeGeneric, compareUnions, equalsUnions } from "fable-core/Util";
import { view as view_1, update as update_1, initCommands as initCommands_2, initialModel, Message, Model } from "./patcher/Patcher";
import { PatchServerResponse, PatchServerRequest } from "./patcher/PatchTypes";
import { Position, NavItem, Model as Model_1, Item } from "./core/Navbar";
import { view as view_2, update as update_2, liftResponseAndFilter, initCommands as initCommands_1, initModel as initModel_1, utilDropdown, Message as Message_1 } from "./core/Base";
import { concat, map, ofArray } from "fable-core/List";
import { ProgramModule, CmdModule } from "fable-elmish/elmish";
import { ServerResponse, ServerCommand, exclusive } from "./core/Types";
import { prompt, Message as Message_2 } from "./core/Modal";
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
  function ShowModel(page, patcher) {
    _classCallCheck(this, ShowModel);

    this.page = page;
    this.patcher = patcher;
  }

  _createClass(ShowModel, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "App.ShowModel",
        interfaces: ["FSharpRecord"],
        properties: {
          page: Page,
          patcher: Model
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
          Patcher: [Message],
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
  dispatch(new Message_1("Inner", [new ShowMessage("SetPage", [new Page("Patcher", [])])]));
});
export var navbar = new Model_1(ofArray([new NavItem("Dropdown", [utilDropdown()])]), ofArray([new NavItem("Single", [patcherNavItem])]), new Position("Right", [0]));
export function initShowModel() {
  return new ShowModel(new Page("Patcher", []), initialModel());
}
export function initModel() {
  return [initModel_1(navbar, initShowModel()), CmdModule.none()];
}
export var initCommands = CmdModule.batch(map(function (msg) {
  return CmdModule.ofMsg(msg);
}, map(function (message) {
  return exclusive(message);
}, concat(ofArray([initCommands_1(), map(function ($var288) {
  return new ServerCommand("Console", [new ShowServerCommand("Patcher", [$var288])]);
}, initCommands_2)])))));
export function wrapShowResponse(message) {
  if (message.Case === "Patcher") {
    return new ShowMessage("Patcher", [new Message("Response", [message.Fields[0]])]);
  } else {
    return new ShowMessage("Error", [message.Fields[0]]);
  }
}
export function updateShow(message, model) {
  if (message.Case === "SetPage") {
    return [new ShowModel(message.Fields[0], model.patcher), CmdModule.none()];
  } else if (message.Case === "Patcher") {
    var patternInput = update_1(message.Fields[0], model.patcher);
    return [new ShowModel(model.page, patternInput[0]), CmdModule.map(function ($var289) {
      return new Message_1("Inner", [new ShowMessage("Patcher", [$var289])]);
    }, patternInput[1])];
  } else {
    return [model, CmdModule.ofMsg(new Message_1("Modal", [new Message_2("Open", [prompt(message.Fields[0])])]))];
  }
}
export function viewShow(openModal, model, dispatch, dispatchServer) {
  return view_1(openModal, model.patcher, function ($var290) {
    return dispatch(function (arg0) {
      return new ShowMessage("Patcher", [arg0]);
    }($var290));
  }, function ($var291) {
    return dispatchServer(function () {
      var f = function f(arg0_1) {
        return new ShowServerCommand("Patcher", [arg0_1]);
      };

      return function (tupledArg) {
        return liftResponseAndFilter(f, tupledArg[0], tupledArg[1]);
      };
    }()($var291));
  });
}
var patternInput_111 = openSocket(function (arg0) {
  return new Message_1("Socket", [arg0]);
}, {
  rsp: makeGeneric(ServerResponse, {
    msg: ShowServerResponse
  }),
  msg: makeGeneric(Message_1, {
    cmd: ShowServerCommand,
    rsp: ShowServerResponse,
    msg: ShowMessage
  })
});
export var subscription = patternInput_111[0];
export var send = patternInput_111[1];
export function update(msg, model) {
  return update_2(initCommands, function (arg00) {
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
  return new Message_1("Response", [arg0]);
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