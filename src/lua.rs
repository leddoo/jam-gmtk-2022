#![allow(unused)]

pub use std::os::raw::{c_char, c_int};

pub use lua54_sys::*;


pub type CStr = *const c_char;

macro_rules! c_str {
    ($str: expr) => {
        concat!($str, "\0").as_ptr() as CStr
    };
}


#[inline(always)]
pub unsafe fn check_args_eq_n(l: *mut lua_State, n: c_int) {
    if lua_gettop(l) != n {
        luaL_error(l, c_str!("expecting exactly %d arguments"), n);
    }
}


pub const fn mk_user_magic_raw(m: u64) -> u64 {
    ((m ^ (m >> 48)) << 16) >> 16
}

pub const fn mk_user_magic<T: 'static>() -> u64 {
    mk_user_magic_raw(unsafe {
        core::mem::transmute(core::any::TypeId::of::<T>())
    })
}


pub trait Userdata: 'static + Sized {
    type Data;

    const NAME: CStr;
    const MAGIC: u64 = mk_user_magic::<Self>();

    const SIZE: usize = core::mem::size_of::<Self::Data>();
}


impl Userdata for () {
    type Data = ();
    const NAME: CStr = c_str!("()");
}


use std::marker::PhantomData;

pub struct UserPtr<U: Userdata> (*mut u64, PhantomData<*mut U::Data>);

impl<U: Userdata> Clone for UserPtr<U> { fn clone(&self) -> Self { *self }}
impl<U: Userdata> core::marker::Copy for UserPtr<U> {}

impl<U: Userdata> UserPtr<U> {
    /// - l[index] must be a full userdata with a valid header.
    /// - returns some if the magic value in the userdata's header is U::MAGIC.
    #[inline(always)]
    pub unsafe fn try_new_fud(l: *mut lua_State, index: c_int) -> Option<Self> {
        let data = lua_touserdata(l, index);
        assert!(data != core::ptr::null_mut());

        let result = Self(data as *mut _, PhantomData);
        if result.magic() == U::MAGIC {
            return Some(result)
        }
        None
    }

    /// - if l[index] is a full userdata, it must have a valid header.
    /// - returns none if l[index] is not a full userdata.
    /// - returns some if the magic value in the userdata's header is U::MAGIC.
    #[inline(always)]
    pub unsafe fn try_new(l: *mut lua_State, index: c_int) -> Option<Self> {
        if lua_type(l, index) == LUA_TUSERDATA {
            return Self::try_new_fud(l, index);
        }
        None
    }

    /// - same as try_new_fud, but raises an error on failure.
    #[inline(always)]
    pub unsafe fn new_or_error_fud(l: *mut lua_State, index: c_int) -> Self {
        if let Some(result) = Self::try_new_fud(l, index) {
            return result;
        }

        luaL_typeerror(l, index, U::NAME);
        unreachable!()
    }

    /// - same as try_new, but raises an error on failure.
    #[inline(always)]
    pub unsafe fn new_or_error(l: *mut lua_State, index: c_int) -> Self {
        if let Some(result) = Self::try_new(l, index) {
            return result;
        }

        luaL_typeerror(l, index, U::NAME);
        unreachable!()
    }

    /// - bypasses all safety checks. plain pointer arithmetic.
    #[inline(always)]
    pub unsafe fn to_data(self) -> *mut U::Data {
        self.0.add(1) as *mut _
    }

    #[inline(always)]
    unsafe fn magic(self) -> u64 {
        #[cfg(target_endian = "little")]
        { self.0.read() >> 16 }
    }

    #[inline(always)]
    unsafe fn get_borrow_header(self) -> *mut i8 {
        (self.0 as *mut i8).add(0)
    }

    #[inline(always)]
    unsafe fn get_freeze_header(self) -> *mut u8 {
        (self.0 as *mut u8).add(1)
    }


    #[inline(always)]
    pub unsafe fn drop(self) {
        let data = self.to_data();
        drop(data.read());
        core::ptr::write_bytes(data, u8::MAX, 1);
        self.0.write(u64::MAX);
    }

    /// - panics if userdata is borrowed mutably.
    #[inline(always)]
    pub unsafe fn freeze(self) {
        let frozen = self.get_freeze_header();
        if frozen.read() == 0 {
            let borrows = self.get_borrow_header();
            assert!(borrows.read() >= 0);
            frozen.write(1);
        }
    }

    /// - concurrent access to the same userdata is undefined behavior!
    ///   the borrow state is *not* modified (because the writes can't be optimized away)
    /// - panics if userdata is borrowed mutably.
    /// - if `f` is called, its result is guaranteed to be returned.
    #[inline(always)]
    pub unsafe fn read_with<R, F: FnOnce(&U::Data) -> R>(self, f: F) -> R {
        let borrows = self.get_borrow_header();
        assert!(borrows.read() >= 0);
        f(&*self.to_data())
    }

    /// - panics if userdata is borrowed mutably.
    #[inline(always)]
    pub unsafe fn read(self) -> U::Data where U::Data: Copy {
        self.read_with(|data| *data)
    }

    /// - concurrent access to the same userdata is undefined behavior!
    ///   the borrow state is *not* modified (because the writes can't be optimized away)
    /// - raises an error if userdata is frozen.
    /// - panics if userdata is borrowed (mutably or not).
    /// - if `f` is called, its result is guaranteed to be returned.
    #[inline(always)]
    pub unsafe fn write_with<R, F: FnOnce(&mut U::Data) -> R>(self, f: F, l: *mut lua_State, index: c_int) -> R{
        let frozen = self.get_freeze_header();
        if frozen.read() != 0 {
            luaL_argerror(l, index, c_str!("tried to modify frozen userdata"));
            unreachable!()
        }

        let borrows = self.get_borrow_header();
        assert!(borrows.read() == 0);
        f(&mut *self.to_data())
    }

