/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./index.ts":
/*!******************!*\
  !*** ./index.ts ***!
  \******************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\nObject(function webpackMissingModule() { var e = new Error(\"Cannot find module 'special-relativity-web'\"); e.code = 'MODULE_NOT_FOUND'; throw e; }());\n\nconst canvas = document.getElementById(\"canvas\");\nif (!canvas) {\n    throw new Error(\"No 'canvas'\");\n}\ncanvas.width = 800;\ncanvas.height = 600;\nconst context = canvas.getContext(\"webgl2\");\nif (!context) {\n    throw new Error(\"webgl2 not supported\");\n}\ncontext.viewport(0, 0, canvas.width, canvas.height);\nconst app = new Object(function webpackMissingModule() { var e = new Error(\"Cannot find module 'special-relativity-web'\"); e.code = 'MODULE_NOT_FOUND'; throw e; }())(context);\ndocument.addEventListener('keydown', (event) => {\n    app.key_down(event.key.toLowerCase());\n});\ndocument.addEventListener('keyup', (event) => {\n    app.key_up(event.key.toLowerCase());\n});\nwindow.addEventListener('blur', (event) => {\n    app.window_blue();\n});\nfunction step(timestamp) {\n    app.tick(timestamp);\n    window.requestAnimationFrame(step);\n}\nwindow.requestAnimationFrame(step);\n\n\n//# sourceURL=webpack://special-relativity-web/./index.ts?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The require scope
/******/ 	var __webpack_require__ = {};
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = {};
/******/ 	__webpack_modules__["./index.ts"](0, __webpack_exports__, __webpack_require__);
/******/ 	
/******/ })()
;