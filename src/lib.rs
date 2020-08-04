// TODO: diff generating
// TODO: diff from 2 streams (maybe mix and match)
// TODO: proper nodejs exports
// TODO: document

#[macro_use]
mod util;
mod dmi;

use nodejs_sys::{napi_env, napi_value, napi_set_named_property, napi_create_function, napi_callback_info, napi_get_boolean, napi_get_cb_info};
use std::{ffi::CString, ptr};
use util::{jsargs_str};
use dmi::{Image};

pub unsafe extern "C" fn test(env: napi_env, info: napi_callback_info) -> napi_value {
    let args = jsargs!(env, info, 1);
    let path = jsargs_str(env, args[0]);
    let img: Image = Image::from_file(std::path::Path::new(&path));

    jsboolean!(env, true)
}

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(env: napi_env, exports: napi_value) -> nodejs_sys::napi_value {
    export_fn!(env, "test", test, exports);

    exports
}