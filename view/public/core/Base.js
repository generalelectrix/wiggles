var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { Message, ServerCommand, ConnectionState, SavesAvailable } from "./Types";
import { GenericParam, Array as _Array, makeGeneric } from "fable-core/Util";
import { ofArray } from "fable-core/List";
import List from "fable-core/List";
import { viewSplash, view as view_2, update as update_2, prompt as prompt_1, Message as Message_1, initialModel, ModalRequest } from "./Modal";
import { view as view_1, update as update_1, Model as Model_1 } from "./Navbar";
import { CmdModule } from "fable-elmish/elmish";
import { fsFormat } from "fable-core/String";
import { createElement } from "react";
import { fold } from "fable-core/Seq";
import { Container } from "./Bootstrap";
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
        type: "Base.BaseModel",
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
  return new BaseModel("", new SavesAvailable(new List(), new List()), new List(), initialModel(), navbar);
}

export function initModel(navbar, showModel) {
  return new Model(new ConnectionState("Waiting", []), initBaseModel(navbar), showModel);
}
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
    socketSend([message.Fields[0], message.Fields[1]]);
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
    return [new Model(new ConnectionState("Open", []), model.baseModel, model.showModel), CmdModule.map(function (tupledArg) {
      return new Message("Command", [tupledArg[0], tupledArg[1]]);
    }, initCommands_1)];
  }
}

function viewInner(viewShow, model, dispatch) {
  var openModal = function openModal(req) {
    dispatch(new Message("Modal", [new Message_1("Open", [req])]));
  };

  var dispatchServer = function dispatchServer($var75) {
    return dispatch(function ($var74) {
      return function (tupledArg_1) {
        return new Message("Command", [tupledArg_1[0], tupledArg_1[1]]);
      }(function () {
        var f = function f(arg0) {
          return new ServerCommand("Console", [arg0]);
        };

        return function (tupledArg) {
          return liftResponseAndFilter(f, tupledArg[0], tupledArg[1]);
        };
      }()($var74));
    }($var75));
  };

  var showView = viewShow(openModal)(model.showModel)(function ($var76) {
    return dispatch(function (arg0_1) {
      return new Message("Inner", [arg0_1]);
    }($var76));
  })(dispatchServer);
  return createElement("div", {}, createElement("div", {}, view_1(model.baseModel.navbar, dispatch, function ($var77) {
    return dispatch(function (arg0_2) {
      return new Message("Navbar", [arg0_2]);
    }($var77));
  })), createElement("div", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Container.Fluid]), showView, view_2(model.baseModel.modalDialog, function ($var78) {
    return dispatch(function (arg0_3) {
      return new Message("Modal", [arg0_3]);
    }($var78));
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