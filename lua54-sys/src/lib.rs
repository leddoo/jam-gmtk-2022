#![allow(non_snake_case)]

use std::os::raw::{c_char, c_int, c_void};


#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod bindings;



// luaconf.h
pub const LUA_EXTRASPACE: usize = std::mem::size_of::<*const c_void>();


// lua.h

// version
pub use bindings::{
    LUA_VERSION_MAJOR,
    LUA_VERSION_MINOR,
    LUA_VERSION_RELEASE,

    LUA_VERSION_NUM,
    LUA_VERSION_RELEASE_NUM,

    LUA_VERSION,
    LUA_RELEASE,
    LUA_COPYRIGHT,
    LUA_AUTHORS,

    LUA_SIGNATURE,

    LUA_MULTRET,
};



/*
** Pseudo-indices
** (-LUAI_MAXSTACK is the minimum valid index; we keep some free empty
** space after that to help overflow detection)
*/
pub use bindings::LUA_REGISTRYINDEX;
#[inline]
pub fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_REGISTRYINDEX - i
}


// thread status
pub const LUA_OK:        c_int = bindings::LUA_OK as c_int;
pub const LUA_YIELD:     c_int = bindings::LUA_YIELD as c_int;
pub const LUA_ERRRUN:    c_int = bindings::LUA_ERRRUN as c_int;
pub const LUA_ERRSYNTAX: c_int = bindings::LUA_ERRSYNTAX as c_int;
pub const LUA_ERRMEM:    c_int = bindings::LUA_ERRMEM as c_int;
pub const LUA_ERRERR:    c_int = bindings::LUA_ERRERR as c_int;


pub use bindings::lua_State;


// basic types
pub const LUA_TNONE: c_int = bindings::LUA_TNONE as c_int;

pub const LUA_TNIL:           c_int = bindings::LUA_TNIL as c_int;
pub const LUA_TBOOLEAN:       c_int = bindings::LUA_TBOOLEAN as c_int;
pub const LUA_TLIGHTUSERDATA: c_int = bindings::LUA_TLIGHTUSERDATA as c_int;
pub const LUA_TNUMBER:        c_int = bindings::LUA_TNUMBER as c_int;
pub const LUA_TSTRING:        c_int = bindings::LUA_TSTRING as c_int;
pub const LUA_TTABLE:         c_int = bindings::LUA_TTABLE as c_int;
pub const LUA_TFUNCTION:      c_int = bindings::LUA_TFUNCTION as c_int;
pub const LUA_TUSERDATA:      c_int = bindings::LUA_TUSERDATA as c_int;
pub const LUA_TTHREAD:        c_int = bindings::LUA_TTHREAD as c_int;

pub const LUA_NUM_TYPES: c_int = bindings::LUA_NUMTYPES as c_int;


/* minimum Lua stack available to a C function */
pub const LUA_MINSTACK: c_int = bindings::LUA_MINSTACK as c_int;


/* predefined values in the registry */
pub const LUA_RIDX_MAINTHREAD: c_int = bindings::LUA_RIDX_MAINTHREAD as c_int;
pub const LUA_RIDX_GLOBALS:    c_int = bindings::LUA_RIDX_GLOBALS as c_int;
pub const LUA_RIDX_LAST:       c_int = bindings::LUA_RIDX_LAST as c_int;



/* type of numbers in Lua */
pub use bindings::lua_Number;


/* type for integer functions */
pub use bindings::lua_Integer;

/* unsigned integer type */
pub use bindings::lua_Unsigned;

/* type for continuation-function contexts */
pub use bindings::lua_KContext;



/*
** Type for C functions registered with Lua
*/
pub use bindings::lua_CFunction;

/*
** Type for continuation functions
*/
pub use bindings::lua_KFunction;

/*
** Type for functions that read/write blocks when loading/dumping Lua chunks
*/
pub use bindings::lua_Reader;
pub use bindings::lua_Writer;


/*
** Type for memory-allocation functions
*/
pub use bindings::lua_Alloc;


/*
** Type for warning functions
*/
pub use bindings::lua_WarnFunction;




/*
** RCS ident string
*/
pub use bindings::lua_ident;


