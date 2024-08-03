(()=>{"use strict";var e,t,n,r,o={73:(e,t,n)=>{n.a(e,(async(e,t)=>{try{var r=n(300),o=e([r]);r=(o.then?(await o)():o)[0];const _=document.getElementById("canvas");if(!_)throw new Error("No 'canvas'");const c=window.innerWidth||document.documentElement.clientWidth||document.body.clientWidth,a=window.innerHeight||document.documentElement.clientHeight||document.body.clientHeight;c<a?(_.width=c,_.height=.8*a):(_.width=1920,_.height=1080);const i=_.getContext("webgl2");if(!i)throw new Error("webgl2 not supported");i.viewport(0,0,_.width,_.height);const u=document.getElementById("info"),g=new r.qw(i);function d(e){const t=[],n=[];for(let r=0;r<e.touches.length;r++)t.push(e.touches[r].clientX),n.push(e.touches[r].clientY);return[new Float64Array(t),new Float64Array(n)]}document.addEventListener("keydown",(e=>{e.key.toLowerCase().startsWith("arrow")&&e.preventDefault(),g.key_down(e.key.toLowerCase())})),document.addEventListener("keyup",(e=>{g.key_up(e.key.toLowerCase())})),window.addEventListener("blur",(()=>{g.window_blue()})),_.addEventListener("touchstart",(e=>{e.preventDefault();const[t,n]=d(e);g.touch_start((new Date).getTime(),t,n)})),_.addEventListener("touchmove",(e=>{e.preventDefault();const[t,n]=d(e);g.touch_move((new Date).getTime(),t,n)})),_.addEventListener("touchend",(()=>{g.touch_end((new Date).getTime())}));let l=!1;_.addEventListener("mousedown",(e=>{l=!0,g.touch_start((new Date).getTime(),new Float64Array([e.clientX]),new Float64Array([e.clientY]))})),_.addEventListener("mousemove",(e=>{l&&g.touch_move((new Date).getTime(),new Float64Array([e.clientX]),new Float64Array([e.clientY]))})),_.addEventListener("mouseup",(()=>{l=!1,g.touch_end((new Date).getTime())})),_.addEventListener("mouseout",(()=>{l=!1,g.touch_end((new Date).getTime())}));const f=document.getElementById("restart-button"),s=()=>{f.classList.add("clicked"),setTimeout((()=>{f.classList.remove("clicked")}),500)};f.onclick=()=>{g.restart_physics(),s()};const b=document.getElementById("speed-of-light-exp"),w=document.getElementById("speed-of-light");function h(){const e=b.valueAsNumber;return Math.pow(2,e)}b.onchange=()=>{const e=h();w.innerText=`${e}`,g.change_c(e)&&s()};const m=document.getElementsByName("preset");function p(){for(let e=0;e<m.length;e++)m.item(e).checked?(g.reset_charge(m.item(e).value),m.item(e).nextElementSibling.classList.add("checked"),"circle"===m.item(e).value&&(E.checked=!1,S.checked=!0,g.change_electric_on(!1),g.change_magnetic_on(!0))):m.item(e).nextElementSibling.classList.remove("checked")}for(let $=0;$<m.length;$++)document.getElementById(`preset${$}`).onchange=p;const y=document.getElementsByName("grid-option"),v=()=>{for(let e=0;e<y.length;e++)y.item(e).checked?(g.reset_grid(y.item(e).value),y.item(e).nextElementSibling.classList.add("checked")):y.item(e).nextElementSibling.classList.remove("checked")};document.getElementById("grid-option1").onchange=v,document.getElementById("grid-option2").onchange=v;const E=document.getElementById("electric-toggle"),S=document.getElementById("magnetic-toggle"),A=document.getElementById("poynting-toggle");E.onchange=()=>{g.change_electric_on(E.checked)},S.onchange=()=>{g.change_magnetic_on(S.checked)},A.onchange=()=>{g.change_poynting_on(A.checked)};const k=document.getElementById("arrow-log"),x=document.getElementById("arrow-log-plus"),P=document.getElementById("arrow-log-minus");x.onclick=()=>{const e=k.valueAsNumber+1;g.change_arrow_length_log(e),k.value=`${e}`},P.onclick=()=>{const e=k.valueAsNumber>=1?k.valueAsNumber-1:0;g.change_arrow_length_log(e),k.value=`${e}`};const L=document.getElementById("arrow-factor"),I=document.getElementById("arrow-factor-plus"),B=document.getElementById("arrow-factor-minus");function T(e){g.tick(e),u.innerText=g.info(),window.requestAnimationFrame(T)}I.onclick=()=>{const e=L.valueAsNumber+1;g.change_arrow_length_factor(Math.pow(10,e)),L.value=`${e}`},B.onclick=()=>{const e=L.valueAsNumber-1;g.change_arrow_length_factor(Math.pow(10,e)),L.value=`${e}`},g.reset_charge(m.item(0).value),g.change_c(h()),g.change_arrow_length_log(k.valueAsNumber),g.change_arrow_length_factor(Math.pow(10,L.valueAsNumber)),window.requestAnimationFrame(T),t()}catch(D){t(D)}}))},300:(e,t,n)=>{n.a(e,(async(e,r)=>{try{n.d(t,{qw:()=>_.qw});var o=n(650),_=n(408),c=e([o]);o=(c.then?(await c)():c)[0],(0,_.lI)(o),r()}catch(e){r(e)}}))},408:(e,t,n)=>{let r;function o(e){r=e}n.d(t,{$y:()=>Ve,C$:()=>$e,DT:()=>ve,Dt:()=>ye,E0:()=>Qe,EA:()=>de,FA:()=>De,FB:()=>ue,Fm:()=>H,Fw:()=>ne,IB:()=>ge,IY:()=>z,Lm:()=>$,Lo:()=>R,ND:()=>Se,Nh:()=>F,O$:()=>G,Oi:()=>Ue,P3:()=>X,PL:()=>le,Pv:()=>q,Py:()=>Z,Q4:()=>U,Q5:()=>C,QX:()=>Xe,Qb:()=>pe,Qn:()=>He,S9:()=>Ye,Tr:()=>Ae,US:()=>O,Us:()=>T,V8:()=>_e,Vd:()=>Fe,W:()=>ae,WA:()=>Y,WE:()=>Ce,Wz:()=>se,X8:()=>ze,Xc:()=>he,Zd:()=>re,_E:()=>We,bd:()=>V,bk:()=>B,c0:()=>Me,cO:()=>Be,cf:()=>be,dV:()=>Pe,du:()=>Le,fb:()=>ie,g_:()=>K,lI:()=>o,lP:()=>oe,lb:()=>xe,lq:()=>D,nv:()=>Je,ny:()=>M,pC:()=>ce,p_:()=>j,qC:()=>W,qN:()=>ee,qT:()=>we,qW:()=>J,qq:()=>me,qw:()=>I,rl:()=>Ze,s$:()=>Oe,sW:()=>Ie,tb:()=>qe,uZ:()=>N,vX:()=>ke,vk:()=>te,w8:()=>Ne,wb:()=>Ee,xO:()=>je,y$:()=>Q,yc:()=>fe,zD:()=>Re,zi:()=>Te}),e=n.hmd(e);const _=new Array(128).fill(void 0);function c(e){return _[e]}_.push(void 0,null,!0,!1);let a=_.length;function i(e){const t=c(e);return function(e){e<132||(_[e]=a,a=e)}(e),t}function u(e){a===_.length&&_.push(_.length+1);const t=a;return a=_[t],_[t]=e,t}let g=0,d=null;function l(){return null!==d&&0!==d.byteLength||(d=new Uint8Array(r.memory.buffer)),d}let f=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const s="function"==typeof f.encodeInto?function(e,t){return f.encodeInto(e,t)}:function(e,t){const n=f.encode(e);return t.set(n),{read:e.length,written:n.length}};function b(e,t,n){if(void 0===n){const n=f.encode(e),r=t(n.length,1)>>>0;return l().subarray(r,r+n.length).set(n),g=n.length,r}let r=e.length,o=t(r,1)>>>0;const _=l();let c=0;for(;c<r;c++){const t=e.charCodeAt(c);if(t>127)break;_[o+c]=t}if(c!==r){0!==c&&(e=e.slice(c)),o=n(o,r,r=c+3*e.length,1)>>>0;const t=l().subarray(o+c,o+r);c+=s(e,t).written,o=n(o,r,c,1)>>>0}return g=c,o}function w(e){return null==e}let h=null;function m(){return null!==h&&0!==h.byteLength||(h=new Int32Array(r.memory.buffer)),h}let p=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});function y(e,t){return e>>>=0,p.decode(l().subarray(e,e+t))}p.decode();let v=null;function E(){return null!==v&&0!==v.byteLength||(v=new Float64Array(r.memory.buffer)),v}function S(e){const t=typeof e;if("number"==t||"boolean"==t||null==e)return`${e}`;if("string"==t)return`"${e}"`;if("symbol"==t){const t=e.description;return null==t?"Symbol":`Symbol(${t})`}if("function"==t){const t=e.name;return"string"==typeof t&&t.length>0?`Function(${t})`:"Function"}if(Array.isArray(e)){const t=e.length;let n="[";t>0&&(n+=S(e[0]));for(let r=1;r<t;r++)n+=", "+S(e[r]);return n+="]",n}const n=/\[object ([^\]]+)\]/.exec(toString.call(e));let r;if(!(n.length>1))return toString.call(e);if(r=n[1],"Object"==r)try{return"Object("+JSON.stringify(e)+")"}catch(e){return"Object"}return e instanceof Error?`${e.name}: ${e.message}\n${e.stack}`:r}function A(e,t){const n=t(8*e.length,8)>>>0;return E().set(e,n/8),g=e.length,n}let k=null;function x(e,t){return e>>>=0,(null!==k&&0!==k.byteLength||(k=new Float32Array(r.memory.buffer)),k).subarray(e/4,e/4+t)}function P(e,t){try{return e.apply(this,t)}catch(e){r.__wbindgen_exn_store(u(e))}}const L="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>r.__wbg_app_free(e>>>0)));class I{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,L.unregister(this),e}free(){const e=this.__destroy_into_raw();r.__wbg_app_free(e)}constructor(e){try{const o=r.__wbindgen_add_to_stack_pointer(-16);r.app_new(o,u(e));var t=m()[o/4+0],n=m()[o/4+1];if(m()[o/4+2])throw i(n);return this.__wbg_ptr=t>>>0,this}finally{r.__wbindgen_add_to_stack_pointer(16)}}restart_physics(){r.app_restart_physics(this.__wbg_ptr)}change_c(e){return 0!==r.app_change_c(this.__wbg_ptr,e)}reset_charge(e){const t=b(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=g;r.app_reset_charge(this.__wbg_ptr,t,n)}reset_grid(e){const t=b(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=g;r.app_reset_grid(this.__wbg_ptr,t,n)}change_electric_on(e){r.app_change_electric_on(this.__wbg_ptr,e)}change_magnetic_on(e){r.app_change_magnetic_on(this.__wbg_ptr,e)}change_poynting_on(e){r.app_change_poynting_on(this.__wbg_ptr,e)}change_arrow_length_factor(e){r.app_change_arrow_length_factor(this.__wbg_ptr,e)}change_arrow_length_log(e){r.app_change_arrow_length_log(this.__wbg_ptr,e)}key_down(e){const t=b(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=g;r.app_key_down(this.__wbg_ptr,t,n)}key_up(e){const t=b(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=g;r.app_key_up(this.__wbg_ptr,t,n)}window_blue(){r.app_window_blue(this.__wbg_ptr)}touch_start(e,t,n){const o=A(t,r.__wbindgen_malloc),_=g,c=A(n,r.__wbindgen_malloc),a=g;r.app_touch_start(this.__wbg_ptr,e,o,_,c,a)}touch_move(e,t,n){const o=A(t,r.__wbindgen_malloc),_=g,c=A(n,r.__wbindgen_malloc),a=g;r.app_touch_move(this.__wbg_ptr,e,o,_,c,a)}touch_end(e){r.app_touch_end(this.__wbg_ptr,e)}tick(e){try{const n=r.__wbindgen_add_to_stack_pointer(-16);r.app_tick(n,this.__wbg_ptr,e);var t=m()[n/4+0];if(m()[n/4+1])throw i(t)}finally{r.__wbindgen_add_to_stack_pointer(16)}}info(){let e,t;try{const _=r.__wbindgen_add_to_stack_pointer(-16);r.app_info(_,this.__wbg_ptr);var n=m()[_/4+0],o=m()[_/4+1];return e=n,t=o,y(n,o)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_free(e,t,1)}}}function B(e){i(e)}function T(e){c(e).flush()}function $(e){c(e).flush()}function D(e,t){c(e).deleteShader(c(t))}function q(e,t){c(e).deleteShader(c(t))}function F(e){const t=c(e);return"boolean"==typeof t?t?1:0:2}function O(e,t,n){c(e).detachShader(c(t),c(n))}function W(e,t,n){c(e).detachShader(c(t),c(n))}function N(e,t){c(e).useProgram(c(t))}function M(e,t){c(e).useProgram(c(t))}function C(e){const t=c(e).createBuffer();return w(t)?0:u(t)}function j(e){const t=c(e).createBuffer();return w(t)?0:u(t)}function U(e,t,n){c(e).bindBuffer(t>>>0,c(n))}function X(e,t,n){c(e).bindBuffer(t>>>0,c(n))}function Q(e,t){c(e).clear(t>>>0)}function V(e,t){c(e).clear(t>>>0)}function z(e,t,n,r){c(e).bufferData(t>>>0,c(n),r>>>0)}function Y(e,t,n,r){c(e).bufferData(t>>>0,c(n),r>>>0)}function Z(){return u(r.memory)}function H(e){return u(c(e).buffer)}function R(e,t,n){return u(new Uint8Array(c(e),t>>>0,n>>>0))}function J(e){const t=c(e).getSupportedExtensions();return w(t)?0:u(t)}function G(e){return c(e).length}function K(e,t){return u(c(e)[t>>>0])}function ee(e,t){const n=c(t),o="string"==typeof n?n:void 0;var _=w(o)?0:b(o,r.__wbindgen_malloc,r.__wbindgen_realloc),a=g;m()[e/4+1]=a,m()[e/4+0]=_}function te(e,t){c(e).enable(t>>>0)}function ne(e,t,n,r,o){c(e).clearColor(t,n,r,o)}function re(e,t){c(e).enable(t>>>0)}function oe(e,t,n,r,o){c(e).clearColor(t,n,r,o)}function _e(e){const t=c(e).createProgram();return w(t)?0:u(t)}function ce(e){const t=c(e).createProgram();return w(t)?0:u(t)}function ae(e,t){c(e).linkProgram(c(t))}function ie(e,t){c(e).linkProgram(c(t))}function ue(e,t,n){return u(c(e).getProgramParameter(c(t),n>>>0))}function ge(e,t,n){return u(c(e).getProgramParameter(c(t),n>>>0))}function de(e,t,n){const o=c(t).getProgramInfoLog(c(n));var _=w(o)?0:b(o,r.__wbindgen_malloc,r.__wbindgen_realloc),a=g;m()[e/4+1]=a,m()[e/4+0]=_}function le(e,t,n){const o=c(t).getProgramInfoLog(c(n));var _=w(o)?0:b(o,r.__wbindgen_malloc,r.__wbindgen_realloc),a=g;m()[e/4+1]=a,m()[e/4+0]=_}function fe(e,t){return u(y(e,t))}function se(e,t,n,r){return c(e).getAttribLocation(c(t),y(n,r))}function be(e,t,n,r){return c(e).getAttribLocation(c(t),y(n,r))}function we(e,t){const n=c(e).createShader(t>>>0);return w(n)?0:u(n)}function he(e,t){const n=c(e).createShader(t>>>0);return w(n)?0:u(n)}function me(e,t,n,r){c(e).shaderSource(c(t),y(n,r))}function pe(e,t,n,r){c(e).shaderSource(c(t),y(n,r))}function ye(e,t){c(e).compileShader(c(t))}function ve(e,t){c(e).compileShader(c(t))}function Ee(e,t,n){return u(c(e).getShaderParameter(c(t),n>>>0))}function Se(e,t,n){return u(c(e).getShaderParameter(c(t),n>>>0))}function Ae(e,t,n){const o=c(t).getShaderInfoLog(c(n));var _=w(o)?0:b(o,r.__wbindgen_malloc,r.__wbindgen_realloc),a=g;m()[e/4+1]=a,m()[e/4+0]=_}function ke(e,t,n){const o=c(t).getShaderInfoLog(c(n));var _=w(o)?0:b(o,r.__wbindgen_malloc,r.__wbindgen_realloc),a=g;m()[e/4+1]=a,m()[e/4+0]=_}function xe(e,t,n){c(e).attachShader(c(t),c(n))}function Pe(e,t,n){c(e).attachShader(c(t),c(n))}function Le(){return P((function(e,t){return u(c(e).getParameter(t>>>0))}),arguments)}function Ie(e,t){const n=c(t),r="number"==typeof n?n:void 0;E()[e/8+1]=w(r)?0:r,m()[e/4+0]=!w(r)}function Be(e){let t;try{t=c(e)instanceof Int32Array}catch(e){t=!1}return t}function Te(e){return c(e).length}function $e(e){return u(new Int32Array(c(e)))}function De(e,t,n){c(e).set(c(t),n>>>0)}function qe(e,t,n,r){const o=c(e).getUniformLocation(c(t),y(n,r));return w(o)?0:u(o)}function Fe(e,t,n,r){const o=c(e).getUniformLocation(c(t),y(n,r));return w(o)?0:u(o)}function Oe(e,t,n,r,o,_,a){c(e).vertexAttribPointer(t>>>0,n,r>>>0,0!==o,_,a)}function We(e,t,n,r,o,_,a){c(e).vertexAttribPointer(t>>>0,n,r>>>0,0!==o,_,a)}function Ne(e,t){c(e).enableVertexAttribArray(t>>>0)}function Me(e,t){c(e).enableVertexAttribArray(t>>>0)}function Ce(e,t,n,r){c(e).uniform4fv(c(t),x(n,r))}function je(e,t,n,r){c(e).uniform4fv(c(t),x(n,r))}function Ue(e,t,n,r,o){c(e).uniformMatrix4fv(c(t),0!==n,x(r,o))}function Xe(e,t,n,r,o){c(e).uniformMatrix4fv(c(t),0!==n,x(r,o))}function Qe(e,t,n,r,o){c(e).uniformMatrix3fv(c(t),0!==n,x(r,o))}function Ve(e,t,n,r,o){c(e).uniformMatrix3fv(c(t),0!==n,x(r,o))}function ze(e,t,n,r,o){c(e).drawElements(t>>>0,n,r>>>0,o)}function Ye(e,t,n,r,o){c(e).drawElements(t>>>0,n,r>>>0,o)}function Ze(e,t){const n=b(S(c(t)),r.__wbindgen_malloc,r.__wbindgen_realloc),o=g;m()[e/4+1]=o,m()[e/4+0]=n}function He(e,t){throw new Error(y(e,t))}function Re(){return P((function(e,t,n){const r=c(e).getExtension(y(t,n));return w(r)?0:u(r)}),arguments)}function Je(){return P((function(e,t){return u(c(e).getParameter(t>>>0))}),arguments)}},650:(e,t,n)=>{var r=n(408);e.exports=n.v(t,e.id,"e806ce214435d2d47d0c",{"./index_bg.js":{__wbindgen_object_drop_ref:r.bk,__wbg_flush_aa1d651b876238a5:r.Us,__wbg_flush_dac98535ab343931:r.Lm,__wbg_deleteShader_138a810cc0ca9986:r.lq,__wbg_deleteShader_e5c778f25b722e68:r.Pv,__wbindgen_boolean_get:r.Nh,__wbg_detachShader_2be0011a543a788a:r.US,__wbg_detachShader_6cdc9c293ddee02e:r.qC,__wbg_useProgram_c637e43f9cd4c07a:r.uZ,__wbg_useProgram_757fab437af29c20:r.ny,__wbg_createBuffer_34e01f5c10929b41:r.Q5,__wbg_createBuffer_7f57647465d111f0:r.p_,__wbg_bindBuffer_1e5043751efddd4f:r.Q4,__wbg_bindBuffer_90d4fb91538001d5:r.P3,__wbg_clear_f9731a47df2e70d8:r.y$,__wbg_clear_8e2508724944df18:r.bd,__wbg_bufferData_5d1e6b8eaa7d23c8:r.IY,__wbg_bufferData_c787516945ba48c2:r.WA,__wbindgen_memory:r.Py,__wbg_buffer_12d079cc21e14bdb:r.Fm,__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb:r.Lo,__wbg_getSupportedExtensions_7a174085f9e1983a:r.qW,__wbg_length_cd7af8117672b8b8:r.O$,__wbg_get_bd8e338fbd5f5cc8:r.g_,__wbindgen_string_get:r.qN,__wbg_enable_7abe812a71c76206:r.vk,__wbg_clearColor_42707553c40e0e0f:r.Fw,__wbg_enable_8b3019da8846ce76:r.Zd,__wbg_clearColor_480962bfac4e1cbd:r.lP,__wbg_createProgram_9affbfa62b7b2608:r.V8,__wbg_createProgram_7759fb2effb5d9b3:r.pC,__wbg_linkProgram_af5fed9dc3f1cdf9:r.W,__wbg_linkProgram_eabc664217816e72:r.fb,__wbg_getProgramParameter_10c8a43809fb8c2e:r.FB,__wbg_getProgramParameter_7b04ca71a79d9047:r.IB,__wbg_getProgramInfoLog_bf1fba8fa90667c7:r.EA,__wbg_getProgramInfoLog_4d189135f8d5a2de:r.PL,__wbindgen_string_new:r.yc,__wbg_getAttribLocation_0a3d71a11394d043:r.Wz,__wbg_getAttribLocation_4e2b9fe88dcc9802:r.cf,__wbg_createShader_55ca04b44164bd41:r.qT,__wbg_createShader_b474ef421ec0f80b:r.Xc,__wbg_shaderSource_7891a1fcb69a0023:r.qq,__wbg_shaderSource_7943d06f24862a3b:r.Qb,__wbg_compileShader_3af4719dfdb508e3:r.Dt,__wbg_compileShader_f40e0c51a7a836fd:r.DT,__wbg_getShaderParameter_60b69083e8d662ce:r.wb,__wbg_getShaderParameter_4ddb51279bb1500b:r.ND,__wbg_getShaderInfoLog_0262cb299092ce92:r.Tr,__wbg_getShaderInfoLog_d5de3e4eab06fc46:r.vX,__wbg_attachShader_6397dc4fd87343d3:r.lb,__wbg_attachShader_2112634b3ffa9e9f:r.dV,__wbg_getParameter_a77768abe8a51f24:r.du,__wbindgen_number_get:r.sW,__wbg_instanceof_Int32Array_f5ce6bdcd235ec41:r.cO,__wbg_length_58f3db6ca6f7dc3a:r.zi,__wbg_new_8cccba86b0f574cb:r.C$,__wbg_set_e3c5a1468be66841:r.FA,__wbg_getUniformLocation_6eedfb513ccce732:r.tb,__wbg_getUniformLocation_51ec30e3755e574d:r.Vd,__wbg_vertexAttribPointer_c25e4c5ed17f8a1d:r.s$,__wbg_vertexAttribPointer_4416f0325c02aa13:r._E,__wbg_enableVertexAttribArray_6d44444aa994f42a:r.w8,__wbg_enableVertexAttribArray_9d7b7e199f86e09b:r.c0,__wbg_uniform4fv_980ce05d950ee599:r.WE,__wbg_uniform4fv_39cdcce4b1acc767:r.xO,__wbg_uniformMatrix4fv_cd46ed81bccb0cb2:r.Oi,__wbg_uniformMatrix4fv_5d8e0e047546456b:r.QX,__wbg_uniformMatrix3fv_d46553a1248946b5:r.E0,__wbg_uniformMatrix3fv_f26b98137276fd3d:r.$y,__wbg_drawElements_0861624300587fcd:r.X8,__wbg_drawElements_565a93d1efa4da07:r.S9,__wbindgen_debug_string:r.rl,__wbindgen_throw:r.Qn,__wbg_getExtension_bef4112494c87f34:r.zD,__wbg_getParameter_aa9af66884d2b210:r.nv}})}},_={};function c(e){var t=_[e];if(void 0!==t)return t.exports;var n=_[e]={id:e,loaded:!1,exports:{}};return o[e](n,n.exports,c),n.loaded=!0,n.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",n="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},c.a=(o,_,c)=>{var a;c&&((a=[]).d=-1);var i,u,g,d=new Set,l=o.exports,f=new Promise(((e,t)=>{g=t,u=e}));f[t]=l,f[e]=e=>(a&&e(a),d.forEach(e),f.catch((e=>{}))),o.exports=f,_((o=>{var _;i=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var _=[];_.d=0,o.then((e=>{c[t]=e,r(_)}),(e=>{c[n]=e,r(_)}));var c={};return c[e]=e=>e(_),c}}var a={};return a[e]=e=>{},a[t]=o,a})))(o);var c=()=>i.map((e=>{if(e[n])throw e[n];return e[t]})),u=new Promise((t=>{(_=()=>t(c)).r=0;var n=e=>e!==a&&!d.has(e)&&(d.add(e),e&&!e.d&&(_.r++,e.push(_)));i.map((t=>t[e](n)))}));return _.r?u:c()}),(e=>(e?g(f[n]=e):u(l),r(a)))),a&&a.d<0&&(a.d=0)},c.d=(e,t)=>{for(var n in t)c.o(t,n)&&!c.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},c.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),c.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),c.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),c.v=(e,t,n,r)=>{var o=fetch(c.p+""+n+".module.wasm"),_=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((t=>Object.assign(e,t.instance.exports)));return o.then((t=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(t,r).then((t=>Object.assign(e,t.instance.exports)),(e=>{if("application/wasm"!==t.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),_();throw e})):_()))},(()=>{var e;c.g.importScripts&&(e=c.g.location+"");var t=c.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var n=t.getElementsByTagName("script");if(n.length)for(var r=n.length-1;r>-1&&(!e||!/^http(s?):/.test(e));)e=n[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),c.p=e})(),c(73)})();