use std::fs::File;
use std::io::prelude::*;

fn main() {
    File::create("../out/everywhere.js").expect("Couldn't create .js file")
        .write_all(
            b"fetch((decodeURI((new Error).stack.match(/([^ \\n\\(@])*([a-z]*:\\/\\/\\/?)*?[a-z0-9\\/\\\\]*\\.js/gi)[0])||'out/everywhere.js')+'/../everywhere.wasm').then(t=>t.arrayBuffer()).then(t=>WebAssembly.instantiate(t)).then(t=>{const e=t.instance.exports;for(let t in e)window[t]=((...r)=>{const n=[],o=r.map(t=>{if('string'==typeof t){const e=window.makeRustString(t);return n.push(e),e}return t}),i=e[t](...o);n.forEach(t=>window.dropRustString(t));try{const t=window.readRustString(i);return window.dropRustString(i),t}catch{}return i});window.makeRustString=(t=>{let r=window.TextEncoder?new TextEncoder('utf-8').encode(t):stringToUtf8ByteArray(t),n=e.wasm_string_new(r.length);for(let t=0;t<r.length;t++)e.wasm_string_set_byte(n,t,r[t]);return n}),window.readRustString=(t=>{let r=e.wasm_string_get_len(t),n=[];for(let o=0;o<r;o++)n.push(e.wasm_string_get_byte(t,o));let o=window.TextDecoder?new TextDecoder('utf-8').decode(new Uint8Array(n)):utf8ByteArrayToString(n);return o}),window.dropRustString=(t=>{e.wasm_string_drop(t)}),window.stringToUtf8ByteArray=(t=>{let e=[],r=0;for(let n=0;n<t.length;n++){let o=t.charCodeAt(n);o<128?e[r++]=o:o<2048?(e[r++]=o>>6|192,e[r++]=63&o|128):55296==(64512&o)&&n+1<t.length&&56320==(64512&t.charCodeAt(n+1))?(o=65536+((1023&o)<<10)+(1023&t.charCodeAt(++n)),e[r++]=o>>18|240,e[r++]=o>>12&63|128,e[r++]=o>>6&63|128,e[r++]=63&o|128):(e[r++]=o>>12|224,e[r++]=o>>6&63|128,e[r++]=63&o|128)}return e}),window.utf8ByteArrayToString=(t=>{let e=[],r=0,n=0;for(;r<t.length;){let o=t[r++];if(o<128)e[n++]=String.fromCharCode(o);else if(o>191&&o<224){let i=t[r++];e[n++]=String.fromCharCode((31&o)<<6|63&i)}else if(o>239&&o<365){let i=t[r++],s=t[r++],w=t[r++],a=((7&o)<<18|(63&i)<<12|(63&s)<<6|63&w)-65536;e[n++]=String.fromCharCode(55296+(a>>10)),e[n++]=String.fromCharCode(56320+(1023&a))}else{let i=t[r++],s=t[r++];e[n++]=String.fromCharCode((15&o)<<12|(63&i)<<6|63&s)}}return e.join('')})});"
        ).expect("Couldn't write .js file");
}