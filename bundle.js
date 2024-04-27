(()=>{"use strict";var e,t,n,r,o={73:(e,t,n)=>{n.a(e,(async(e,t)=>{try{var r=n(300),o=e([r]);r=(o.then?(await o)():o)[0];const _=document.getElementById("canvas");if(!_)throw new Error("No 'canvas'");const a=window.innerWidth||document.documentElement.clientWidth||document.body.clientWidth,c=window.innerHeight||document.documentElement.clientHeight||document.body.clientHeight;a<c?(_.width=a,_.height=.8*c):(_.width=1200,_.height=800);const i=_.getContext("webgl2");if(!i)throw new Error("webgl2 not supported");i.viewport(0,0,_.width,_.height);const u=document.getElementById("info"),f=new r.qw(i);function d(e){const t=[],n=[];for(let r=0;r<e.touches.length;r++)t.push(e.touches[r].clientX),n.push(e.touches[r].clientY);return[new Float64Array(t),new Float64Array(n)]}document.addEventListener("keydown",(e=>{e.key.toLowerCase().startsWith("arrow")&&e.preventDefault(),f.key_down(e.key.toLowerCase())})),document.addEventListener("keyup",(e=>{f.key_up(e.key.toLowerCase())})),window.addEventListener("blur",(()=>{f.window_blue()})),_.addEventListener("touchstart",(e=>{e.preventDefault();const[t,n]=d(e);f.touch_start((new Date).getTime(),t,n)})),_.addEventListener("touchmove",(e=>{e.preventDefault();const[t,n]=d(e);f.touch_move((new Date).getTime(),t,n)})),_.addEventListener("touchend",(()=>{f.touch_end((new Date).getTime())}));let g=!1;_.addEventListener("mousedown",(e=>{g=!0,f.touch_start((new Date).getTime(),new Float64Array([e.clientX]),new Float64Array([e.clientY]))})),_.addEventListener("mousemove",(e=>{g&&f.touch_move((new Date).getTime(),new Float64Array([e.clientX]),new Float64Array([e.clientY]))})),_.addEventListener("mouseup",(()=>{g=!1,f.touch_end((new Date).getTime())})),_.addEventListener("mouseout",(()=>{g=!1,f.touch_end((new Date).getTime())}));const b=document.getElementById("speed-of-light-exp"),l=document.getElementById("speed-of-light");function s(){const e=b.valueAsNumber;return Math.pow(2,e)}b.onchange=()=>{const e=s();l.innerText=e.toString(),f.change_c(e)};const w=document.getElementsByName("preset");function h(){for(let e=0;e<w.length;e++)if(w.item(e).checked){f.reset_charge(w.item(e).value);break}}for(let x=0;x<5;x++)document.getElementById(`preset${x}`).onchange=h;const m=document.getElementsByName("grid-option"),p=()=>{for(let e=0;e<m.length;e++)m.item(e).checked?(f.reset_grid(m.item(e).value),m.item(e).nextElementSibling.classList.add("checked")):m.item(e).nextElementSibling.classList.remove("checked")};document.getElementById("grid-option1").onchange=p,document.getElementById("grid-option2").onchange=p;const y=document.getElementById("poynting_on");y.onchange=()=>{f.change_poynting_on(y.checked)};const v=document.getElementById("arrow-log");v.onchange=()=>{f.change_arrow_length_log(v.valueAsNumber)};const S=document.getElementById("arrow-factor");function E(e){f.tick(e),u.innerText=f.info(),window.requestAnimationFrame(E)}S.onchange=()=>{f.change_arrow_length_factor(Math.pow(2,S.valueAsNumber))},f.reset_charge(w.item(0).value),f.change_c(s()),f.change_arrow_length_log(v.valueAsNumber),f.change_arrow_length_factor(Math.pow(2,S.valueAsNumber)),window.requestAnimationFrame(E),t()}catch(A){t(A)}}))},300:(e,t,n)=>{n.a(e,(async(e,r)=>{try{n.d(t,{qw:()=>_.qw});var o=n(650),_=n(408),a=e([o]);o=(a.then?(await a)():a)[0],(0,_.lI)(o),r()}catch(e){r(e)}}))},408:(e,t,n)=>{let r;function o(e){r=e}n.d(t,{$y:()=>Ye,C$:()=>qe,DT:()=>Se,Dt:()=>ve,E0:()=>ze,EA:()=>se,FA:()=>Fe,FB:()=>be,Fm:()=>te,Fw:()=>Ue,IB:()=>le,IY:()=>z,Lm:()=>T,Lo:()=>ne,ND:()=>xe,Nh:()=>O,O$:()=>Ce,Oi:()=>G,P3:()=>X,PL:()=>we,Pv:()=>F,Py:()=>ee,Q4:()=>U,Q5:()=>N,QX:()=>K,Qb:()=>ye,Qn:()=>He,S9:()=>H,Tr:()=>Ae,US:()=>W,Us:()=>D,V8:()=>ue,Vd:()=>We,W:()=>de,WA:()=>Y,WE:()=>R,Wz:()=>re,X8:()=>Z,Xc:()=>me,Zd:()=>Xe,_E:()=>ae,bd:()=>V,bk:()=>B,c0:()=>ie,cO:()=>De,cf:()=>oe,dV:()=>Le,du:()=>Ie,fb:()=>ge,g_:()=>Me,lI:()=>o,lP:()=>Qe,lb:()=>ke,lq:()=>q,nv:()=>Je,ny:()=>M,pC:()=>fe,p_:()=>j,qC:()=>$,qN:()=>Ne,qT:()=>he,qW:()=>$e,qq:()=>pe,qw:()=>I,rl:()=>Ze,s$:()=>_e,sW:()=>Be,tb:()=>Oe,uZ:()=>C,vX:()=>Pe,vk:()=>je,w8:()=>ce,wb:()=>Ee,xO:()=>J,y$:()=>Q,yc:()=>Ve,zD:()=>Re,zi:()=>Te}),e=n.hmd(e);const _=new Array(128).fill(void 0);function a(e){return _[e]}_.push(void 0,null,!0,!1);let c=_.length;function i(e){const t=a(e);return function(e){e<132||(_[e]=c,c=e)}(e),t}function u(e){c===_.length&&_.push(_.length+1);const t=c;return c=_[t],_[t]=e,t}function f(e){return null==e}let d=null;function g(){return null!==d&&0!==d.byteLength||(d=new Float64Array(r.memory.buffer)),d}let b=null;function l(){return null!==b&&0!==b.byteLength||(b=new Int32Array(r.memory.buffer)),b}let s=0,w=null;function h(){return null!==w&&0!==w.byteLength||(w=new Uint8Array(r.memory.buffer)),w}let m=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const p="function"==typeof m.encodeInto?function(e,t){return m.encodeInto(e,t)}:function(e,t){const n=m.encode(e);return t.set(n),{read:e.length,written:n.length}};function y(e,t,n){if(void 0===n){const n=m.encode(e),r=t(n.length,1)>>>0;return h().subarray(r,r+n.length).set(n),s=n.length,r}let r=e.length,o=t(r,1)>>>0;const _=h();let a=0;for(;a<r;a++){const t=e.charCodeAt(a);if(t>127)break;_[o+a]=t}if(a!==r){0!==a&&(e=e.slice(a)),o=n(o,r,r=a+3*e.length,1)>>>0;const t=h().subarray(o+a,o+r);a+=p(e,t).written,o=n(o,r,a,1)>>>0}return s=a,o}let v=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});function S(e,t){return e>>>=0,v.decode(h().subarray(e,e+t))}function E(e){const t=typeof e;if("number"==t||"boolean"==t||null==e)return`${e}`;if("string"==t)return`"${e}"`;if("symbol"==t){const t=e.description;return null==t?"Symbol":`Symbol(${t})`}if("function"==t){const t=e.name;return"string"==typeof t&&t.length>0?`Function(${t})`:"Function"}if(Array.isArray(e)){const t=e.length;let n="[";t>0&&(n+=E(e[0]));for(let r=1;r<t;r++)n+=", "+E(e[r]);return n+="]",n}const n=/\[object ([^\]]+)\]/.exec(toString.call(e));let r;if(!(n.length>1))return toString.call(e);if(r=n[1],"Object"==r)try{return"Object("+JSON.stringify(e)+")"}catch(e){return"Object"}return e instanceof Error?`${e.name}: ${e.message}\n${e.stack}`:r}function x(e,t){const n=t(8*e.length,8)>>>0;return g().set(e,n/8),s=e.length,n}v.decode();let A=null;function P(e,t){return e>>>=0,(null!==A&&0!==A.byteLength||(A=new Float32Array(r.memory.buffer)),A).subarray(e/4,e/4+t)}function k(e,t){try{return e.apply(this,t)}catch(e){r.__wbindgen_exn_store(u(e))}}const L="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>r.__wbg_app_free(e>>>0)));class I{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,L.unregister(this),e}free(){const e=this.__destroy_into_raw();r.__wbg_app_free(e)}constructor(e){try{const o=r.__wbindgen_add_to_stack_pointer(-16);r.app_new(o,u(e));var t=l()[o/4+0],n=l()[o/4+1];if(l()[o/4+2])throw i(n);return this.__wbg_ptr=t>>>0,this}finally{r.__wbindgen_add_to_stack_pointer(16)}}change_c(e){r.app_change_c(this.__wbg_ptr,e)}reset_charge(e){const t=y(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=s;r.app_reset_charge(this.__wbg_ptr,t,n)}reset_grid(e){const t=y(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=s;r.app_reset_grid(this.__wbg_ptr,t,n)}change_poynting_on(e){r.app_change_poynting_on(this.__wbg_ptr,e)}change_arrow_length_factor(e){r.app_change_arrow_length_factor(this.__wbg_ptr,e)}change_arrow_length_log(e){r.app_change_arrow_length_log(this.__wbg_ptr,e)}key_down(e){const t=y(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=s;r.app_key_down(this.__wbg_ptr,t,n)}key_up(e){const t=y(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=s;r.app_key_up(this.__wbg_ptr,t,n)}window_blue(){r.app_window_blue(this.__wbg_ptr)}touch_start(e,t,n){const o=x(t,r.__wbindgen_malloc),_=s,a=x(n,r.__wbindgen_malloc),c=s;r.app_touch_start(this.__wbg_ptr,e,o,_,a,c)}touch_move(e,t,n){const o=x(t,r.__wbindgen_malloc),_=s,a=x(n,r.__wbindgen_malloc),c=s;r.app_touch_move(this.__wbg_ptr,e,o,_,a,c)}touch_end(e){r.app_touch_end(this.__wbg_ptr,e)}tick(e){try{const n=r.__wbindgen_add_to_stack_pointer(-16);r.app_tick(n,this.__wbg_ptr,e);var t=l()[n/4+0];if(l()[n/4+1])throw i(t)}finally{r.__wbindgen_add_to_stack_pointer(16)}}info(){let e,t;try{const _=r.__wbindgen_add_to_stack_pointer(-16);r.app_info(_,this.__wbg_ptr);var n=l()[_/4+0],o=l()[_/4+1];return e=n,t=o,S(n,o)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_free(e,t,1)}}}function B(e){i(e)}function D(e){a(e).flush()}function T(e){a(e).flush()}function q(e,t){a(e).deleteShader(a(t))}function F(e,t){a(e).deleteShader(a(t))}function O(e){const t=a(e);return"boolean"==typeof t?t?1:0:2}function W(e,t,n){a(e).detachShader(a(t),a(n))}function $(e,t,n){a(e).detachShader(a(t),a(n))}function C(e,t){a(e).useProgram(a(t))}function M(e,t){a(e).useProgram(a(t))}function N(e){const t=a(e).createBuffer();return f(t)?0:u(t)}function j(e){const t=a(e).createBuffer();return f(t)?0:u(t)}function U(e,t,n){a(e).bindBuffer(t>>>0,a(n))}function X(e,t,n){a(e).bindBuffer(t>>>0,a(n))}function Q(e,t){a(e).clear(t>>>0)}function V(e,t){a(e).clear(t>>>0)}function z(e,t,n,r){a(e).bufferData(t>>>0,a(n),r>>>0)}function Y(e,t,n,r){a(e).bufferData(t>>>0,a(n),r>>>0)}function Z(e,t,n,r,o){a(e).drawElements(t>>>0,n,r>>>0,o)}function H(e,t,n,r,o){a(e).drawElements(t>>>0,n,r>>>0,o)}function R(e,t,n,r){a(e).uniform4fv(a(t),P(n,r))}function J(e,t,n,r){a(e).uniform4fv(a(t),P(n,r))}function G(e,t,n,r,o){a(e).uniformMatrix4fv(a(t),0!==n,P(r,o))}function K(e,t,n,r,o){a(e).uniformMatrix4fv(a(t),0!==n,P(r,o))}function ee(){return u(r.memory)}function te(e){return u(a(e).buffer)}function ne(e,t,n){return u(new Uint8Array(a(e),t>>>0,n>>>0))}function re(e,t,n,r){return a(e).getAttribLocation(a(t),S(n,r))}function oe(e,t,n,r){return a(e).getAttribLocation(a(t),S(n,r))}function _e(e,t,n,r,o,_,c){a(e).vertexAttribPointer(t>>>0,n,r>>>0,0!==o,_,c)}function ae(e,t,n,r,o,_,c){a(e).vertexAttribPointer(t>>>0,n,r>>>0,0!==o,_,c)}function ce(e,t){a(e).enableVertexAttribArray(t>>>0)}function ie(e,t){a(e).enableVertexAttribArray(t>>>0)}function ue(e){const t=a(e).createProgram();return f(t)?0:u(t)}function fe(e){const t=a(e).createProgram();return f(t)?0:u(t)}function de(e,t){a(e).linkProgram(a(t))}function ge(e,t){a(e).linkProgram(a(t))}function be(e,t,n){return u(a(e).getProgramParameter(a(t),n>>>0))}function le(e,t,n){return u(a(e).getProgramParameter(a(t),n>>>0))}function se(e,t,n){const o=a(t).getProgramInfoLog(a(n));var _=f(o)?0:y(o,r.__wbindgen_malloc,r.__wbindgen_realloc),c=s;l()[e/4+1]=c,l()[e/4+0]=_}function we(e,t,n){const o=a(t).getProgramInfoLog(a(n));var _=f(o)?0:y(o,r.__wbindgen_malloc,r.__wbindgen_realloc),c=s;l()[e/4+1]=c,l()[e/4+0]=_}function he(e,t){const n=a(e).createShader(t>>>0);return f(n)?0:u(n)}function me(e,t){const n=a(e).createShader(t>>>0);return f(n)?0:u(n)}function pe(e,t,n,r){a(e).shaderSource(a(t),S(n,r))}function ye(e,t,n,r){a(e).shaderSource(a(t),S(n,r))}function ve(e,t){a(e).compileShader(a(t))}function Se(e,t){a(e).compileShader(a(t))}function Ee(e,t,n){return u(a(e).getShaderParameter(a(t),n>>>0))}function xe(e,t,n){return u(a(e).getShaderParameter(a(t),n>>>0))}function Ae(e,t,n){const o=a(t).getShaderInfoLog(a(n));var _=f(o)?0:y(o,r.__wbindgen_malloc,r.__wbindgen_realloc),c=s;l()[e/4+1]=c,l()[e/4+0]=_}function Pe(e,t,n){const o=a(t).getShaderInfoLog(a(n));var _=f(o)?0:y(o,r.__wbindgen_malloc,r.__wbindgen_realloc),c=s;l()[e/4+1]=c,l()[e/4+0]=_}function ke(e,t,n){a(e).attachShader(a(t),a(n))}function Le(e,t,n){a(e).attachShader(a(t),a(n))}function Ie(){return k((function(e,t){return u(a(e).getParameter(t>>>0))}),arguments)}function Be(e,t){const n=a(t),r="number"==typeof n?n:void 0;g()[e/8+1]=f(r)?0:r,l()[e/4+0]=!f(r)}function De(e){let t;try{t=a(e)instanceof Int32Array}catch(e){t=!1}return t}function Te(e){return a(e).length}function qe(e){return u(new Int32Array(a(e)))}function Fe(e,t,n){a(e).set(a(t),n>>>0)}function Oe(e,t,n,r){const o=a(e).getUniformLocation(a(t),S(n,r));return f(o)?0:u(o)}function We(e,t,n,r){const o=a(e).getUniformLocation(a(t),S(n,r));return f(o)?0:u(o)}function $e(e){const t=a(e).getSupportedExtensions();return f(t)?0:u(t)}function Ce(e){return a(e).length}function Me(e,t){return u(a(e)[t>>>0])}function Ne(e,t){const n=a(t),o="string"==typeof n?n:void 0;var _=f(o)?0:y(o,r.__wbindgen_malloc,r.__wbindgen_realloc),c=s;l()[e/4+1]=c,l()[e/4+0]=_}function je(e,t){a(e).enable(t>>>0)}function Ue(e,t,n,r,o){a(e).clearColor(t,n,r,o)}function Xe(e,t){a(e).enable(t>>>0)}function Qe(e,t,n,r,o){a(e).clearColor(t,n,r,o)}function Ve(e,t){return u(S(e,t))}function ze(e,t,n,r,o){a(e).uniformMatrix3fv(a(t),0!==n,P(r,o))}function Ye(e,t,n,r,o){a(e).uniformMatrix3fv(a(t),0!==n,P(r,o))}function Ze(e,t){const n=y(E(a(t)),r.__wbindgen_malloc,r.__wbindgen_realloc),o=s;l()[e/4+1]=o,l()[e/4+0]=n}function He(e,t){throw new Error(S(e,t))}function Re(){return k((function(e,t,n){const r=a(e).getExtension(S(t,n));return f(r)?0:u(r)}),arguments)}function Je(){return k((function(e,t){return u(a(e).getParameter(t>>>0))}),arguments)}},650:(e,t,n)=>{var r=n(408);e.exports=n.v(t,e.id,"b08cea8a705296e5bfc5",{"./index_bg.js":{__wbindgen_object_drop_ref:r.bk,__wbg_flush_aa1d651b876238a5:r.Us,__wbg_flush_dac98535ab343931:r.Lm,__wbg_deleteShader_138a810cc0ca9986:r.lq,__wbg_deleteShader_e5c778f25b722e68:r.Pv,__wbindgen_boolean_get:r.Nh,__wbg_detachShader_2be0011a543a788a:r.US,__wbg_detachShader_6cdc9c293ddee02e:r.qC,__wbg_useProgram_c637e43f9cd4c07a:r.uZ,__wbg_useProgram_757fab437af29c20:r.ny,__wbg_createBuffer_34e01f5c10929b41:r.Q5,__wbg_createBuffer_7f57647465d111f0:r.p_,__wbg_bindBuffer_1e5043751efddd4f:r.Q4,__wbg_bindBuffer_90d4fb91538001d5:r.P3,__wbg_clear_f9731a47df2e70d8:r.y$,__wbg_clear_8e2508724944df18:r.bd,__wbg_bufferData_5d1e6b8eaa7d23c8:r.IY,__wbg_bufferData_c787516945ba48c2:r.WA,__wbg_drawElements_0861624300587fcd:r.X8,__wbg_drawElements_565a93d1efa4da07:r.S9,__wbg_uniform4fv_980ce05d950ee599:r.WE,__wbg_uniform4fv_39cdcce4b1acc767:r.xO,__wbg_uniformMatrix4fv_cd46ed81bccb0cb2:r.Oi,__wbg_uniformMatrix4fv_5d8e0e047546456b:r.QX,__wbindgen_memory:r.Py,__wbg_buffer_12d079cc21e14bdb:r.Fm,__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb:r.Lo,__wbg_getAttribLocation_0a3d71a11394d043:r.Wz,__wbg_getAttribLocation_4e2b9fe88dcc9802:r.cf,__wbg_vertexAttribPointer_c25e4c5ed17f8a1d:r.s$,__wbg_vertexAttribPointer_4416f0325c02aa13:r._E,__wbg_enableVertexAttribArray_6d44444aa994f42a:r.w8,__wbg_enableVertexAttribArray_9d7b7e199f86e09b:r.c0,__wbg_createProgram_9affbfa62b7b2608:r.V8,__wbg_createProgram_7759fb2effb5d9b3:r.pC,__wbg_linkProgram_af5fed9dc3f1cdf9:r.W,__wbg_linkProgram_eabc664217816e72:r.fb,__wbg_getProgramParameter_10c8a43809fb8c2e:r.FB,__wbg_getProgramParameter_7b04ca71a79d9047:r.IB,__wbg_getProgramInfoLog_bf1fba8fa90667c7:r.EA,__wbg_getProgramInfoLog_4d189135f8d5a2de:r.PL,__wbg_createShader_55ca04b44164bd41:r.qT,__wbg_createShader_b474ef421ec0f80b:r.Xc,__wbg_shaderSource_7891a1fcb69a0023:r.qq,__wbg_shaderSource_7943d06f24862a3b:r.Qb,__wbg_compileShader_3af4719dfdb508e3:r.Dt,__wbg_compileShader_f40e0c51a7a836fd:r.DT,__wbg_getShaderParameter_60b69083e8d662ce:r.wb,__wbg_getShaderParameter_4ddb51279bb1500b:r.ND,__wbg_getShaderInfoLog_0262cb299092ce92:r.Tr,__wbg_getShaderInfoLog_d5de3e4eab06fc46:r.vX,__wbg_attachShader_6397dc4fd87343d3:r.lb,__wbg_attachShader_2112634b3ffa9e9f:r.dV,__wbg_getParameter_a77768abe8a51f24:r.du,__wbindgen_number_get:r.sW,__wbg_instanceof_Int32Array_f5ce6bdcd235ec41:r.cO,__wbg_length_58f3db6ca6f7dc3a:r.zi,__wbg_new_8cccba86b0f574cb:r.C$,__wbg_set_e3c5a1468be66841:r.FA,__wbg_getUniformLocation_6eedfb513ccce732:r.tb,__wbg_getUniformLocation_51ec30e3755e574d:r.Vd,__wbg_getSupportedExtensions_7a174085f9e1983a:r.qW,__wbg_length_cd7af8117672b8b8:r.O$,__wbg_get_bd8e338fbd5f5cc8:r.g_,__wbindgen_string_get:r.qN,__wbg_enable_7abe812a71c76206:r.vk,__wbg_clearColor_42707553c40e0e0f:r.Fw,__wbg_enable_8b3019da8846ce76:r.Zd,__wbg_clearColor_480962bfac4e1cbd:r.lP,__wbindgen_string_new:r.yc,__wbg_uniformMatrix3fv_d46553a1248946b5:r.E0,__wbg_uniformMatrix3fv_f26b98137276fd3d:r.$y,__wbindgen_debug_string:r.rl,__wbindgen_throw:r.Qn,__wbg_getExtension_bef4112494c87f34:r.zD,__wbg_getParameter_aa9af66884d2b210:r.nv}})}},_={};function a(e){var t=_[e];if(void 0!==t)return t.exports;var n=_[e]={id:e,loaded:!1,exports:{}};return o[e](n,n.exports,a),n.loaded=!0,n.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",n="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},a.a=(o,_,a)=>{var c;a&&((c=[]).d=-1);var i,u,f,d=new Set,g=o.exports,b=new Promise(((e,t)=>{f=t,u=e}));b[t]=g,b[e]=e=>(c&&e(c),d.forEach(e),b.catch((e=>{}))),o.exports=b,_((o=>{var _;i=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var _=[];_.d=0,o.then((e=>{a[t]=e,r(_)}),(e=>{a[n]=e,r(_)}));var a={};return a[e]=e=>e(_),a}}var c={};return c[e]=e=>{},c[t]=o,c})))(o);var a=()=>i.map((e=>{if(e[n])throw e[n];return e[t]})),u=new Promise((t=>{(_=()=>t(a)).r=0;var n=e=>e!==c&&!d.has(e)&&(d.add(e),e&&!e.d&&(_.r++,e.push(_)));i.map((t=>t[e](n)))}));return _.r?u:a()}),(e=>(e?f(b[n]=e):u(g),r(c)))),c&&c.d<0&&(c.d=0)},a.d=(e,t)=>{for(var n in t)a.o(t,n)&&!a.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},a.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),a.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),a.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),a.v=(e,t,n,r)=>{var o=fetch(a.p+""+n+".module.wasm"),_=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((t=>Object.assign(e,t.instance.exports)));return o.then((t=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(t,r).then((t=>Object.assign(e,t.instance.exports)),(e=>{if("application/wasm"!==t.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),_();throw e})):_()))},(()=>{var e;a.g.importScripts&&(e=a.g.location+"");var t=a.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var n=t.getElementsByTagName("script");if(n.length)for(var r=n.length-1;r>-1&&(!e||!/^http(s?):/.test(e));)e=n[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),a.p=e})(),a(73)})();