use crate::sys::{
    // napi_create_error, napi_create_range_error, napi_create_type_error,
    napi_status,
    napi_value,
};

macro_rules! mk_error_kind {
    ($($t:ident => $s:ident),+) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum NodeJSErrorKind {
            $($t),+
        }

        impl NodeJSErrorKind {
            #[allow(unreachable_patterns)]
            #[inline]
            pub fn from_napi_status(status: napi_status) -> Self {
                match status {
                    napi_status::napi_ok => unreachable!(),
                    $(napi_status::$s => NodeJSErrorKind::$t),+,
                    _ => unimplemented!(),
                }
            }
        }
    }
}

mk_error_kind! {
    InvalidArg => napi_invalid_arg,
    ObjectExpected => napi_object_expected,
    StringExpected => napi_string_expected,
    NameExpected => napi_name_expected,
    FunctionExpected => napi_function_expected,
    NumberExpected => napi_number_expected,
    BooleanExpected => napi_boolean_expected,
    ArrayExpected => napi_array_expected,
    GenericFailure => napi_generic_failure,
    PendingException => napi_pending_exception,
    Cancelled => napi_cancelled,
    EscapeCalledTwice => napi_escape_called_twice,
    HandleScopeMismatch => napi_handle_scope_mismatch,
    CallbackScopeMismatch => napi_callback_scope_mismatch,
    QueueFull => napi_queue_full,
    Closing => napi_closing,
    BigintExpected => napi_bigint_expected
}

#[derive(Clone, Debug)]
pub struct NodeJSError {
    pub kind: NodeJSErrorKind,
    pub message: Option<String>,
    pub exception: Option<napi_value>,
}

pub type NodeJSResult<T> = Result<T, NodeJSError>;
