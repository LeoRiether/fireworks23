(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var fireworks23__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! fireworks23 */ \"./node_modules/.pnpm/file+..+pkg/node_modules/fireworks23/fireworks23.js\");\n\n\nconsole.log(\"[JS] initializing fireworks23\");\n\nlet fireworks = fireworks23__WEBPACK_IMPORTED_MODULE_0__[\"Fireworks\"].new();\n// app.ticker.add(_ => fireworks.tick());\n\nconst loop = () => {\n    fireworks.tick();\n    requestAnimationFrame(loop);\n};\nrequestAnimationFrame(loop);\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);