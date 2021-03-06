var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { ResultModule, Result as Result_1 } from "fable-elmish/result";
import { compareUnions, equalsUnions, GenericParam, toString } from "fable-core/Util";
import { fsFormat, trim } from "fable-core/String";
import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { add, tryFind } from "fable-core/Map";
export var EnterKey = 13;
export var EscapeKey = 27;
export var Result = function (__exports) {
  var ofOption = __exports.ofOption = function (o) {
    if (o == null) {
      return new Result_1("Error", [null]);
    } else {
      return new Result_1("Ok", [o]);
    }
  };

  return __exports;
}({});
export function withDefault(value, opt) {
  if (opt == null) {
    return value;
  } else {
    return opt;
  }
}
export function emptyIfNone(opt) {
  if (opt == null) {
    return "";
  } else {
    return toString(opt);
  }
}
export function noneIfEmpty(s) {
  if (trim(s, "both") === "") {
    return null;
  } else {
    return s;
  }
}
export var errorIfEmpty = function errorIfEmpty($var4) {
  return Result.ofOption(noneIfEmpty($var4));
};
export function parseInt(s) {
  var parsed = Number.parseInt(s);

  if (Number.isNaN(parsed)) {
    return null;
  } else {
    return parsed;
  }
}
export function parseFloat(s) {
  var parsed = Number.parseFloat(s);

  if (Number.isNaN(parsed)) {
    return null;
  } else {
    return parsed;
  }
}
export var Optional = function () {
  function Optional(caseName, fields) {
    _classCallCheck(this, Optional);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(Optional, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Util.Optional",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Absent: [],
          Present: [GenericParam("T")]
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
    key: "ToString",
    value: function () {
      if (this.Case === "Absent") {
        return "";
      } else {
        return toString(this.Fields[0]);
      }
    }
  }]);

  return Optional;
}();
setType("Util.Optional", Optional);
export var OptionalModule = function (__exports) {
  var ofOption = __exports.ofOption = function (opt) {
    if (opt == null) {
      return new Optional("Absent", []);
    } else {
      return new Optional("Present", [opt]);
    }
  };

  return __exports;
}({});
export function parseOptionalNumber(validator, v) {
  var matchValue = noneIfEmpty(v);

  if (matchValue != null) {
    return ResultModule.map(function (arg0) {
      return new Optional("Present", [arg0]);
    }, function (r) {
      return ResultModule.bind(validator, r);
    }(Result.ofOption(parseInt(matchValue))));
  } else {
    return new Result_1("Ok", [new Optional("Absent", [])]);
  }
}
export function enqueueBrowserAction(action) {
  window.setTimeout(function (_arg1) {
    return action(null);
  }, 0);
}
export function logException(msg, e) {
  console.error(msg, e);
}
export function logError(msg) {
  console.error(msg);
}
export function transformMapItem(key, f, map) {
  var matchValue = function (table) {
    return tryFind(key, table);
  }(map);

  if (matchValue == null) {
    logError(fsFormat("Tried to operate on map key %+A but it is not present.")(function (x) {
      return x;
    })(key));
    return map;
  } else {
    return function () {
      var value = f(matchValue);
      return function (table_1) {
        return add(key, value, table_1);
      };
    }()(map);
  }
}