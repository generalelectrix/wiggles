var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compareRecords, equalsRecords, compareUnions, equalsUnions } from "fable-core/Util";
import { map, ofArray } from "fable-core/List";
import { view as view_1, update as update_1, Message as Message_1, Model as Model_1 } from "./Table";
import { ResponseFilter, LoadShow as LoadShow_1, ServerCommand, LoadSpec } from "./Types";
import { createElement } from "react";
import { tryItem, fold } from "fable-core/Seq";
import { Button, Grid, InputType } from "./Bootstrap";
import { errorIfEmpty, logError } from "./Util";
import { fsFormat } from "fable-core/String";
import { view as view_2, update as update_2, initModel as initModel_1 } from "./SimpleEditor";
export var LoadShow = function (__exports) {
  var Row = __exports.Row = function () {
    function Row(caseName, fields) {
      _classCallCheck(this, Row);

      this.Case = caseName;
      this.Fields = fields;
    }

    _createClass(Row, [{
      key: _Symbol.reflection,
      value: function () {
        return {
          type: "UtilPages.LoadShow.Row",
          interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable", "Table.IRow"],
          cases: {
            Row: ["string"]
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
      key: "ToStrings",
      value: function () {
        return ofArray([this.Fields[0]]);
      }
    }]);

    return Row;
  }();

  setType("UtilPages.LoadShow.Row", Row);

  var Model = __exports.Model = function () {
    function Model(table, loadSpec) {
      _classCallCheck(this, Model);

      this.table = table;
      this.loadSpec = loadSpec;
    }

    _createClass(Model, [{
      key: _Symbol.reflection,
      value: function () {
        return {
          type: "UtilPages.LoadShow.Model",
          interfaces: ["FSharpRecord", "System.IEquatable", "System.IComparable"],
          properties: {
            table: Model_1,
            loadSpec: LoadSpec
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

  setType("UtilPages.LoadShow.Model", Model);

  var initModel = __exports.initModel = function () {
    return new Model(new Model_1(300, ofArray(["Show name"]), null), new LoadSpec("Latest", []));
  };

  var Message = __exports.Message = function () {
    function Message(caseName, fields) {
      _classCallCheck(this, Message);

      this.Case = caseName;
      this.Fields = fields;
    }

    _createClass(Message, [{
      key: _Symbol.reflection,
      value: function () {
        return {
          type: "UtilPages.LoadShow.Message",
          interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
          cases: {
            LoadSpec: [LoadSpec],
            Table: [Message_1]
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

  setType("UtilPages.LoadShow.Message", Message);

  var update = __exports.update = function (message, model) {
    if (message.Case === "LoadSpec") {
      return new Model(model.table, message.Fields[0]);
    } else {
      return new Model(update_1(message.Fields[0], model.table), model.loadSpec);
    }
  };

  var loadModeSelector = __exports.loadModeSelector = function (selected, dispatch) {
    var radio = function radio(text) {
      return function (spec) {
        var onClick = function onClick(_arg1) {
          dispatch(new Message("LoadSpec", [spec]));
        };

        return createElement("div", {
          className: "radio"
        }, createElement("label", {}, createElement("input", fold(function (o, kv) {
          o[kv[0]] = kv[1];
          return o;
        }, {}, [["readOnly", true], ["checked", selected.Equals(spec)], ["onClick", onClick], InputType.Radio])), text));
      };
    };

    return Grid.layout(ofArray([[3, ofArray([radio("Load from save")(new LoadSpec("Latest", []))])], [3, ofArray([radio("Recover from autosave")(new LoadSpec("LatestAutosave", []))])]]));
  };

  var loadButton = __exports.loadButton = function (shows, model, onComplete, dispatchServer) {
    var onClick = function onClick(_arg1) {
      var matchValue = model.table.selected;

      if (matchValue == null) {} else {
        var matchValue_1 = function (list) {
          return tryItem(matchValue, list);
        }(shows);

        if (matchValue_1 == null) {
          logError(fsFormat("Load action had a selected value %d that was not in range.")(function (x) {
            return x;
          })(matchValue));
        } else {
          var command = new ServerCommand("Load", [new LoadShow_1(matchValue_1, model.loadSpec)]);
          dispatchServer([new ResponseFilter("All", []), command]);
          onComplete(null);
        }
      }
    };

    return createElement("button", fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [["onClick", onClick], Button.Primary]), "Load");
  };

  var cancelButton = __exports.cancelButton = function (onComplete) {
    var onClick = function onClick(_arg1) {
      onComplete(null);
    };

    return createElement("button", fold(function (o, kv) {
      o[kv[0]] = kv[1];
      return o;
    }, {}, [["onClick", onClick], Button.Default]), "Cancel");
  };

  var view = __exports.view = function (shows, model, onComplete, dispatch, dispatchServer) {
    var showTable = view_1(map(function (arg0) {
      return new Row("Row", [arg0]);
    }, shows), model.table, function ($var97) {
      return dispatch(function (arg0_1) {
        return new Message("Table", [arg0_1]);
      }($var97));
    });
    var loadButton_1 = loadButton(shows, model, onComplete, dispatchServer);
    return createElement("div", {}, Grid.fullRow(ofArray([showTable])), Grid.fullRow(ofArray([loadModeSelector(model.loadSpec, dispatch)])), Grid.layout(ofArray([[1, ofArray([loadButton_1])], [1, ofArray([cancelButton(onComplete)])]])));
  };

  return __exports;
}({});
export var RenameShow = function (__exports) {
  var initModel = __exports.initModel = function () {
    return initModel_1("Rename", "Rename this show:", errorIfEmpty, InputType.Text);
  };

  var update = __exports.update = function () {
    return function (message) {
      return function (model) {
        return update_2(message, model);
      };
    };
  };

  var view = __exports.view = function (showName, model, onComplete, dispatch, dispatchServer) {
    var onOk = function onOk(name) {
      var command = new ServerCommand("Rename", [name]);
      dispatchServer([new ResponseFilter("All", []), command]);
    };

    return view_2(showName, model, onOk, onComplete, dispatch);
  };

  return __exports;
}({});
export var SaveShowAs = function (__exports) {
  var initModel = __exports.initModel = function () {
    return initModel_1("Save", "Save show as...", errorIfEmpty, InputType.Text);
  };

  var update = __exports.update = function () {
    return function (message) {
      return function (model) {
        return update_2(message, model);
      };
    };
  };

  var view = __exports.view = function (showName, model, onComplete, dispatch, dispatchServer) {
    var onOk = function onOk(newName) {
      var command = new ServerCommand("SaveAs", [newName]);
      dispatchServer([new ResponseFilter("All", []), command]);
    };

    return view_2(showName, model, onOk, onComplete, dispatch);
  };

  return __exports;
}({});
export var NewShow = function (__exports) {
  var initModel = __exports.initModel = function () {
    return initModel_1("New", "Name for new show:", errorIfEmpty, InputType.Text);
  };

  var update = __exports.update = function () {
    return function (message) {
      return function (model) {
        return update_2(message, model);
      };
    };
  };

  var view = __exports.view = function (model, onComplete, dispatch, dispatchServer) {
    var onOk = function onOk(name) {
      var command = new ServerCommand("NewShow", [name]);
      dispatchServer([new ResponseFilter("All", []), command]);
    };

    return view_2("", model, onOk, onComplete, dispatch);
  };

  return __exports;
}({});