/*
** state manipulation
*/
pub use bindings::{
    lua_newstate,
    lua_close,
    lua_newthread,
    lua_resetthread,

    lua_atpanic,

    lua_version,
};


/*
** basic stack manipulation
*/
pub use bindings::{
    lua_absindex,
    lua_gettop,
    lua_settop,
    lua_pushvalue,
    lua_rotate,
    lua_copy,
    lua_checkstack,

    lua_xmove,
};


/*
** access functions (stack -> C)
*/
pub use bindings::{
    lua_isnumber,
    lua_isstring,
    lua_iscfunction,
    lua_isinteger,
    lua_isuserdata,
    lua_type,
    lua_typename,

    lua_tonumberx,
    lua_tointegerx,
    lua_toboolean,
    lua_tolstring,
    lua_rawlen,
    lua_tocfunction,
    lua_touserdata,
    lua_tothread,
    lua_topointer,
};


/*
** Comparison and arithmetic functions
*/

pub const LUA_OPADD: c_int = bindings::LUA_OPADD as c_int;
pub const LUA_OPSUB: c_int = bindings::LUA_OPSUB as c_int;
pub const LUA_OPMUL: c_int = bindings::LUA_OPMUL as c_int;
pub const LUA_OPMOD: c_int = bindings::LUA_OPMOD as c_int;
pub const LUA_OPPOW: c_int = bindings::LUA_OPPOW as c_int;
pub const LUA_OPDIV: c_int = bindings::LUA_OPDIV as c_int;
pub const LUA_OPIDIV: c_int = bindings::LUA_OPIDIV as c_int;
pub const LUA_OPBAND: c_int = bindings::LUA_OPBAND as c_int;
pub const LUA_OPBOR: c_int = bindings::LUA_OPBOR as c_int;
pub const LUA_OPBXOR: c_int = bindings::LUA_OPBXOR as c_int;
pub const LUA_OPSHL: c_int = bindings::LUA_OPSHL as c_int;
pub const LUA_OPSHR: c_int = bindings::LUA_OPSHR as c_int;
pub const LUA_OPUNM: c_int = bindings::LUA_OPUNM as c_int;
pub const LUA_OPBNOT: c_int = bindings::LUA_OPBNOT as c_int;

pub use bindings::lua_arith;

pub const LUA_OPEQ: c_int = bindings::LUA_OPEQ as c_int;
pub const LUA_OPLT: c_int = bindings::LUA_OPLT as c_int;
pub const LUA_OPLE: c_int = bindings::LUA_OPLE as c_int;

pub use bindings::lua_rawequal;
pub use bindings::lua_compare;



/*
** push functions (C -> stack)
*/
pub use bindings::{
    lua_pushnil,
    lua_pushnumber,
    lua_pushinteger,
    lua_pushlstring,
    lua_pushstring,
    lua_pushvfstring,
    lua_pushfstring,
    lua_pushcclosure,
    lua_pushboolean,
    lua_pushlightuserdata,
    lua_pushthread,
};


/*
** get functions (Lua -> stack)
*/
pub use bindings::{
    lua_getglobal,
    lua_gettable,
    lua_getfield,
    lua_geti,
    lua_rawget,
    lua_rawgeti,
    lua_rawgetp,

    lua_createtable,
    lua_newuserdatauv,
    lua_getmetatable,
    lua_getiuservalue,
};


/*
** set functions (stack -> Lua)
*/
pub use bindings::{
    lua_setglobal,
    lua_settable,
    lua_setfield,
    lua_seti,
    lua_rawset,
    lua_rawseti,
    lua_rawsetp,
    lua_setmetatable,
    lua_setiuservalue,
};


/*
** 'load' and 'call' functions (load and run Lua code)
*/
pub use bindings::lua_callk;

#[inline]
pub unsafe fn lua_call(
    L: *mut lua_State,
    nargs: ::std::os::raw::c_int,
    nresults: ::std::os::raw::c_int,
) {
    lua_callk(L, nargs, nresults, 0, None)
}

pub use bindings::lua_pcallk;

#[inline]
pub unsafe fn lua_pcall(
    L: *mut lua_State,
    nargs: ::std::os::raw::c_int,
    nresults: ::std::os::raw::c_int,
    errfunc: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    lua_pcallk(L, nargs, nresults, errfunc, 0, None)
}