    /// - raises an error if userdata is frozen.
    /// - panics if userdata is borrowed (mutably or not).
    #[inline(always)]
    pub unsafe fn write(self, v: U::Data, l: *mut lua_State, index: c_int) {
        self.write_with(|data| *data = v, l, index)
    }
}


#[inline]
pub unsafe fn ud_push_raw<U: Userdata>(l: *mut lua_State) -> *mut U::Data {
    let u = lua_newuserdatauv(l, 8 + U::SIZE, 0) as *mut u64;
    u.write(U::MAGIC << 16);

    lua_getfield(l, LUA_REGISTRYINDEX, U::NAME);
    assert!(!lua_isnil(l, -1));
    lua_setmetatable(l, -2);

    u.add(1) as *mut U::Data
}

#[inline]
pub unsafe fn ud_push<U: Userdata>(l: *mut lua_State, v: U::Data) -> c_int {
    let u = ud_push_raw::<U>(l);
    u.write(v);
    1
}

/// freezes the userdata if arg is a userdata, else raises an error.
pub unsafe extern "C" fn ud_freeze(l: *mut lua_State) -> c_int {
    check_args_eq_n(l, 1);
    if lua_type(l, 1) == LUA_TUSERDATA {
        let p: UserPtr<()> = UserPtr(lua_touserdata(l, 1) as *mut _, PhantomData);
        p.freeze();
        return 1;
    }

    luaL_typeerror(l, 1, c_str!("userdata"))
}

/// freezes the userdata if arg is a userdata.
pub unsafe extern "C" fn ud_freeze_maybe(l: *mut lua_State) -> c_int {
    check_args_eq_n(l, 1);
    if lua_type(l, 1) == LUA_TUSERDATA {
        let p: UserPtr<()> = UserPtr(lua_touserdata(l, 1) as *mut _, PhantomData);
        p.freeze();
    }
    1
}

/// returns frozen state if argument is a userdata, else returns false.
pub unsafe extern "C" fn ud_frozen(l: *mut lua_State) -> c_int {
    check_args_eq_n(l, 1);
    let frozen =
        if lua_type(l, 1) == LUA_TUSERDATA {
            let p: UserPtr<()> = UserPtr(lua_touserdata(l, 1) as *mut _, PhantomData);
            p.get_freeze_header().read() != 0
        }
        else { false };

    lua_pushboolean(l, frozen as i32);
    1
}

/// raises an error (used as the __newindex metamethod on userdata metatables).
pub unsafe extern "C" fn ud_newindex(l: *mut lua_State) -> c_int {
    luaL_error(l, c_str!("attempt to modify userdata metatable"))
}


macro_rules! def_userdata {
    ($ty: ident, $data: ty, $name: expr) => {
        struct $ty ();

        impl Userdata for $ty {
            type Data = $data;
            const NAME: CStr = c_str!($name);
        }

        impl $ty {
            #[allow(unused)]
            #[inline(always)]
            pub unsafe fn push_raw(l: *mut lua_State) -> *mut $data {
                const _ALIGN_CHECK: () = assert!(core::mem::align_of::<$data>() <= 8);
                ud_push_raw::<$ty>(l)
            }

            #[allow(unused)]
            #[inline(always)]
            pub unsafe fn push(l: *mut lua_State, v: $data) -> c_int {
                const _ALIGN_CHECK: () = assert!(core::mem::align_of::<$data>() <= 8);
                ud_push::<$ty>(l, v)
            }

            #[allow(unused)]
            #[inline(always)]
            pub unsafe fn check_fud(l: *mut lua_State, index: c_int) -> UserPtr<$ty> {
                UserPtr::new_or_error_fud(l, index)
            }

            #[allow(unused)]
            #[inline(always)]
            pub unsafe fn check(l: *mut lua_State, index: c_int) -> UserPtr<$ty> {
                UserPtr::new_or_error(l, index)
            }

            #[allow(unused)]
            #[inline(always)]
            pub unsafe fn check_self_plus_n(l: *mut lua_State, n: c_int) -> UserPtr<$ty> {
                check_args_eq_n(l, 1 + n);
                Self::check(l, 1)
            }

            #[allow(unused)]
            #[inline(always)]
            pub unsafe fn check_self_only(l: *mut lua_State) -> UserPtr<$ty> {
                Self::check_self_plus_n(l, 0)
            }
        }
    };
}

/// - registers mt as name in the registry.
/// - panics if name is already used.
/// - prevents __newindex and getmetatable.
pub unsafe fn register_metatable(l: *mut lua_State, name: CStr, mt: &[luaL_Reg]) {
    assert!(mt.last().unwrap().name == core::ptr::null());
    assert!(mt.last().unwrap().func == None);

    // create metatable.
    let was_new = luaL_newmetatable(l, name) != 0;
    assert!(was_new);
    luaL_setfuncs(l, mt.as_ptr(), 0);

    // mt.__index = mt
    lua_pushliteral(l, c_str!("__index"));
    lua_pushvalue(l, -2);
    lua_rawset(l, -3);

    // mt.__newindex = mt
    lua_pushliteral(l, c_str!("__newindex"));
    lua_pushcfunction(l, Some(ud_newindex));
    lua_rawset(l, -3);

    // mt.__metatable = false
    lua_pushliteral(l, c_str!("__metatable"));
    lua_pushboolean(l, 0);
    lua_rawset(l, -3);

    lua_pop(l, 1);
}


pub(crate) use c_str;
pub(crate) use def_userdata;

