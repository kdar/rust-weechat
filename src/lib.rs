#[macro_use]
extern crate weechat_sys;

use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;
use std::slice;

weechat_plugin!(
  name: b"double\0"; 7,
  author: b"Kevin Darlington <kevin@outroot.com>\0"; 37,
  description: b"Test plugin for WeeChat\0"; 24,
  version: b"0.1\0"; 4,
  license: b"MIT\0"; 4,
);

#[no_mangle]
pub unsafe extern "C" fn command_double_cb(pointer: *const c_void,
                                           data: *mut c_void,
                                           buffer: *mut weechat_sys::t_gui_buffer,
                                           argc: c_int,
                                           argv: *mut *mut c_char,
                                           argv_eol: *mut *mut c_char)
                                           -> c_int {
    let plugin = {
        assert!(!pointer.is_null());
        pointer as *mut weechat_sys::t_weechat_plugin
    };

    let p = *plugin;

    if argc > 1 {
        // p.command.unwrap()(plugin,
        //                    ptr::null_mut(),
        //                    "/print hey\x00".as_ptr() as *const i8);

        let argv_r = slice::from_raw_parts(argv_eol, argc as usize);
        p.command.unwrap()(plugin, ptr::null_mut(), argv_r[1]);
        p.command.unwrap()(plugin, ptr::null_mut(), argv_r[1]);
    }

    weechat_sys::WEECHAT_RC_OK as c_int
}

#[no_mangle]
pub unsafe extern "C" fn weechat_plugin_init(plugin: *mut weechat_sys::t_weechat_plugin,
                                             argc: c_int,
                                             argv: *mut *mut c_char)
                                             -> c_uint {
    let p = {
        assert!(!plugin.is_null());
        *plugin
    };


    p.hook_command.unwrap()(plugin,
                            "double\x00".as_ptr() as *const i8,
                            "Display two \
                             times a message \
                             or execute \
                             two times a \
                             command\x00"
                                .as_ptr() as *const i8,
                            "message | command\x00".as_ptr() as *const i8,
                            "message: message to display two times\ncommand: command to \
                             execute two times\x00"
                                .as_ptr() as *const i8,
                            ptr::null(),
                            Some(command_double_cb),
                            plugin as *const c_void,
                            ptr::null_mut());

    weechat_sys::WEECHAT_RC_OK
}

#[no_mangle]
pub unsafe extern "C" fn weechat_plugin_end(plugin: *mut weechat_sys::t_weechat_plugin) -> c_uint {
    weechat_sys::WEECHAT_RC_OK
}