pub use bindings::lua_load;
pub use bindings::lua_dump;


/*
** coroutine functions
*/
pub use bindings::{
    lua_yieldk,
    lua_resume,
    lua_status,
    lua_isyieldable,
};

#[inline]
pub unsafe fn lua_yield(
    L: *mut lua_State,
    nresults: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    lua_yieldk(L, nresults, 0, None)
}


/*
** Warning-related functions
*/
pub use bindings::{
    lua_setwarnf,
    lua_warning,
};


/*
** garbage-collection function and options
*/

pub const LUA_GCSTOP: c_int = bindings::LUA_GCSTOP as c_int;
pub const LUA_GCRESTART: c_int = bindings::LUA_GCRESTART as c_int;
pub const LUA_GCCOLLECT: c_int = bindings::LUA_GCCOLLECT as c_int;
pub const LUA_GCCOUNT: c_int = bindings::LUA_GCCOUNT as c_int;
pub const LUA_GCCOUNTB: c_int = bindings::LUA_GCCOUNTB as c_int;
pub const LUA_GCSTEP: c_int = bindings::LUA_GCSTEP as c_int;
pub const LUA_GCSETPAUSE: c_int = bindings::LUA_GCSETPAUSE as c_int;
pub const LUA_GCSETSTEPMUL: c_int = bindings::LUA_GCSETSTEPMUL as c_int;
pub const LUA_GCISRUNNING: c_int = bindings::LUA_GCISRUNNING as c_int;
pub const LUA_GCGEN: c_int = bindings::LUA_GCGEN as c_int;
pub const LUA_GCINC: c_int = bindings::LUA_GCINC as c_int;

pub use bindings::lua_gc;


/*
** miscellaneous functions
*/

pub use bindings::{
    lua_error,

    lua_next,

    lua_concat,
    lua_len,

    lua_stringtonumber,

    lua_getallocf,
    lua_setallocf,

    lua_toclose,
    lua_closeslot,
};


/*
** {==============================================================
** some useful macros
** ===============================================================
*/

#[inline]
pub unsafe fn lua_getextraspace(L: *mut lua_State) -> *mut c_void {
    (L as *mut c_char).sub(LUA_EXTRASPACE) as *mut c_void
}

#[inline]
pub unsafe fn lua_tonumber(L: *mut lua_State, i: c_int) -> lua_Number {
    lua_tonumberx(L, i, std::ptr::null_mut())
}

#[inline]
pub unsafe fn lua_tointeger(L: *mut lua_State, i: c_int) -> lua_Integer {
    lua_tointegerx(L, i, std::ptr::null_mut())
}

#[inline]
pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
    lua_settop(L, -(n)-1)
}

#[inline]
pub unsafe fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0)
}

#[inline]
pub unsafe fn lua_register(L: *mut lua_State, n: *const c_char, f: lua_CFunction) {
    lua_pushcfunction(L, f);
    lua_setglobal(L, n)
}

#[inline]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0)
}

#[inline]
pub unsafe fn lua_isfunction(L: *mut lua_State, n: c_int) -> bool {
    lua_type(L, n) == LUA_TFUNCTION
}

#[inline]
pub unsafe fn lua_istable(L: *mut lua_State, n: c_int) -> bool {
    lua_type(L, n) == LUA_TTABLE
}

#[inline]
pub unsafe fn lua_islightuserdata(L: *mut lua_State, n: c_int) -> bool {
    lua_type(L, n) == LUA_TLIGHTUSERDATA
}

#[inline]
pub unsafe fn lua_isnil(L: *mut lua_State, n: c_int) -> bool {
    lua_type(L, n) == LUA_TNIL
}

#[inline]
pub unsafe fn lua_isboolean(L: *mut lua_State, n: c_int) -> bool {
    lua_type(L, n) == LUA_TBOOLEAN
}

#[inline]
pub unsafe fn lua_isthread(L: *mut lua_State, n: c_int) -> bool {
    lua_type(L, n) == LUA_TTHREAD
}

