var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

import { setType } from "fable-core/Symbol";
import _Symbol from "fable-core/Symbol";
import { compareUnions, equalsUnions } from "fable-core/Util";
import { CmdModule } from "fable-elmish/elmish";
import { fsFormat } from "fable-core/String";
import { toJson, ofJson } from "fable-core/Serialize";
export var SocketMessage = function () {
  function SocketMessage(caseName, fields) {
    _classCallCheck(this, SocketMessage);

    this.Case = caseName;
    this.Fields = fields;
  }

  _createClass(SocketMessage, [{
    key: _Symbol.reflection,
    value: function () {
      return {
        type: "Socket.SocketMessage",
        interfaces: ["FSharpUnion", "System.IEquatable", "System.IComparable"],
        cases: {
          Connected: [],
          Disconnected: []
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

  return SocketMessage;
}();
setType("Socket.SocketMessage", SocketMessage);

function logException(msg, e) {
  console.error(msg, e);
}

export function openSocket(wrapSocketMessage, _genArgs) {
  var host = "ws://127.0.0.1:2794";
  var ws = new WebSocket(host, "wiggles");

  var subscription = function subscription(messageWrapper) {
    return function (_arg1) {
      return CmdModule.ofSub(function (dispatch) {
        ws.addEventListener('message', function (event) {
          try {
            var message = event.data;
            fsFormat("Received message: %s")(function (x) {
              console.log(x);
            })(message);
            var deserialized = ofJson(message, {
              T: _genArgs.rsp
            });
            dispatch(messageWrapper(deserialized));
          } catch (e) {
            logException("Message deserialization error:", e);
          }

          return null;
        });
        ws.addEventListener('open', function (_arg1_1) {
          dispatch(wrapSocketMessage(new SocketMessage("Connected", [])));
          return null;
        });
        ws.addEventListener('close', function (_arg2) {
          dispatch(wrapSocketMessage(new SocketMessage("Disconnected", [])));
          return null;
        });
      });
    };
  };

  var send = function send(msg) {
    var jsonMessage = toJson(msg);
    fsFormat("Sending message from socket: %s")(function (x) {
      console.log(x);
    })(jsonMessage);

    try {
      ws.send(jsonMessage);
    } catch (e_1) {
      logException(fsFormat("Websocket error while sending message:\n%s\n\n")(function (x) {
        return x;
      })(jsonMessage), e_1);
    }
  };

  return [subscription, send];
}