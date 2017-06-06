var path = require("path");

module.exports = {
    entry: "./public/src/App.js",
    output: {
        filename: "bundle.js",
        path: path.resolve(__dirname, "public")
    }
};