#[inline]
pub unsafe fn lua_isnone(L: *mut lua_State, n: c_int) -> bool {
    lua_type(L, n) == LUA_TNONE
}

#[inline]
pub unsafe fn lua_isnoneornil(L: *mut lua_State, n: c_int) -> bool {
    lua_type(L, n) <= 0
}

#[inline]
pub unsafe fn lua_pushliteral(L: *mut lua_State, s: *const c_char) -> *const c_char {
    lua_pushstring(L, s)
}

#[inline]
pub unsafe fn lua_pushglobaltable(L: *mut lua_State) -> i32 {
    lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS as lua_Integer)
}

#[inline]
pub unsafe fn lua_tostring(L: *mut lua_State, i: c_int) -> *const c_char {
    lua_tolstring(L, i, std::ptr::null_mut())
}


#[inline]
pub unsafe fn lua_insert(L: *mut lua_State, idx: c_int) {
    lua_rotate(L, idx, 1)
}

#[inline]
pub unsafe fn lua_remove(L: *mut lua_State, idx: c_int) {
    lua_rotate(L, idx, -1);
    lua_pop(L, 1)
}

#[inline]
pub unsafe fn lua_replace(L: *mut lua_State, idx: c_int) {
    lua_copy(L, -1, idx);
    lua_pop(L, 1)
}

/* }============================================================== */



/*
** {======================================================================
** Debug API
** =======================================================================
*/


/*
** Event codes
*/
pub const LUA_HOOKCALL: c_int = bindings::LUA_HOOKCALL as c_int;
pub const LUA_HOOKRET: c_int = bindings::LUA_HOOKRET as c_int;
pub const LUA_HOOKLINE: c_int = bindings::LUA_HOOKLINE as c_int;
pub const LUA_HOOKCOUNT: c_int = bindings::LUA_HOOKCOUNT as c_int;
pub const LUA_HOOKTAILCALL: c_int = bindings::LUA_HOOKTAILCALL as c_int;


/*
** Event masks
*/
pub const LUA_MASKCALL: c_int = bindings::LUA_MASKCALL as c_int;
pub const LUA_MASKRET: c_int = bindings::LUA_MASKRET as c_int;
pub const LUA_MASKLINE: c_int = bindings::LUA_MASKLINE as c_int;
pub const LUA_MASKCOUNT: c_int = bindings::LUA_MASKCOUNT as c_int;

pub use bindings::lua_Debug;


/* Functions to be called by the debugger in specific events */
pub use bindings::lua_Hook;


pub use bindings::{
    lua_getstack,
    lua_getinfo,
    lua_getlocal,
    lua_setlocal,
    lua_getupvalue,
    lua_setupvalue,

    lua_upvalueid,
    lua_upvaluejoin,

    lua_sethook,
    lua_gethook,
    lua_gethookmask,
    lua_gethookcount,

    lua_setcstacklimit,
};


/* }====================================================================== */


// lauxlib.h

/* global table */
pub use bindings::LUA_GNAME;

pub use bindings::luaL_Buffer;


/* extra error code for 'luaL_loadfilex' */
pub const LUA_ERRFILE: c_int = bindings::LUA_ERRFILE as c_int;


/* key, in the registry, for table of loaded modules */
pub const LUA_LOADED_TABLE: &[u8; 8] = b"_LOADED\0";

/* key, in the registry, for table of preloaded loaders */
pub const LUA_PRELOAD_TABLE: &[u8; 9] = b"_PRELOAD\0";


pub use bindings::luaL_Reg;

pub const LUAL_NUMSIZES: usize = std::mem::size_of::<lua_Integer>()*16 + std::mem::size_of::<lua_Number>();


pub use bindings::luaL_checkversion_;

#[inline]
pub unsafe fn luaL_checkversion(L: *mut lua_State) {
    luaL_checkversion_(L, LUA_VERSION_NUM as lua_Number, LUAL_NUMSIZES)
}


pub use bindings::{
    luaL_getmetafield,
    luaL_callmeta,
    luaL_tolstring,
    luaL_argerror,
    luaL_typeerror,
    luaL_checklstring,
    luaL_optlstring,
    luaL_checknumber,
    luaL_optnumber,

    luaL_checkinteger,
    luaL_optinteger,

    luaL_checkstack,
    luaL_checktype,
    luaL_checkany,

    luaL_newmetatable,
    luaL_setmetatable,
    luaL_testudata,
    luaL_checkudata,

    luaL_where,
    luaL_error,

    luaL_checkoption,

    luaL_fileresult,
    luaL_execresult,
};


