var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { GenericParam, Array as _Array, compareRecords, equalsRecords, makeGeneric, compareUnions, equalsUnions } from "fable-core/Util";
import { ofArray } from "fable-core/List";
import List from "fable-core/List";
import { viewSplash, view as view_2, update as update_2, prompt as prompt_1, Message as Message_1, initialModel, ModalRequest } from "./Modal";
import { view as view_1, update as update_1, Message as Message_2, Model as Model_1 } from "./Navbar";
import { SocketMessage } from "./Socket";
import { CmdModule } from "fable-elmish/elmish";
import { fsFormat } from "fable-core/String";
import { createElement } from "react";
import { fold } from "fable-core/Seq";
import { Container } from "./Bootstrap";
export var ConnectionState = function () {
  function ConnectionState(caseName, fields) {
    _classCallCheck(this, ConnectionState);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ConnectionState, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WigglesBase.ConnectionState",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Closed: [],
          Open: [],
          Waiting: []
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

  return ConnectionState;
}();
setType("WigglesBase.ConnectionState", ConnectionState);
export var SavesAvailable = function () {
  function SavesAvailable(saves, autosaves) {
    _classCallCheck(this, SavesAvailable);

    this.saves = saves;
    this.autosaves = autosaves;
  }

  _createClass(SavesAvailable, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WigglesBase.SavesAvailable",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          saves: makeGeneric(List, {
            T: "string"
          }),
          autosaves: makeGeneric(List, {
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

  return SavesAvailable;
}();
setType("WigglesBase.SavesAvailable", SavesAvailable);
export var BaseModel = function () {
  function BaseModel(name, savesAvailable, showsAvailable, modalDialog, navbar) {
    _classCallCheck(this, BaseModel);

    this.name = name;
    this.savesAvailable = savesAvailable;
    this.showsAvailable = showsAvailable;
    this.modalDialog = modalDialog;
    this.navbar = navbar;
  }

  _createClass(BaseModel, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WigglesBase.BaseModel",
        interfaces: ["FSharpRecord"],
        properties: {
          name: "string",
          savesAvailable: SavesAvailable,
          showsAvailable: makeGeneric(List, {
            T: "string"
          }),
          modalDialog: _Array(ModalRequest),
          navbar: makeGeneric(Model_1, {
            msg: GenericParam("msg")
          })
        }
      };
    }
  }]);

  return BaseModel;
}();
setType("WigglesBase.BaseModel", BaseModel);

function initBaseModel(navbar) {
  return new BaseModel("", new SavesAvailable(new List(), new List()), new List(), initialModel(), navbar);
}

