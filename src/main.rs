#![feature(const_type_id)]
mod lua;

use lua::*;


unsafe extern "C" fn lua_setup(l: *mut lua_State) -> c_int {

    // stdlib
    bindings::luaL_openlibs(l);

    lua_register(l, c_str!("ud_freeze"), Some(ud_freeze));
    lua_register(l, c_str!("ud_freeze_maybe"), Some(ud_freeze_maybe));
    lua_register(l, c_str!("ud_frozen"), Some(ud_frozen));

    0
}


fn main() {
    std::panic::set_hook(Box::new(|info| {
        println!("panic: {:?}", info);
        std::process::exit(1);
    }));

    //let main = std::fs::read("src/main.lua").unwrap();
    let main = include_bytes!("main.lua");

    unsafe {
        let l = luaL_newstate();
        assert!(l != core::ptr::null_mut());

        lua_pushcfunction(l, Some(lua_setup));
        let result = lua_pcall(l, 0, 0, 0);
        // TODO: find a safe way to print the string.
        assert!(result == LUA_OK);


        // TODO: make a pcall.

        // load the code.
        let result = luaL_loadbufferx(l,
            main.as_ptr() as *const _, main.len(),
            "main\0".as_ptr() as *const _,
            "t\0".as_ptr() as *const _);
        if result != LUA_OK {
            let mut len = 0;
            // unsafe (m).
            let string = lua_tolstring(l, -1, &mut len) as *const u8;
            let string = core::slice::from_raw_parts(string, len);
            println!("{}", core::str::from_utf8_unchecked(string));
            return;
        }


        let result = lua_pcall(l, 0, 0, 0);
        if result != LUA_OK {
            let mut len = 0;
            // unsafe (m).
            let string = lua_tolstring(l, -1, &mut len) as *const u8;
            let string = core::slice::from_raw_parts(string, len);
            println!("{}", core::str::from_utf8_unchecked(string));
            return;
        }

        lua_close(l);
    }
}

