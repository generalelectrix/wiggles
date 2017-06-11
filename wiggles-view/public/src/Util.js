import { ResultModule, Result as Result_1 } from "fable-elmish/result";
import { toString } from "fable-core/Util";
import { trim } from "fable-core/String";
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
export var Option = function (__exports) {
  var orElse = __exports.orElse = function (x, y) {
    if (function () {
      return y != null;
    }(null)) {
      return y;
    } else {
      return x;
    }
  };

  return __exports;
}({});
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
export var errorIfEmpty = function errorIfEmpty($var1) {
  return Result.ofOption(noneIfEmpty($var1));
};
export function parseInt(s) {
  var parsed = Number.parseInt(s);

  if (Number.isNaN(parsed)) {
    return null;
  } else {
    return parsed;
  }
}
export function parseOptionalNumber(validator, v) {
  var matchValue = noneIfEmpty(v);

  if (matchValue != null) {
    return ResultModule.map(function (arg0) {
      return arg0;
    }, function (r) {
      return ResultModule.bind(validator, r);
    }(Result.ofOption(parseInt(matchValue))));
  } else {
    return new Result_1("Ok", [null]);
  }
}