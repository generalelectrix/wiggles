var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { GenericParam, compareRecords, equalsRecords, makeGeneric, compareUnions, equalsUnions } from "fable-core/Util";
import List from "fable-core/List";
import { Message as Message_1 } from "./Modal";
import { Message as Message_2 } from "./Navbar";
import { SocketMessage } from "./Socket";
export var ResponseFilter = function () {
  function ResponseFilter(caseName, fields) {
    _classCallCheck(this, ResponseFilter);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(ResponseFilter, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Types.ResponseFilter",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          All: [],
          AllButSelf: [],
          Exclusive: []
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

  return ResponseFilter;
}();
setType("Types.ResponseFilter", ResponseFilter);
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
        type: "Types.ConnectionState",
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
setType("Types.ConnectionState", ConnectionState);
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
        type: "Types.SavesAvailable",
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
setType("Types.SavesAvailable", SavesAvailable);
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
        type: "Types.LoadSpec",
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
setType("Types.LoadSpec", LoadSpec);
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
        type: "Types.LoadShow",
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
setType("Types.LoadShow", LoadShow);
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
        type: "Types.ServerCommand",
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
setType("Types.ServerCommand", ServerCommand);
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
        type: "Types.ServerResponse",
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
setType("Types.ServerResponse", ServerResponse);
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
        type: "Types.Message",
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
          Socket: [SocketMessage]
        }
      };
    }
  }]);

  return Message;
}();
setType("Types.Message", Message);