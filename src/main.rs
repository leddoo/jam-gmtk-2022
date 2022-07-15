#![feature(const_type_id)]
mod lua;

use lua::*;
use macroquad::prelude::*;


unsafe extern "C" fn lua_setup(l: *mut lua_State) -> c_int {

    // stdlib
    bindings::luaL_openlibs(l);

    lua_register(l, c_str!("ud_freeze"), Some(ud_freeze));
    lua_register(l, c_str!("ud_freeze_maybe"), Some(ud_freeze_maybe));
    lua_register(l, c_str!("ud_frozen"), Some(ud_frozen));

    mq_setup(l);

    0
}

unsafe fn mq_setup(l: *mut lua_State) {

    unsafe fn check_vec2(l: *mut lua_State, idx: c_int) -> Vec2 {
        luaL_checktype(l, idx, LUA_TTABLE);
        luaL_argcheck(l, lua_rawgeti(l, idx, 1) == LUA_TNUMBER, idx, c_str!("vec[1] is not a number"));
        luaL_argcheck(l, lua_rawgeti(l, idx, 2) == LUA_TNUMBER, idx, c_str!("vec[2] is not a number"));
        let x = lua_tonumber(l, -2) as f32;
        let y = lua_tonumber(l, -1) as f32;
        lua_pop(l, 2);
        Vec2::new(x, y)
    }

    unsafe fn check_color(l: *mut lua_State, idx: c_int) -> Color {
        luaL_checktype(l, idx, LUA_TTABLE);
        luaL_argcheck(l, lua_rawgeti(l, idx, 1) == LUA_TNUMBER, idx, c_str!("color[1] is not a number"));
        luaL_argcheck(l, lua_rawgeti(l, idx, 2) == LUA_TNUMBER, idx, c_str!("color[2] is not a number"));
        luaL_argcheck(l, lua_rawgeti(l, idx, 3) == LUA_TNUMBER, idx, c_str!("color[3] is not a number"));
        luaL_argcheck(l, lua_rawgeti(l, idx, 4) == LUA_TNUMBER, idx, c_str!("color[4] is not a number"));
        let r = lua_tonumber(l, -4) as f32;
        let g = lua_tonumber(l, -3) as f32;
        let b = lua_tonumber(l, -2) as f32;
        let a = lua_tonumber(l, -1) as f32;
        lua_pop(l, 4);
        Color::new(r, g, b, a)
    }


    unsafe extern "C" fn mq_clear_background(l: *mut lua_State) -> c_int {
        check_args_eq_n(l, 1);
        clear_background(check_color(l, 1));
        0
    }
    lua_register(l, c_str!("mq_clear_background"), Some(mq_clear_background));

    unsafe extern "C" fn mq_draw_line(l: *mut lua_State) -> c_int {
        check_args_eq_n(l, 4);
        let p0 = check_vec2(l, 1);
        let p1 = check_vec2(l, 2);
        let w  = luaL_checknumber(l, 3) as f32;
        let c  = check_color(l, 4);
        draw_line(p0.x, p0.y,  p1.x, p1.y,  w, c);
        0
    }
    lua_register(l, c_str!("mq_draw_line"), Some(mq_draw_line));

    unsafe extern "C" fn mq_draw_rect(l: *mut lua_State) -> c_int {
        check_args_eq_n(l, 3);
        let p0 = check_vec2(l, 1);
        let p1 = check_vec2(l, 2);
        let c  = check_color(l, 3);
        let min = p0.min(p1);
        let max = p0.max(p1);
        let size = max - min;
        draw_rectangle(min.x, min.y, size.x, size.y, c);
        0
    }
    lua_register(l, c_str!("mq_draw_rect"), Some(mq_draw_rect));

    unsafe extern "C" fn mq_draw_rect_wh(l: *mut lua_State) -> c_int {
        check_args_eq_n(l, 3);
        let pos  = check_vec2(l, 1);
        let size = check_vec2(l, 2);
        let c  = check_color(l, 3);
        draw_rectangle(pos.x, pos.y, size.x, size.y, c);
        0
    }
    lua_register(l, c_str!("mq_draw_rect_wh"), Some(mq_draw_rect_wh));


    unsafe extern "C" fn mq_screen_width(l: *mut lua_State) -> c_int {
        check_args_eq_n(l, 0);
        lua_pushnumber(l, screen_width() as f64);
        1
    }
    lua_register(l, c_str!("mq_screen_width"), Some(mq_screen_width));

    unsafe extern "C" fn mq_screen_height(l: *mut lua_State) -> c_int {
        check_args_eq_n(l, 0);
        lua_pushnumber(l, screen_height() as f64);
        1
    }
    lua_register(l, c_str!("mq_screen_height"), Some(mq_screen_height));

}


#[macroquad::main("gmtk-2022")]
async fn main() {
    std::panic::set_hook(Box::new(|info| {
        println!("panic: {:?}", info);
        std::process::exit(1);
    }));

    let main = std::fs::read("src/main.lua").unwrap();
    //let main = include_bytes!("main.lua");

    unsafe {
        let l = luaL_newstate();
        assert!(l != core::ptr::null_mut());

        lua_pushcfunction(l, Some(lua_setup));
        let result = lua_pcall(l, 0, 0, 0);
        // TODO: find a safe way to print the error.
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


        let result = lua_pcall(l, 0, 1, 0);
        if result != LUA_OK {
            let mut len = 0;
            // unsafe (m).
            let string = lua_tolstring(l, -1, &mut len) as *const u8;
            let string = core::slice::from_raw_parts(string, len);
            println!("{}", core::str::from_utf8_unchecked(string));
            return;
        }

        assert!(lua_gettop(l) == 1);

        lua_pushstring(l, c_str!("setup"));
        lua_rawget(l, 1);
        let result = lua_pcall(l, 0, 1, 0);
        if result != LUA_OK {
            let mut len = 0;
            // unsafe (m).
            let string = lua_tolstring(l, -1, &mut len) as *const u8;
            let string = core::slice::from_raw_parts(string, len);
            println!("{}", core::str::from_utf8_unchecked(string));
            return;
        }

        assert!(lua_gettop(l) == 2);

        lua_pushstring(l, c_str!("update"));
        lua_rawget(l, 1);

        loop {
            assert!(lua_gettop(l) == 3);
            lua_pushvalue(l, 3);
            lua_pushvalue(l, 2);
            lua_pushnumber(l, get_frame_time() as f64);

            let result = lua_pcall(l, 2, 1, 0);
            if result != LUA_OK {
                let mut len = 0;
                // unsafe (m).
                let string = lua_tolstring(l, -1, &mut len) as *const u8;
                let string = core::slice::from_raw_parts(string, len);
                println!("{}", core::str::from_utf8_unchecked(string));
                return;
            }

            if lua_toboolean(l, -1) == 0 {
                break;
            }
            lua_pop(l, 1);

            next_frame().await;
        }

        lua_close(l);
    }
}

