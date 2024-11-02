(()=>{"use strict";var e,t,n,r,o={73:(e,t,n)=>{n.a(e,(async(e,t)=>{try{var r=n(300),o=e([r]);r=(o.then?(await o)():o)[0];const c=document.getElementById("canvas");if(!c)throw new Error("No 'canvas'");const _=window.innerWidth||document.documentElement.clientWidth||document.body.clientWidth,a=window.innerHeight||document.documentElement.clientHeight||document.body.clientHeight;_<a?(c.width=_,c.height=.8*a):(c.width=1920,c.height=1080);const i=c.getContext("webgl2");if(!i)throw new Error("webgl2 not supported");i.viewport(0,0,c.width,c.height);const g=document.getElementById("info"),d=new r.qw(i),u={speedOfLight:2,electricOn:!0,magneticOn:!0,poyntingOn:!1,arrowLog:1,arrowFactor:2},l={eom_with_static:u,eom:u,static:{...u,speedOfLight:0},line_o:u,circle:{...u,electricOn:!1}},f=()=>{c.toBlob((e=>{if(!e)return void console.error("Fail to get canvas blob data");const t=new ClipboardItem({"image/png":e});navigator.clipboard.write([t]).catch((e=>{console.error("Failed to copy to clipboard: ",e)}))}))};document.addEventListener("keydown",(e=>{const t=e.key.toLowerCase();t.startsWith("arrow")&&e.preventDefault(),d.key_down(t)})),document.addEventListener("keyup",(e=>{d.key_up(e.key.toLowerCase())})),window.addEventListener("blur",(()=>{d.window_blue()}));const s=e=>{const t=[],n=[];for(let r=0;r<e.touches.length;r++)t.push(e.touches[r].clientX),n.push(e.touches[r].clientY);return[new Float64Array(t),new Float64Array(n)]};c.addEventListener("touchstart",(e=>{e.preventDefault();const[t,n]=s(e);d.touch_start((new Date).getTime(),t,n)})),c.addEventListener("touchmove",(e=>{e.preventDefault();const[t,n]=s(e);d.touch_move((new Date).getTime(),t,n)})),c.addEventListener("touchend",(()=>{d.touch_end((new Date).getTime())}));let b=!1;c.addEventListener("mousedown",(e=>{b=!0,d.touch_start((new Date).getTime(),new Float64Array([e.clientX]),new Float64Array([e.clientY])),e.ctrlKey&&(f(),b=!1)})),c.addEventListener("mousemove",(e=>{b&&d.touch_move((new Date).getTime(),new Float64Array([e.clientX]),new Float64Array([e.clientY]))})),c.addEventListener("mouseup",(()=>{b=!1,d.touch_end((new Date).getTime())})),c.addEventListener("mouseout",(()=>{b=!1,d.touch_end((new Date).getTime())}));const w=document.getElementById("restart-button"),h=()=>{w.classList.add("clicked"),setTimeout((()=>{w.classList.remove("clicked")}),500)};w.onclick=()=>{d.restart_physics(),h()};const m=document.getElementById("speed-of-light-exp"),p=document.getElementById("speed-of-light"),y=()=>{const e=m.valueAsNumber;return Math.pow(2,e)},v=()=>{const e=y();p.innerText=`${e}`,d.change_c(e)&&h()};m.onchange=v;const E=document.getElementById("lorentz");E.onchange=()=>{d.change_correct_lorentz(!E.checked)};const S=document.getElementsByName("preset"),k=()=>{for(let e=0;e<S.length;e++)S.item(e).checked?(M(S.item(e).value),S.item(e).nextElementSibling.classList.add("checked")):S.item(e).nextElementSibling.classList.remove("checked")};for(let e=0;e<S.length;e++)document.getElementById(`preset${e}`).onchange=k;const x=document.getElementsByName("grid-option"),A=()=>{for(let e=0;e<x.length;e++)x.item(e).checked?(d.reset_grid(x.item(e).value),x.item(e).nextElementSibling.classList.add("checked")):x.item(e).nextElementSibling.classList.remove("checked")};document.getElementById("grid-option1").onchange=A,document.getElementById("grid-option2").onchange=A;const P=document.getElementById("electric-toggle"),L=document.getElementById("magnetic-toggle"),I=document.getElementById("poynting-toggle"),B=e=>{P.checked=e,d.change_electric_on(e)},O=e=>{L.checked=e,d.change_magnetic_on(e)},F=e=>{I.checked=e,d.change_poynting_on(e)};P.onchange=()=>{d.change_electric_on(P.checked)},L.onchange=()=>{d.change_magnetic_on(L.checked)},I.onchange=()=>{d.change_poynting_on(I.checked)};const T=document.getElementById("arrow-log"),D=document.getElementById("arrow-log-plus"),q=document.getElementById("arrow-log-minus"),$=e=>{d.change_arrow_length_log(e),T.value=`${e}`};D.onclick=()=>{const e=T.valueAsNumber+1;$(e)},q.onclick=()=>{const e=T.valueAsNumber>=1?T.valueAsNumber-1:0;$(e)};const W=document.getElementById("arrow-factor"),N=document.getElementById("arrow-factor-plus"),C=document.getElementById("arrow-factor-minus"),j=e=>{d.change_arrow_length_factor(Math.pow(10,e)),W.value=`${e}`};N.onclick=()=>{const e=W.valueAsNumber+1;j(e)},C.onclick=()=>{const e=W.valueAsNumber-1;j(e)};const M=e=>{const t=l[e];m.valueAsNumber=t.speedOfLight,v(),$(t.arrowLog),j(t.arrowFactor),B(t.electricOn),O(t.magneticOn),F(t.poyntingOn),d.reset_charge(e)};M(S.item(0).value);const z=e=>{d.tick(e),g.innerText=d.info(),window.requestAnimationFrame(z)};window.requestAnimationFrame(z),t()}catch(e){t(e)}}))},300:(e,t,n)=>{n.a(e,(async(e,r)=>{try{n.d(t,{qw:()=>c.qw});var o=n(650),c=n(408),_=e([o]);o=(_.then?(await _)():_)[0],(0,c.lI)(o),r()}catch(e){r(e)}}))},408:(e,t,n)=>{let r;function o(e){r=e}n.d(t,{$y:()=>Le,C$:()=>be,DT:()=>oe,Dt:()=>re,E0:()=>Pe,EA:()=>Qe,FA:()=>we,FB:()=>Ue,Fm:()=>H,Fw:()=>$e,IB:()=>Xe,IY:()=>V,Lm:()=>F,Lo:()=>R,ND:()=>_e,Nh:()=>q,O$:()=>Fe,Oi:()=>xe,P3:()=>z,PL:()=>Ve,Pv:()=>T,Py:()=>Z,Q4:()=>U,Q5:()=>j,QX:()=>Ae,Qb:()=>ne,Qn:()=>He,S9:()=>Be,Tr:()=>ae,US:()=>$,Us:()=>O,V8:()=>Ce,Vd:()=>me,W:()=>Me,WA:()=>Y,WE:()=>Se,Wz:()=>J,X8:()=>Ie,Xc:()=>ee,Zd:()=>We,_E:()=>ye,bd:()=>Q,bk:()=>B,c0:()=>Ee,cO:()=>fe,cf:()=>K,dV:()=>de,du:()=>ue,fb:()=>ze,g_:()=>Te,lI:()=>o,lP:()=>Ne,lb:()=>ge,lq:()=>D,nv:()=>Je,ny:()=>N,pC:()=>je,p_:()=>M,qC:()=>W,qN:()=>De,qT:()=>G,qW:()=>Oe,qq:()=>te,qw:()=>I,rl:()=>Ze,s$:()=>pe,sW:()=>le,tb:()=>he,uZ:()=>C,vX:()=>ie,vk:()=>qe,w8:()=>ve,wb:()=>ce,xO:()=>ke,y$:()=>X,yc:()=>Ye,zD:()=>Re,zi:()=>se}),e=n.hmd(e);const c=new Array(128).fill(void 0);function _(e){return c[e]}c.push(void 0,null,!0,!1);let a=c.length;function i(e){const t=_(e);return function(e){e<132||(c[e]=a,a=e)}(e),t}function g(e){a===c.length&&c.push(c.length+1);const t=a;return a=c[t],c[t]=e,t}function d(e){return null==e}let u=null;function l(){return null!==u&&0!==u.byteLength||(u=new Float64Array(r.memory.buffer)),u}let f=null;function s(){return null!==f&&0!==f.byteLength||(f=new Int32Array(r.memory.buffer)),f}let b=0,w=null;function h(){return null!==w&&0!==w.byteLength||(w=new Uint8Array(r.memory.buffer)),w}let m=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const p="function"==typeof m.encodeInto?function(e,t){return m.encodeInto(e,t)}:function(e,t){const n=m.encode(e);return t.set(n),{read:e.length,written:n.length}};function y(e,t,n){if(void 0===n){const n=m.encode(e),r=t(n.length,1)>>>0;return h().subarray(r,r+n.length).set(n),b=n.length,r}let r=e.length,o=t(r,1)>>>0;const c=h();let _=0;for(;_<r;_++){const t=e.charCodeAt(_);if(t>127)break;c[o+_]=t}if(_!==r){0!==_&&(e=e.slice(_)),o=n(o,r,r=_+3*e.length,1)>>>0;const t=h().subarray(o+_,o+r);_+=p(e,t).written,o=n(o,r,_,1)>>>0}return b=_,o}let v=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});function E(e,t){return e>>>=0,v.decode(h().subarray(e,e+t))}function S(e){const t=typeof e;if("number"==t||"boolean"==t||null==e)return`${e}`;if("string"==t)return`"${e}"`;if("symbol"==t){const t=e.description;return null==t?"Symbol":`Symbol(${t})`}if("function"==t){const t=e.name;return"string"==typeof t&&t.length>0?`Function(${t})`:"Function"}if(Array.isArray(e)){const t=e.length;let n="[";t>0&&(n+=S(e[0]));for(let r=1;r<t;r++)n+=", "+S(e[r]);return n+="]",n}const n=/\[object ([^\]]+)\]/.exec(toString.call(e));let r;if(!(n.length>1))return toString.call(e);if(r=n[1],"Object"==r)try{return"Object("+JSON.stringify(e)+")"}catch(e){return"Object"}return e instanceof Error?`${e.name}: ${e.message}\n${e.stack}`:r}function k(e,t){const n=t(8*e.length,8)>>>0;return l().set(e,n/8),b=e.length,n}v.decode();let x=null;function A(e,t){return e>>>=0,(null!==x&&0!==x.byteLength||(x=new Float32Array(r.memory.buffer)),x).subarray(e/4,e/4+t)}function P(e,t){try{return e.apply(this,t)}catch(e){r.__wbindgen_exn_store(g(e))}}const L="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>r.__wbg_app_free(e>>>0)));class I{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,L.unregister(this),e}free(){const e=this.__destroy_into_raw();r.__wbg_app_free(e)}constructor(e){try{const o=r.__wbindgen_add_to_stack_pointer(-16);r.app_new(o,g(e));var t=s()[o/4+0],n=s()[o/4+1];if(s()[o/4+2])throw i(n);return this.__wbg_ptr=t>>>0,this}finally{r.__wbindgen_add_to_stack_pointer(16)}}restart_physics(){r.app_restart_physics(this.__wbg_ptr)}change_c(e){return 0!==r.app_change_c(this.__wbg_ptr,e)}reset_charge(e){const t=y(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=b;r.app_reset_charge(this.__wbg_ptr,t,n)}reset_grid(e){const t=y(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=b;r.app_reset_grid(this.__wbg_ptr,t,n)}change_correct_lorentz(e){r.app_change_correct_lorentz(this.__wbg_ptr,e)}change_electric_on(e){r.app_change_electric_on(this.__wbg_ptr,e)}change_magnetic_on(e){r.app_change_magnetic_on(this.__wbg_ptr,e)}change_poynting_on(e){r.app_change_poynting_on(this.__wbg_ptr,e)}change_arrow_length_factor(e){r.app_change_arrow_length_factor(this.__wbg_ptr,e)}change_arrow_length_log(e){r.app_change_arrow_length_log(this.__wbg_ptr,e)}key_down(e){const t=y(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=b;r.app_key_down(this.__wbg_ptr,t,n)}key_up(e){const t=y(e,r.__wbindgen_malloc,r.__wbindgen_realloc),n=b;r.app_key_up(this.__wbg_ptr,t,n)}window_blue(){r.app_window_blue(this.__wbg_ptr)}touch_start(e,t,n){const o=k(t,r.__wbindgen_malloc),c=b,_=k(n,r.__wbindgen_malloc),a=b;r.app_touch_start(this.__wbg_ptr,e,o,c,_,a)}touch_move(e,t,n){const o=k(t,r.__wbindgen_malloc),c=b,_=k(n,r.__wbindgen_malloc),a=b;r.app_touch_move(this.__wbg_ptr,e,o,c,_,a)}touch_end(e){r.app_touch_end(this.__wbg_ptr,e)}tick(e){try{const n=r.__wbindgen_add_to_stack_pointer(-16);r.app_tick(n,this.__wbg_ptr,e);var t=s()[n/4+0];if(s()[n/4+1])throw i(t)}finally{r.__wbindgen_add_to_stack_pointer(16)}}info(){let e,t;try{const c=r.__wbindgen_add_to_stack_pointer(-16);r.app_info(c,this.__wbg_ptr);var n=s()[c/4+0],o=s()[c/4+1];return e=n,t=o,E(n,o)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_free(e,t,1)}}}function B(e){i(e)}function O(e){_(e).flush()}function F(e){_(e).flush()}function T(e,t){_(e).deleteShader(_(t))}function D(e,t){_(e).deleteShader(_(t))}function q(e){const t=_(e);return"boolean"==typeof t?t?1:0:2}function $(e,t,n){_(e).detachShader(_(t),_(n))}function W(e,t,n){_(e).detachShader(_(t),_(n))}function N(e,t){_(e).useProgram(_(t))}function C(e,t){_(e).useProgram(_(t))}function j(e){const t=_(e).createBuffer();return d(t)?0:g(t)}function M(e){const t=_(e).createBuffer();return d(t)?0:g(t)}function z(e,t,n){_(e).bindBuffer(t>>>0,_(n))}function U(e,t,n){_(e).bindBuffer(t>>>0,_(n))}function X(e,t){_(e).clear(t>>>0)}function Q(e,t){_(e).clear(t>>>0)}function V(e,t,n,r){_(e).bufferData(t>>>0,_(n),r>>>0)}function Y(e,t,n,r){_(e).bufferData(t>>>0,_(n),r>>>0)}function Z(){return g(r.memory)}function H(e){return g(_(e).buffer)}function R(e,t,n){return g(new Uint8Array(_(e),t>>>0,n>>>0))}function J(e,t,n,r){return _(e).getAttribLocation(_(t),E(n,r))}function K(e,t,n,r){return _(e).getAttribLocation(_(t),E(n,r))}function G(e,t){const n=_(e).createShader(t>>>0);return d(n)?0:g(n)}function ee(e,t){const n=_(e).createShader(t>>>0);return d(n)?0:g(n)}function te(e,t,n,r){_(e).shaderSource(_(t),E(n,r))}function ne(e,t,n,r){_(e).shaderSource(_(t),E(n,r))}function re(e,t){_(e).compileShader(_(t))}function oe(e,t){_(e).compileShader(_(t))}function ce(e,t,n){return g(_(e).getShaderParameter(_(t),n>>>0))}function _e(e,t,n){return g(_(e).getShaderParameter(_(t),n>>>0))}function ae(e,t,n){const o=_(t).getShaderInfoLog(_(n));var c=d(o)?0:y(o,r.__wbindgen_malloc,r.__wbindgen_realloc),a=b;s()[e/4+1]=a,s()[e/4+0]=c}function ie(e,t,n){const o=_(t).getShaderInfoLog(_(n));var c=d(o)?0:y(o,r.__wbindgen_malloc,r.__wbindgen_realloc),a=b;s()[e/4+1]=a,s()[e/4+0]=c}function ge(e,t,n){_(e).attachShader(_(t),_(n))}function de(e,t,n){_(e).attachShader(_(t),_(n))}function ue(){return P((function(e,t){return g(_(e).getParameter(t>>>0))}),arguments)}function le(e,t){const n=_(t),r="number"==typeof n?n:void 0;l()[e/8+1]=d(r)?0:r,s()[e/4+0]=!d(r)}function fe(e){let t;try{t=_(e)instanceof Int32Array}catch(e){t=!1}return t}function se(e){return _(e).length}function be(e){return g(new Int32Array(_(e)))}function we(e,t,n){_(e).set(_(t),n>>>0)}function he(e,t,n,r){const o=_(e).getUniformLocation(_(t),E(n,r));return d(o)?0:g(o)}function me(e,t,n,r){const o=_(e).getUniformLocation(_(t),E(n,r));return d(o)?0:g(o)}function pe(e,t,n,r,o,c,a){_(e).vertexAttribPointer(t>>>0,n,r>>>0,0!==o,c,a)}function ye(e,t,n,r,o,c,a){_(e).vertexAttribPointer(t>>>0,n,r>>>0,0!==o,c,a)}function ve(e,t){_(e).enableVertexAttribArray(t>>>0)}function Ee(e,t){_(e).enableVertexAttribArray(t>>>0)}function Se(e,t,n,r){_(e).uniform4fv(_(t),A(n,r))}function ke(e,t,n,r){_(e).uniform4fv(_(t),A(n,r))}function xe(e,t,n,r,o){_(e).uniformMatrix4fv(_(t),0!==n,A(r,o))}function Ae(e,t,n,r,o){_(e).uniformMatrix4fv(_(t),0!==n,A(r,o))}function Pe(e,t,n,r,o){_(e).uniformMatrix3fv(_(t),0!==n,A(r,o))}function Le(e,t,n,r,o){_(e).uniformMatrix3fv(_(t),0!==n,A(r,o))}function Ie(e,t,n,r,o){_(e).drawElements(t>>>0,n,r>>>0,o)}function Be(e,t,n,r,o){_(e).drawElements(t>>>0,n,r>>>0,o)}function Oe(e){const t=_(e).getSupportedExtensions();return d(t)?0:g(t)}function Fe(e){return _(e).length}function Te(e,t){return g(_(e)[t>>>0])}function De(e,t){const n=_(t),o="string"==typeof n?n:void 0;var c=d(o)?0:y(o,r.__wbindgen_malloc,r.__wbindgen_realloc),a=b;s()[e/4+1]=a,s()[e/4+0]=c}function qe(e,t){_(e).enable(t>>>0)}function $e(e,t,n,r,o){_(e).clearColor(t,n,r,o)}function We(e,t){_(e).enable(t>>>0)}function Ne(e,t,n,r,o){_(e).clearColor(t,n,r,o)}function Ce(e){const t=_(e).createProgram();return d(t)?0:g(t)}function je(e){const t=_(e).createProgram();return d(t)?0:g(t)}function Me(e,t){_(e).linkProgram(_(t))}function ze(e,t){_(e).linkProgram(_(t))}function Ue(e,t,n){return g(_(e).getProgramParameter(_(t),n>>>0))}function Xe(e,t,n){return g(_(e).getProgramParameter(_(t),n>>>0))}function Qe(e,t,n){const o=_(t).getProgramInfoLog(_(n));var c=d(o)?0:y(o,r.__wbindgen_malloc,r.__wbindgen_realloc),a=b;s()[e/4+1]=a,s()[e/4+0]=c}function Ve(e,t,n){const o=_(t).getProgramInfoLog(_(n));var c=d(o)?0:y(o,r.__wbindgen_malloc,r.__wbindgen_realloc),a=b;s()[e/4+1]=a,s()[e/4+0]=c}function Ye(e,t){return g(E(e,t))}function Ze(e,t){const n=y(S(_(t)),r.__wbindgen_malloc,r.__wbindgen_realloc),o=b;s()[e/4+1]=o,s()[e/4+0]=n}function He(e,t){throw new Error(E(e,t))}function Re(){return P((function(e,t,n){const r=_(e).getExtension(E(t,n));return d(r)?0:g(r)}),arguments)}function Je(){return P((function(e,t){return g(_(e).getParameter(t>>>0))}),arguments)}},650:(e,t,n)=>{var r=n(408);e.exports=n.v(t,e.id,"d60412023d8306b1cfc6",{"./index_bg.js":{__wbindgen_object_drop_ref:r.bk,__wbg_flush_aa1d651b876238a5:r.Us,__wbg_flush_dac98535ab343931:r.Lm,__wbg_deleteShader_e5c778f25b722e68:r.Pv,__wbg_deleteShader_138a810cc0ca9986:r.lq,__wbindgen_boolean_get:r.Nh,__wbg_detachShader_2be0011a543a788a:r.US,__wbg_detachShader_6cdc9c293ddee02e:r.qC,__wbg_useProgram_757fab437af29c20:r.ny,__wbg_useProgram_c637e43f9cd4c07a:r.uZ,__wbg_createBuffer_34e01f5c10929b41:r.Q5,__wbg_createBuffer_7f57647465d111f0:r.p_,__wbg_bindBuffer_90d4fb91538001d5:r.P3,__wbg_bindBuffer_1e5043751efddd4f:r.Q4,__wbg_clear_f9731a47df2e70d8:r.y$,__wbg_clear_8e2508724944df18:r.bd,__wbg_bufferData_5d1e6b8eaa7d23c8:r.IY,__wbg_bufferData_c787516945ba48c2:r.WA,__wbindgen_memory:r.Py,__wbg_buffer_12d079cc21e14bdb:r.Fm,__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb:r.Lo,__wbg_getAttribLocation_0a3d71a11394d043:r.Wz,__wbg_getAttribLocation_4e2b9fe88dcc9802:r.cf,__wbg_createShader_55ca04b44164bd41:r.qT,__wbg_createShader_b474ef421ec0f80b:r.Xc,__wbg_shaderSource_7891a1fcb69a0023:r.qq,__wbg_shaderSource_7943d06f24862a3b:r.Qb,__wbg_compileShader_3af4719dfdb508e3:r.Dt,__wbg_compileShader_f40e0c51a7a836fd:r.DT,__wbg_getShaderParameter_60b69083e8d662ce:r.wb,__wbg_getShaderParameter_4ddb51279bb1500b:r.ND,__wbg_getShaderInfoLog_0262cb299092ce92:r.Tr,__wbg_getShaderInfoLog_d5de3e4eab06fc46:r.vX,__wbg_attachShader_6397dc4fd87343d3:r.lb,__wbg_attachShader_2112634b3ffa9e9f:r.dV,__wbg_getParameter_a77768abe8a51f24:r.du,__wbindgen_number_get:r.sW,__wbg_instanceof_Int32Array_f5ce6bdcd235ec41:r.cO,__wbg_length_58f3db6ca6f7dc3a:r.zi,__wbg_new_8cccba86b0f574cb:r.C$,__wbg_set_e3c5a1468be66841:r.FA,__wbg_getUniformLocation_6eedfb513ccce732:r.tb,__wbg_getUniformLocation_51ec30e3755e574d:r.Vd,__wbg_vertexAttribPointer_c25e4c5ed17f8a1d:r.s$,__wbg_vertexAttribPointer_4416f0325c02aa13:r._E,__wbg_enableVertexAttribArray_6d44444aa994f42a:r.w8,__wbg_enableVertexAttribArray_9d7b7e199f86e09b:r.c0,__wbg_uniform4fv_980ce05d950ee599:r.WE,__wbg_uniform4fv_39cdcce4b1acc767:r.xO,__wbg_uniformMatrix4fv_cd46ed81bccb0cb2:r.Oi,__wbg_uniformMatrix4fv_5d8e0e047546456b:r.QX,__wbg_uniformMatrix3fv_d46553a1248946b5:r.E0,__wbg_uniformMatrix3fv_f26b98137276fd3d:r.$y,__wbg_drawElements_0861624300587fcd:r.X8,__wbg_drawElements_565a93d1efa4da07:r.S9,__wbg_getSupportedExtensions_7a174085f9e1983a:r.qW,__wbg_length_cd7af8117672b8b8:r.O$,__wbg_get_bd8e338fbd5f5cc8:r.g_,__wbindgen_string_get:r.qN,__wbg_enable_7abe812a71c76206:r.vk,__wbg_clearColor_42707553c40e0e0f:r.Fw,__wbg_enable_8b3019da8846ce76:r.Zd,__wbg_clearColor_480962bfac4e1cbd:r.lP,__wbg_createProgram_9affbfa62b7b2608:r.V8,__wbg_createProgram_7759fb2effb5d9b3:r.pC,__wbg_linkProgram_af5fed9dc3f1cdf9:r.W,__wbg_linkProgram_eabc664217816e72:r.fb,__wbg_getProgramParameter_10c8a43809fb8c2e:r.FB,__wbg_getProgramParameter_7b04ca71a79d9047:r.IB,__wbg_getProgramInfoLog_bf1fba8fa90667c7:r.EA,__wbg_getProgramInfoLog_4d189135f8d5a2de:r.PL,__wbindgen_string_new:r.yc,__wbindgen_debug_string:r.rl,__wbindgen_throw:r.Qn,__wbg_getExtension_bef4112494c87f34:r.zD,__wbg_getParameter_aa9af66884d2b210:r.nv}})}},c={};function _(e){var t=c[e];if(void 0!==t)return t.exports;var n=c[e]={id:e,loaded:!1,exports:{}};return o[e](n,n.exports,_),n.loaded=!0,n.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",n="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},_.a=(o,c,_)=>{var a;_&&((a=[]).d=-1);var i,g,d,u=new Set,l=o.exports,f=new Promise(((e,t)=>{d=t,g=e}));f[t]=l,f[e]=e=>(a&&e(a),u.forEach(e),f.catch((e=>{}))),o.exports=f,c((o=>{var c;i=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var c=[];c.d=0,o.then((e=>{_[t]=e,r(c)}),(e=>{_[n]=e,r(c)}));var _={};return _[e]=e=>e(c),_}}var a={};return a[e]=e=>{},a[t]=o,a})))(o);var _=()=>i.map((e=>{if(e[n])throw e[n];return e[t]})),g=new Promise((t=>{(c=()=>t(_)).r=0;var n=e=>e!==a&&!u.has(e)&&(u.add(e),e&&!e.d&&(c.r++,e.push(c)));i.map((t=>t[e](n)))}));return c.r?g:_()}),(e=>(e?d(f[n]=e):g(l),r(a)))),a&&a.d<0&&(a.d=0)},_.d=(e,t)=>{for(var n in t)_.o(t,n)&&!_.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},_.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),_.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),_.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),_.v=(e,t,n,r)=>{var o=fetch(_.p+""+n+".module.wasm"),c=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((t=>Object.assign(e,t.instance.exports)));return o.then((t=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(t,r).then((t=>Object.assign(e,t.instance.exports)),(e=>{if("application/wasm"!==t.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),c();throw e})):c()))},(()=>{var e;_.g.importScripts&&(e=_.g.location+"");var t=_.g.document;if(!e&&t&&(t.currentScript&&"SCRIPT"===t.currentScript.tagName.toUpperCase()&&(e=t.currentScript.src),!e)){var n=t.getElementsByTagName("script");if(n.length)for(var r=n.length-1;r>-1&&(!e||!/^http(s?):/.test(e));)e=n[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),_.p=e})(),_(73)})();