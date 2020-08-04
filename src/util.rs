use nodejs_sys::{napi_env, napi_value, napi_get_value_string_utf8};
use std::{ptr};

#[macro_export]
macro_rules! cstring {
    ($str:expr) => {
        cstring!($str, "CString::new failed")
    };
    ($str:expr, $err:expr) => {
        CString::new($str).expect("CString::new failed")
    };
}

#[macro_export]
macro_rules! jsstring {
    ($env:expr, $str:expr) => {{
        let mut local: napi_value = std::mem::zeroed();
        let p = cstring!($str);
        napi_create_string_utf8($env, p.as_ptr(), $str.len() as u64, &mut local);
    
        local
    }};
}

#[macro_export]
macro_rules! jsboolean {
    ($env:expr, $value:expr) => {{
        let mut local: napi_value = std::mem::zeroed();
        napi_get_boolean($env, $value, &mut local);

        local
    }};
}

#[macro_export]
macro_rules! jsargs {
    ($env:expr, $info:expr, $num_args:expr) => {{
        let mut buffer: [napi_value; $num_args] = std::mem::MaybeUninit::zeroed().assume_init();
        napi_get_cb_info($env, $info, &mut $num_args, buffer.as_mut_ptr(), ptr::null_mut(), ptr::null_mut());

        buffer
    }};
}

#[macro_export]
macro_rules! export_fn {
    ($env:expr, $name:expr, $func:expr, $exports:expr) => {
        let mut fn_loc: napi_value = std::mem::zeroed();
        let fn_name = cstring!($name);
        napi_create_function($env, fn_name.as_ptr(), $name.len() as u64, Some($func), ptr::null_mut(), &mut fn_loc);
        napi_set_named_property($env, $exports, fn_name.as_ptr(), fn_loc);
    };
}

pub unsafe extern "C" fn jsargs_str<'a>(env: napi_env, arg: napi_value) -> String {
    let mut len: u64 = 0;
    napi_get_value_string_utf8(env, arg, ptr::null_mut(), 0, &mut len);
    if len > 0 {
        let mut ve: Vec<u8> = Vec::with_capacity((len + 1) as usize);
        let raw = ve.as_mut_ptr();
        std::mem::forget(ve);
        let mut cap = 0;
        napi_get_value_string_utf8(env, arg, raw as *mut i8, len + 1, &mut cap);

        String::from_raw_parts(raw, cap as usize, len as usize)
    } else {
        String::new()
    }
}