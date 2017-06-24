var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { GenericParam, Array as _Array, Option, makeGeneric, compareUnions, equalsUnions } from "fable-core/Util";
import { map, ofArray } from "fable-core/List";
import List from "fable-core/List";
import { ServerResponse, ConnectionState, SavesAvailable, ServerCommand, ResponseFilter } from "./Types";
import { view as view_1, update as update_3, Message as Message_3, initModel as initModel_1, Model as Model_1 } from "./LoadShow";
import { viewSplash, view as view_3, update as update_2, prompt as prompt_1, Message as Message_1, initialModel, ModalRequest } from "./Modal";
import { view as view_2, update as update_1, Message as Message_2, Model as Model_2 } from "./Navbar";
import { SocketMessage } from "./Socket";
import { CmdModule } from "fable-elmish/elmish";
import { fsFormat } from "fable-core/String";
import { createElement } from "react";
import { fold } from "fable-core/Seq";
import { Container } from "./Bootstrap";
export var UtilPage = function () {
  function UtilPage(caseName, fields) {
    _classCallCheck(this, UtilPage);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(UtilPage, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Base.UtilPage",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          ShowLoader: []
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

  return UtilPage;
}();
setType("Base.UtilPage", UtilPage);
export function commandsForUtilPageChange(page) {
  return ofArray([[new ResponseFilter("Exclusive", []), new ServerCommand("SavedShows", [])]]);
}
export var BaseModel = function () {
  function BaseModel(name, savesAvailable, showsAvailable, utilPage, showLoader, modalDialog, navbar) {
    _classCallCheck(this, BaseModel);

    this.name = name;
    this.savesAvailable = savesAvailable;
    this.showsAvailable = showsAvailable;
    this.utilPage = utilPage;
    this.showLoader = showLoader;
    this.modalDialog = modalDialog;
    this.navbar = navbar;
  }

  _createClass(BaseModel, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Base.BaseModel",
        interfaces: ["FSharpRecord"],
        properties: {
          name: "string",
          savesAvailable: SavesAvailable,
          showsAvailable: makeGeneric(List, {
            T: "string"
          }),
          utilPage: Option(UtilPage),
          showLoader: Model_1,
          modalDialog: _Array(ModalRequest),
          navbar: makeGeneric(Model_2, {
            msg: GenericParam("msg")
          })
        }
      };
    }
  }]);

  return BaseModel;
}();
setType("Base.BaseModel", BaseModel);
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
        type: "Base.Model",
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
setType("Base.Model", Model);
export function liftResponseAndFilter(f, filter, message) {
  return [filter, f(message)];
}

function initBaseModel(navbar) {
  return new BaseModel("", new SavesAvailable(new List(), new List()), new List(), null, initModel_1(), initialModel(), navbar);
}

export function initModel(navbar, showModel) {
  return new Model(new ConnectionState("Waiting", []), initBaseModel(navbar), showModel);
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
        type: "Base.Message",
        interfaces: ["FSharpUnion"],
        cases: {
          Command: [ResponseFilter, makeGeneric(ServerCommand, {
            m: GenericParam("cmd")
          })],
          Inner: [GenericParam("msg")],
          Modal: [Message_1],
          Navbar: [Message_2],
          Response: [makeGeneric(ServerResponse, {
            msg: GenericParam("rsp")
          })],
          ShowLoader: [Message_3],
          Socket: [SocketMessage],
          UtilPage: [Option(UtilPage)]
        }
      };
    }
  }]);

  return Message;
}();
setType("Base.Message", Message);
export function initCommands() {
  return ofArray([new ServerCommand("ShowName", [])]);
}

function prompt(msg) {
  return CmdModule.ofMsg(new Message("Modal", [new Message_1("Open", [prompt_1(msg)])]));
}

