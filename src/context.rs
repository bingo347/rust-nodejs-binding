use crate::result::*;
use crate::sys::{self, napi_env, napi_status, napi_value};

#[derive(Clone, Copy, Debug)]
pub struct Context {
    env: napi_env,
}

impl From<napi_env> for Context {
    fn from(env: napi_env) -> Self {
        Self { env }
    }
}

impl Context {
    pub fn get_env(&self) -> napi_env {
        self.env
    }

    pub fn handle_status(&self, status: napi_status) -> NodeJSResult<()> {
        if status == napi_status::napi_ok {
            Ok(())
        } else {
            Err(NodeJSError {
                kind: NodeJSErrorKind::from_napi_status(status),
                message: get_error_message(self.env),
                exception: get_pending_exception_for_status(self.env, status),
            })
        }
    }
}

fn get_error_message(env: napi_env) -> Option<String> {
    use std::{ffi::CStr, ptr};
    let mut extended_error_info = ptr::null();
    let raw_error_message = unsafe {
        sys::napi_get_last_error_info(env, &mut extended_error_info);
        (*extended_error_info).error_message
    };
    if raw_error_message.is_null() {
        None
    } else {
        let c_string = unsafe { CStr::from_ptr(raw_error_message) };
        Some(c_string.to_string_lossy().into_owned())
    }
}

fn get_pending_exception_for_status(env: napi_env, status: napi_status) -> Option<napi_value> {
    let mut is_exception_pending = true;
    if status != napi_status::napi_pending_exception {
        unsafe {
            sys::napi_is_exception_pending(env, &mut is_exception_pending);
        }
    }
    if !is_exception_pending {
        return None;
    }
    let mut exception = std::ptr::null_mut();
    unsafe {
        sys::napi_get_and_clear_last_exception(env, &mut exception);
    }
    if exception.is_null() {
        None
    } else {
        Some(exception)
    }
}