/* predefined references */
pub const LUA_NOREF: c_int = bindings::LUA_NOREF as c_int;
pub const LUA_REFNIL: c_int = bindings::LUA_REFNIL as c_int;

pub use bindings::{
    luaL_ref,
    luaL_unref,
};

pub use bindings::luaL_loadfilex;

#[inline]
pub unsafe fn luaL_loadfile(L: *mut lua_State, f: *const c_char) -> c_int {
    luaL_loadfilex(L, f, std::ptr::null_mut())
}

pub use bindings::{
    luaL_loadbufferx,
    luaL_loadstring,

    luaL_newstate,

    luaL_len,

    luaL_addgsub,
    luaL_gsub,

    luaL_setfuncs,

    luaL_getsubtable,

    luaL_traceback,

    luaL_requiref,
};

/*
** ===============================================================
** some useful macros
** ===============================================================
*/


#[inline]
pub unsafe fn luaL_newlibtable(L: *mut lua_State, l: &[luaL_Reg]) {
    lua_createtable(L, 0, (l.len() - 1) as c_int)
}

#[inline]
pub unsafe fn luaL_newlib(L: *mut lua_State, l: &[luaL_Reg]) {
    luaL_checkversion(L);
    luaL_newlibtable(L, l);
    luaL_setfuncs(L, l.as_ptr(), 0);
}

#[inline]
pub unsafe fn luaL_argcheck(L: *mut lua_State, cond: bool, arg: c_int, extramsg: *const c_char) {
    if !cond {
        luaL_argerror(L, arg, extramsg);
        unreachable!()
    }
}

#[inline]
pub unsafe fn luaL_argexpected(L: *mut lua_State, cond: bool, arg: c_int, tname: *const c_char) {
    if !cond {
        luaL_typeerror(L, arg, tname);
        unreachable!()
    }
}

#[inline]
pub unsafe fn luaL_checkstring(L: *mut lua_State, n: c_int) -> *const [c_char] {
    let mut len = 0;
    let ptr = luaL_checklstring(L, n, &mut len);
    std::slice::from_raw_parts(ptr, len)
}

#[inline]
pub unsafe fn luaL_optstring(L: *mut lua_State, n: c_int, d: *const c_char) -> *const c_char {
    luaL_optlstring(L, n, d, core::ptr::null_mut())
}

#[inline]
pub unsafe fn luaL_typename(L: *mut lua_State, i: c_int) -> *const c_char {
    lua_typename(L, lua_type(L, i))
}

#[inline]
pub unsafe fn luaL_dofile(L: *mut lua_State, f: *const c_char) -> c_int {
    let err = luaL_loadfile(L, f);
    if err == LUA_OK {
        return lua_pcall(L, 0, LUA_MULTRET, 0);
    }
    err
}

#[inline]
pub unsafe fn luaL_dostring(L: *mut lua_State, s: *const c_char) -> c_int {
    let err = luaL_loadstring(L, s);
    if err == LUA_OK {
        return lua_pcall(L, 0, LUA_MULTRET, 0);
    }
    err
}

#[inline]
pub unsafe fn luaL_getmetatable(L: *mut lua_State, n: *const c_char) -> c_int {
    lua_getfield(L, LUA_REGISTRYINDEX, n)
}


/******************************************************************************
* Copyright (C) 1994-2022 Lua.org, PUC-Rio.
*
* Permission is hereby granted, free of charge, to any person obtaining
* a copy of this software and associated documentation files (the
* "Software"), to deal in the Software without restriction, including
* without limitation the rights to use, copy, modify, merge, publish,
* distribute, sublicense, and/or sell copies of the Software, and to
* permit persons to whom the Software is furnished to do so, subject to
* the following conditions:
*
* The above copyright notice and this permission notice shall be
* included in all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
* EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
* MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
* IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
* CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
* TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
* SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
******************************************************************************/