function updateFromResponse(wrapShowResponse, updateShow, message, model) {
  if (message.Case === "SavesAvailable") {
    return [function () {
      var baseModel = new BaseModel(model.baseModel.name, message.Fields[0], model.baseModel.showsAvailable, model.baseModel.utilPage, model.baseModel.showLoader, model.baseModel.modalDialog, model.baseModel.navbar);
      return new Model(model.connection, baseModel, model.showModel);
    }(), CmdModule.none()];
  } else if (message.Case === "ShowsAvailable") {
    return [function () {
      var baseModel_1 = new BaseModel(model.baseModel.name, model.baseModel.savesAvailable, message.Fields[0], model.baseModel.utilPage, model.baseModel.showLoader, model.baseModel.modalDialog, model.baseModel.navbar);
      return new Model(model.connection, baseModel_1, model.showModel);
    }(), CmdModule.none()];
  } else if (message.Case === "Loaded") {
    throw new Error("A new show was loaded but view reloading is not implemented yet.");
  } else if (message.Case === "Renamed") {
    return [function () {
      var baseModel_2 = new BaseModel(message.Fields[0], model.baseModel.savesAvailable, model.baseModel.showsAvailable, model.baseModel.utilPage, model.baseModel.showLoader, model.baseModel.modalDialog, model.baseModel.navbar);
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
      var baseModel_3 = new BaseModel(message.Fields[0], model.baseModel.savesAvailable, model.baseModel.showsAvailable, model.baseModel.utilPage, model.baseModel.showLoader, model.baseModel.modalDialog, model.baseModel.navbar);
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
    socketSend([message.Fields[0], message.Fields[1]]);
    return [model, CmdModule.none()];
  } else if (message.Case === "Response") {
    return updateFromResponse(wrapShowResponse, updateShow, message.Fields[0], model);
  } else if (message.Case === "UtilPage") {
    var newModel = updateBaseModel(function (bm) {
      return new BaseModel(bm.name, bm.savesAvailable, bm.showsAvailable, message.Fields[0], bm.showLoader, bm.modalDialog, bm.navbar);
    });
    var commands = CmdModule.batch(map(function ($var77) {
      return function (msg) {
        return CmdModule.ofMsg(msg);
      }(function (tupledArg) {
        return new Message("Command", [tupledArg[0], tupledArg[1]]);
      }($var77));
    }, message.Fields[0] == null ? new List() : commandsForUtilPageChange(message.Fields[0])));
    return [newModel, commands];
  } else if (message.Case === "Navbar") {
    var newModel_1 = updateBaseModel(function (bm_1) {
      var navbar = update_1(message.Fields[0], bm_1.navbar);
      return new BaseModel(bm_1.name, bm_1.savesAvailable, bm_1.showsAvailable, bm_1.utilPage, bm_1.showLoader, bm_1.modalDialog, navbar);
    });
    return [newModel_1, CmdModule.none()];
  } else if (message.Case === "Modal") {
    var newModel_2 = updateBaseModel(function (bm_2) {
      var modalDialog = update_2(message.Fields[0], bm_2.modalDialog);
      return new BaseModel(bm_2.name, bm_2.savesAvailable, bm_2.showsAvailable, bm_2.utilPage, bm_2.showLoader, modalDialog, bm_2.navbar);
    });
    return [newModel_2, CmdModule.none()];
  } else if (message.Case === "ShowLoader") {
    var newModel_3 = updateBaseModel(function (bm_3) {
      var showLoader = update_3(message.Fields[0], bm_3.showLoader);
      return new BaseModel(bm_3.name, bm_3.savesAvailable, bm_3.showsAvailable, bm_3.utilPage, showLoader, bm_3.modalDialog, bm_3.navbar);
    });
    return [newModel_3, CmdModule.none()];
  } else if (message.Case === "Inner") {
    var patternInput = updateShow(message.Fields[0])(model.showModel);
    return [new Model(model.connection, model.baseModel, patternInput[0]), CmdModule.map(function (arg0) {
      return new Message("Inner", [arg0]);
    }, patternInput[1])];
  } else if (message.Fields[0].Case === "Disconnected") {
    return [new Model(new ConnectionState("Closed", []), model.baseModel, model.showModel), CmdModule.none()];
  } else {
    return [new Model(new ConnectionState("Open", []), model.baseModel, model.showModel), CmdModule.map(function (tupledArg_1) {
      return new Message("Command", [tupledArg_1[0], tupledArg_1[1]]);
    }, initCommands_1)];
  }
}
export function viewUtil(utilPage, model, dispatch, dispatchServer) {
  var onComplete = function onComplete() {
    dispatch(new Message("UtilPage", [null]));
  };

  return view_1(model.baseModel.showsAvailable, model.baseModel.showLoader, onComplete, function ($var78) {
    return dispatch(function (arg0) {
      return new Message("ShowLoader", [arg0]);
    }($var78));
  }, dispatchServer);
}

function viewInner(viewShow, model, dispatch) {
  var openModal = function openModal(req) {
    dispatch(new Message("Modal", [new Message_1("Open", [req])]));
  };

  var page = void 0;
  var matchValue = model.baseModel.utilPage;

  if (matchValue != null) {
    page = viewUtil(matchValue, model, dispatch, function ($var79) {
      return dispatch(function (tupledArg) {
        return new Message("Command", [tupledArg[0], tupledArg[1]]);
      }($var79));
    });
  } else {
    var dispatchServer = function dispatchServer($var81) {
      return dispatch(function ($var80) {
        return function (tupledArg_2) {
          return new Message("Command", [tupledArg_2[0], tupledArg_2[1]]);
        }(function () {
          var f = function f(arg0) {
            return new ServerCommand("Console", [arg0]);
          };

          return function (tupledArg_1) {
            return liftResponseAndFilter(f, tupledArg_1[0], tupledArg_1[1]);
          };
        }()($var80));
      }($var81));
    };

    page = viewShow(openModal)(model.showModel)(function ($var82) {
      return dispatch(function (arg0_1) {
        return new Message("Inner", [arg0_1]);
      }($var82));
    })(dispatchServer);
  }

  return createElement("div", {}, createElement("div", {}, view_2(model.baseModel.navbar, dispatch, function ($var83) {
    return dispatch(function (arg0_2) {
      return new Message("Navbar", [arg0_2]);
    }($var83));
  })), createElement("div", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Container.Fluid]), page, view_3(model.baseModel.modalDialog, function ($var84) {
    return dispatch(function (arg0_3) {
      return new Message("Modal", [arg0_3]);
    }($var84));
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