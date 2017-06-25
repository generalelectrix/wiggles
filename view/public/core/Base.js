var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { GenericParam, Array as _Array, Option, makeGeneric, compareUnions, equalsUnions } from "fable-core/Util";
import { map, ofArray } from "fable-core/List";
import List from "fable-core/List";
import { ServerResponse, ConnectionState, SavesAvailable, ServerCommand, ResponseFilter } from "./Types";
import { view as view_4, update as update_3, Message as Message_4, initModel as initModel_1, Model as Model_1 } from "./LoadShow";
import { Model as Model_2 } from "./SimpleEditor";
import { viewSplash, view as view_6, confirm, update as update_2, prompt as prompt_1, Message as Message_1, initialModel, ModalRequest } from "./Modal";
import { view as view_5, DropdownItem, DropdownModel, Item, update as update_1, Message as Message_2, Model as Model_3 } from "./Navbar";
import { view as view_1, update as update_4, initModel as initModel_2 } from "./SaveShowAs";
import { view as view_3, update as update_6, initModel as initModel_3 } from "./NewShow";
import { view as view_2, update as update_5, initModel as initModel_4 } from "./RenameShow";
import { Message as Message_3 } from "./EditBox";
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
          NewShow: [],
          RenameShow: [],
          SaveShowAs: [],
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
  if (page.Case === "SaveShowAs") {
    return new List();
  } else if (page.Case === "NewShow") {
    return new List();
  } else if (page.Case === "RenameShow") {
    return new List();
  } else {
    return ofArray([[new ResponseFilter("Exclusive", []), new ServerCommand("SavedShows", [])]]);
  }
}
export var BaseModel = function () {
  function BaseModel(name, savesAvailable, showsAvailable, utilPage, showLoader, saveAsUtil, newShowUtil, renameShowUtil, modalDialog, navbar) {
    _classCallCheck(this, BaseModel);

    this.name = name;
    this.savesAvailable = savesAvailable;
    this.showsAvailable = showsAvailable;
    this.utilPage = utilPage;
    this.showLoader = showLoader;
    this.saveAsUtil = saveAsUtil;
    this.newShowUtil = newShowUtil;
    this.renameShowUtil = renameShowUtil;
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
          saveAsUtil: makeGeneric(Model_2, {
            a: "string"
          }),
          newShowUtil: makeGeneric(Model_2, {
            a: "string"
          }),
          renameShowUtil: makeGeneric(Model_2, {
            a: "string"
          }),
          modalDialog: _Array(ModalRequest),
          navbar: makeGeneric(Model_3, {
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
  return new BaseModel("", new SavesAvailable(new List(), new List()), new List(), null, initModel_1(), initModel_2(), initModel_3(), initModel_4(), initialModel(), navbar);
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
          NewShow: [makeGeneric(Message_3, {
            T: "string"
          })],
          RenameShow: [makeGeneric(Message_3, {
            T: "string"
          })],
          Response: [makeGeneric(ServerResponse, {
            msg: GenericParam("rsp")
          })],
          SaveShowAs: [makeGeneric(Message_3, {
            T: "string"
          })],
          ShowLoader: [Message_4],
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
      var baseModel = new BaseModel(model.baseModel.name, message.Fields[0], model.baseModel.showsAvailable, model.baseModel.utilPage, model.baseModel.showLoader, model.baseModel.saveAsUtil, model.baseModel.newShowUtil, model.baseModel.renameShowUtil, model.baseModel.modalDialog, model.baseModel.navbar);
      return new Model(model.connection, baseModel, model.showModel);
    }(), CmdModule.none()];
  } else if (message.Case === "ShowsAvailable") {
    return [function () {
      var baseModel_1 = new BaseModel(model.baseModel.name, model.baseModel.savesAvailable, message.Fields[0], model.baseModel.utilPage, model.baseModel.showLoader, model.baseModel.saveAsUtil, model.baseModel.newShowUtil, model.baseModel.renameShowUtil, model.baseModel.modalDialog, model.baseModel.navbar);
      return new Model(model.connection, baseModel_1, model.showModel);
    }(), CmdModule.none()];
  } else if (message.Case === "Loaded") {
    throw new Error("A new show was loaded but view reloading is not implemented yet.");
  } else if (message.Case === "Renamed") {
    return [function () {
      var baseModel_2 = new BaseModel(message.Fields[0], model.baseModel.savesAvailable, model.baseModel.showsAvailable, model.baseModel.utilPage, model.baseModel.showLoader, model.baseModel.saveAsUtil, model.baseModel.newShowUtil, model.baseModel.renameShowUtil, model.baseModel.modalDialog, model.baseModel.navbar);
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
      var baseModel_3 = new BaseModel(message.Fields[0], model.baseModel.savesAvailable, model.baseModel.showsAvailable, model.baseModel.utilPage, model.baseModel.showLoader, model.baseModel.saveAsUtil, model.baseModel.newShowUtil, model.baseModel.renameShowUtil, model.baseModel.modalDialog, model.baseModel.navbar);
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
      return new BaseModel(bm.name, bm.savesAvailable, bm.showsAvailable, message.Fields[0], bm.showLoader, bm.saveAsUtil, bm.newShowUtil, bm.renameShowUtil, bm.modalDialog, bm.navbar);
    });
    var commands = CmdModule.batch(map(function ($var124) {
      return function (msg) {
        return CmdModule.ofMsg(msg);
      }(function (tupledArg) {
        return new Message("Command", [tupledArg[0], tupledArg[1]]);
      }($var124));
    }, message.Fields[0] == null ? new List() : commandsForUtilPageChange(message.Fields[0])));
    return [newModel, commands];
  } else if (message.Case === "Navbar") {
    var newModel_1 = updateBaseModel(function (bm_1) {
      var navbar = update_1(message.Fields[0], bm_1.navbar);
      return new BaseModel(bm_1.name, bm_1.savesAvailable, bm_1.showsAvailable, bm_1.utilPage, bm_1.showLoader, bm_1.saveAsUtil, bm_1.newShowUtil, bm_1.renameShowUtil, bm_1.modalDialog, navbar);
    });
    return [newModel_1, CmdModule.none()];
  } else if (message.Case === "Modal") {
    var newModel_2 = updateBaseModel(function (bm_2) {
      var modalDialog = update_2(message.Fields[0], bm_2.modalDialog);
      return new BaseModel(bm_2.name, bm_2.savesAvailable, bm_2.showsAvailable, bm_2.utilPage, bm_2.showLoader, bm_2.saveAsUtil, bm_2.newShowUtil, bm_2.renameShowUtil, modalDialog, bm_2.navbar);
    });
    return [newModel_2, CmdModule.none()];
  } else if (message.Case === "ShowLoader") {
    var newModel_3 = updateBaseModel(function (bm_3) {
      var showLoader = update_3(message.Fields[0], bm_3.showLoader);
      return new BaseModel(bm_3.name, bm_3.savesAvailable, bm_3.showsAvailable, bm_3.utilPage, showLoader, bm_3.saveAsUtil, bm_3.newShowUtil, bm_3.renameShowUtil, bm_3.modalDialog, bm_3.navbar);
    });
    return [newModel_3, CmdModule.none()];
  } else if (message.Case === "SaveShowAs") {
    var newModel_4 = updateBaseModel(function (bm_4) {
      var saveAsUtil = update_4()(message.Fields[0])(bm_4.saveAsUtil);
      return new BaseModel(bm_4.name, bm_4.savesAvailable, bm_4.showsAvailable, bm_4.utilPage, bm_4.showLoader, saveAsUtil, bm_4.newShowUtil, bm_4.renameShowUtil, bm_4.modalDialog, bm_4.navbar);
    });
    return [newModel_4, CmdModule.none()];
  } else if (message.Case === "RenameShow") {
    var newModel_5 = updateBaseModel(function (bm_5) {
      var renameShowUtil = update_5()(message.Fields[0])(bm_5.renameShowUtil);
      return new BaseModel(bm_5.name, bm_5.savesAvailable, bm_5.showsAvailable, bm_5.utilPage, bm_5.showLoader, bm_5.saveAsUtil, bm_5.newShowUtil, renameShowUtil, bm_5.modalDialog, bm_5.navbar);
    });
    return [newModel_5, CmdModule.none()];
  } else if (message.Case === "NewShow") {
    var newModel_6 = updateBaseModel(function (bm_6) {
      var newShowUtil = update_6()(message.Fields[0])(bm_6.newShowUtil);
      return new BaseModel(bm_6.name, bm_6.savesAvailable, bm_6.showsAvailable, bm_6.utilPage, bm_6.showLoader, bm_6.saveAsUtil, newShowUtil, bm_6.renameShowUtil, bm_6.modalDialog, bm_6.navbar);
    });
    return [newModel_6, CmdModule.none()];
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

function viewUtil(utilPage, model, dispatch, dispatchServer) {
  var onComplete = function onComplete() {
    dispatch(new Message("UtilPage", [null]));
  };

  if (utilPage.Case === "SaveShowAs") {
    return view_1(model.baseModel.name, model.baseModel.saveAsUtil, onComplete, function ($var125) {
      return dispatch(function (arg0) {
        return new Message("SaveShowAs", [arg0]);
      }($var125));
    }, dispatchServer);
  } else if (utilPage.Case === "RenameShow") {
    return view_2(model.baseModel.name, model.baseModel.renameShowUtil, onComplete, function ($var126) {
      return dispatch(function (arg0_1) {
        return new Message("RenameShow", [arg0_1]);
      }($var126));
    }, dispatchServer);
  } else if (utilPage.Case === "NewShow") {
    return view_3(model.baseModel.newShowUtil, onComplete, function ($var127) {
      return dispatch(function (arg0_2) {
        return new Message("NewShow", [arg0_2]);
      }($var127));
    }, dispatchServer);
  } else {
    return view_4(model.baseModel.showsAvailable, model.baseModel.showLoader, onComplete, function ($var128) {
      return dispatch(function (arg0_3) {
        return new Message("ShowLoader", [arg0_3]);
      }($var128));
    }, dispatchServer);
  }
}

function utilPageItem(text, page) {
  return new Item(text, function (dispatch) {
    dispatch(new Message("UtilPage", [page]));
  });
}

function saveShowItem() {
  return new Item("Save", function (dispatch) {
    dispatch(function (tupledArg) {
      return new Message("Command", [tupledArg[0], tupledArg[1]]);
    }([new ResponseFilter("Exclusive", []), new ServerCommand("Save", [])]));
  });
}

function quitItem() {
  return new Item("Quit", function (dispatch) {
    var modalAction = confirm("Are you sure you want to quit?", function (_arg1) {
      dispatch(function (tupledArg) {
        return new Message("Command", [tupledArg[0], tupledArg[1]]);
      }([new ResponseFilter("All", []), new ServerCommand("Quit", [])]));
    });
    dispatch(new Message("Modal", [new Message_1("Open", [modalAction])]));
  });
}

export function utilDropdown() {
  return new DropdownModel("Wiggles", ofArray([new DropdownItem("Selection", [utilPageItem("New show...", new UtilPage("NewShow", []))]), new DropdownItem("Selection", [utilPageItem("Load show...", new UtilPage("ShowLoader", []))]), new DropdownItem("Separator", []), new DropdownItem("Selection", [saveShowItem()]), new DropdownItem("Selection", [utilPageItem("Save as...", new UtilPage("SaveShowAs", []))]), new DropdownItem("Selection", [utilPageItem("Rename...", new UtilPage("RenameShow", []))]), new DropdownItem("Separator", []), new DropdownItem("Selection", [quitItem()])]), false);
}

function viewInner(viewShow, model, dispatch) {
  var openModal = function openModal(req) {
    dispatch(new Message("Modal", [new Message_1("Open", [req])]));
  };

  var page = void 0;
  var matchValue = model.baseModel.utilPage;

  if (matchValue != null) {
    page = viewUtil(matchValue, model, dispatch, function ($var129) {
      return dispatch(function (tupledArg) {
        return new Message("Command", [tupledArg[0], tupledArg[1]]);
      }($var129));
    });
  } else {
    var dispatchServer = function dispatchServer($var131) {
      return dispatch(function ($var130) {
        return function (tupledArg_2) {
          return new Message("Command", [tupledArg_2[0], tupledArg_2[1]]);
        }(function () {
          var f = function f(arg0) {
            return new ServerCommand("Console", [arg0]);
          };

          return function (tupledArg_1) {
            return liftResponseAndFilter(f, tupledArg_1[0], tupledArg_1[1]);
          };
        }()($var130));
      }($var131));
    };

    page = viewShow(openModal)(model.showModel)(function ($var132) {
      return dispatch(function (arg0_1) {
        return new Message("Inner", [arg0_1]);
      }($var132));
    })(dispatchServer);
  }

  return createElement("div", {}, createElement("div", {}, view_5(model.baseModel.navbar, dispatch, function ($var133) {
    return dispatch(function (arg0_2) {
      return new Message("Navbar", [arg0_2]);
    }($var133));
  })), createElement("div", fold(function (o, kv) {
    o[kv[0]] = kv[1];
    return o;
  }, {}, [Container.Fluid]), page, view_6(model.baseModel.modalDialog, function ($var134) {
    return dispatch(function (arg0_3) {
      return new Message("Modal", [arg0_3]);
    }($var134));
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