export var Model = function () {
  function Model(connection, baseModel, showModel) {
    _classCallCheck(this, Model);

    this.connection = connection;
    this.baseModel = baseModel;
    this.showModel = showModel;
  }

  _createClass(Model, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WigglesBase.Model",
        interfaces: ["FSharpRecord"],
        properties: {
          connection: ConnectionState,
          baseModel: makeGeneric(BaseModel, {
            msg: GenericParam("msg")
          }),
          showModel: GenericParam("m")
        }
      };
    }
  }]);

  return Model;
}();
setType("WigglesBase.Model", Model);
export function initModel(navbar, showModel) {
  return new Model(new ConnectionState("Waiting", []), initBaseModel(navbar), showModel);
}
export var LoadSpec = function () {
  function LoadSpec(caseName, fields) {
    _classCallCheck(this, LoadSpec);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(LoadSpec, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WigglesBase.LoadSpec",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Exact: ["string"],
          ExactAutosave: ["string"],
          Latest: [],
          LatestAutosave: []
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

  return LoadSpec;
}();
setType("WigglesBase.LoadSpec", LoadSpec);
export var LoadShow = function () {
  function LoadShow(name, spec) {
    _classCallCheck(this, LoadShow);

    this.name = name;
    this.spec = spec;
  }

  _createClass(LoadShow, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WigglesBase.LoadShow",
        interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
        properties: {
          name: "string",
          spec: LoadSpec
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

  return LoadShow;
}();
setType("WigglesBase.LoadShow", LoadShow);
export var ServerCommand = function () {
  function ServerCommand(caseName, fields) {
    _classCallCheck(this, ServerCommand);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ServerCommand, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WigglesBase.ServerCommand",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          AvailableSaves: [],
          Console: [GenericParam("m")],
          Load: [LoadShow],
          NewShow: ["string"],
          Quit: [],
          Rename: ["string"],
          Save: [],
          SaveAs: ["string"],
          SavedShows: [],
          ShowName: []
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

  return ServerCommand;
}();
setType("WigglesBase.ServerCommand", ServerCommand);
export var ServerResponse = function () {
  function ServerResponse(caseName, fields) {
    _classCallCheck(this, ServerResponse);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ServerResponse, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "WigglesBase.ServerResponse",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Console: [GenericParam("msg")],
          Loaded: ["string"],
          Quit: [],
          Renamed: ["string"],
          Saved: [],
          SavesAvailable: [SavesAvailable],
          ShowLibErr: ["string"],
          ShowName: ["string"],
          ShowsAvailable: [makeGeneric(List, {
            T: "string"
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

  return ServerResponse;
}();
setType("WigglesBase.ServerResponse", ServerResponse);
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
        type: "WigglesBase.Message",
        interfaces: ["FSharpUnion"],
        cases: {
          Command: [makeGeneric(ServerCommand, {
            m: GenericParam("cmd")
          })],
          Inner: [GenericParam("msg")],
          Modal: [Message_1],
          Navbar: [Message_2],
          Response: [makeGeneric(ServerResponse, {
            msg: GenericParam("rsp")
          })],
          Socket: [SocketMessage]
        }
      };
    }
  }]);

  return Message;
}();
setType("WigglesBase.Message", Message);
export function initCommands() {
  return ofArray([new ServerCommand("ShowName", [])]);
}

function prompt(msg) {
  return CmdModule.ofMsg(new Message("Modal", [new Message_1("Open", [prompt_1(msg)])]));
}

function updateFromResponse(wrapShowResponse, updateShow, message, model) {
  if (message.Case === "SavesAvailable") {
    return [function () {
      var baseModel = new BaseModel(model.baseModel.name, message.Fields[0], model.baseModel.showsAvailable, model.baseModel.modalDialog, model.baseModel.navbar);
      return new Model(model.connection, baseModel, model.showModel);
    }(), CmdModule.none()];
  } else if (message.Case === "ShowsAvailable") {
    return [function () {
      var baseModel_1 = new BaseModel(model.baseModel.name, model.baseModel.savesAvailable, message.Fields[0], model.baseModel.modalDialog, model.baseModel.navbar);
      return new Model(model.connection, baseModel_1, model.showModel);
    }(), CmdModule.none()];
  } else if (message.Case === "Loaded") {
    throw new Error("A new show was loaded but view reloading is not implemented yet.");
  } else if (message.Case === "Renamed") {
    return [function () {
      var baseModel_2 = new BaseModel(message.Fields[0], model.baseModel.savesAvailable, model.baseModel.showsAvailable, model.baseModel.modalDialog, model.baseModel.navbar);
      return new Model(model.connection, baseModel_2, model.showModel);
    }(), CmdModule.none()];
  } else if (message.Case === "Saved") {
    return [model, prompt("The show has been saved.")];
  } else if (message.Case === "ShowLibErr") {
    return [model, prompt(fsFormat("A show library error occurred: %s")(function (x) {
      return x;
    })(message.Fields[0]))];
  } else if (message.Case === "Quit") {
    fsFormat("The server sent the quit message.  The socket connection should close.")(function (x) {
      console.log(x);
    });
    return [model, CmdModule.none()];
  } else if (message.Case === "Console") {
    var patternInput = updateShow(wrapShowResponse(message.Fields[0]))(model.showModel);
    return [new Model(model.connection, model.baseModel, patternInput[0]), CmdModule.map(function (arg0) {
      return new Message("Inner", [arg0]);
    }, patternInput[1])];
  } else {
    return [function () {
      var baseModel_3 = new BaseModel(message.Fields[0], model.baseModel.savesAvailable, model.baseModel.showsAvailable, model.baseModel.modalDialog, model.baseModel.navbar);
      return new Model(model.connection, baseModel_3, model.showModel);
    }(), CmdModule.none()];
  }
}

export function update(initCommands_1, socketSend, wrapShowResponse, updateShow, message, model) {
  var updateBaseModel = function updateBaseModel(f) {
    var baseModel = f(model.baseModel);
    return new Model(model.connection, baseModel, model.showModel);
  };

  if (message.Case === "Command") {
    socketSend(message.Fields[0]);
    return [model, CmdModule.none()];
  } else if (message.Case === "Response") {
    return updateFromResponse(wrapShowResponse, updateShow, message.Fields[0], model);
  } else if (message.Case === "Navbar") {
    var newModel = updateBaseModel(function (bm) {
      var navbar = update_1(message.Fields[0], bm.navbar);
      return new BaseModel(bm.name, bm.savesAvailable, bm.showsAvailable, bm.modalDialog, navbar);
    });
    return [newModel, CmdModule.none()];
  } else if (message.Case === "Modal") {
    var newModel_1 = updateBaseModel(function (bm_1) {
      var modalDialog = update_2(message.Fields[0], bm_1.modalDialog);
      return new BaseModel(bm_1.name, bm_1.savesAvailable, bm_1.showsAvailable, modalDialog, bm_1.navbar);
    });
    return [newModel_1, CmdModule.none()];
  } else if (message.Case === "Inner") {
    var patternInput = updateShow(message.Fields[0])(model.showModel);
    return [new Model(model.connection, model.baseModel, patternInput[0]), CmdModule.map(function (arg0) {
      return new Message("Inner", [arg0]);
    }, patternInput[1])];
  } else if (message.Fields[0].Case === "Disconnected") {
    return [new Model(new ConnectionState("Closed", []), model.baseModel, model.showModel), CmdModule.none()];
  } else {
    return [new Model(new ConnectionState("Open", []), model.baseModel, model.showModel), CmdModule.map(function (arg0_1) {
      return new Message("Command", [arg0_1]);
    }, initCommands_1)];
  }
}

function viewInner(viewShow, model, dispatch) {
  var openModal = function openModal(req) {
    dispatch(new Message("Modal", [new Message_1("Open", [req])]));
  };

  return createElement("div", {}, view_1(model.baseModel.navbar, dispatch, function ($var58) {
    return dispatch(function (arg0) {
      return new Message("Navbar", [arg0]);
    }($var58));
  }), createElement("div", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Container.Fluid]), viewShow(openModal)(model.showModel)(function ($var59) {
    return dispatch(function (arg0_1) {
      return new Message("Inner", [arg0_1]);
    }($var59));
  })(function ($var61) {
    return dispatch(function ($var60) {
      return new Message("Command", [new ServerCommand("Console", [$var60])]);
    }($var61));
  }), view_2(model.baseModel.modalDialog, function ($var62) {
    return dispatch(function (arg0_2) {
      return new Message("Modal", [arg0_2]);
    }($var62));
  })));
}

export function view(viewShow, model, dispatch) {
  if (model.connection.Case === "Open") {
    return viewInner(viewShow, model, dispatch);
  } else if (model.connection.Case === "Closed") {
    return viewSplash("The console server disconnected.");
  } else {
    return viewSplash("Waiting for console server connection to be established.");
  }
}