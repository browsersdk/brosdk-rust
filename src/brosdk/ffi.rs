//! Raw C FFI bindings for brosdk.
//! Matches brosdk.h — loaded at runtime via libloading.

use std::ffi::{c_char, c_int, c_void};

pub type SdkHandleT = *mut c_void;
pub type SdkResultCb =
    unsafe extern "C" fn(code: i32, user_data: *mut c_void, data: *const c_char, len: usize);
pub type SdkCookiesStorageCb = unsafe extern "C" fn(
    data: *const c_char,
    len: usize,
    new_data: *mut *mut c_char,
    new_len: *mut usize,
    user_data: *mut c_void,
);

/// Symbols loaded from the brosdk dynamic library.
pub struct BrosdkLib {
    // --- Registration ---
    pub sdk_register_result_cb:
        libloading::Symbol<'static, unsafe extern "C" fn(SdkResultCb, *mut c_void) -> i32>,
    pub sdk_register_cookies_storage_cb:
        libloading::Symbol<'static, unsafe extern "C" fn(SdkCookiesStorageCb, *mut c_void) -> i32>,
    // --- Core lifecycle ---
    pub sdk_init: libloading::Symbol<
        'static,
        unsafe extern "C" fn(
            *mut SdkHandleT,
            *const c_char,
            usize,
            *mut *mut c_char,
            *mut usize,
        ) -> i32,
    >,
    /// Query SDK runtime info (version, state, etc.).
    /// Signature: int32_t sdk_info(char **out_data, size_t *out_len)
    /// Returns an SDK-allocated JSON string; caller must free via sdk_free.
    pub sdk_info: libloading::Symbol<
        'static,
        unsafe extern "C" fn(*mut *mut c_char, *mut usize) -> i32,
    >,
    pub sdk_shutdown: libloading::Symbol<'static, unsafe extern "C" fn() -> i32>,
    // --- Browser ---
    pub sdk_browser_open:
        libloading::Symbol<'static, unsafe extern "C" fn(*const c_char, usize) -> i32>,
    pub sdk_browser_close:
        libloading::Symbol<'static, unsafe extern "C" fn(*const c_char, usize) -> i32>,
    // --- Auth ---
    pub sdk_token_update:
        libloading::Symbol<'static, unsafe extern "C" fn(*const c_char, usize) -> i32>,
    // --- Memory management ---
    pub sdk_free: libloading::Symbol<'static, unsafe extern "C" fn(*mut c_void)>,
    pub sdk_malloc: libloading::Symbol<'static, unsafe extern "C" fn(usize) -> *mut c_void>,
    // --- Code classification ---
    pub sdk_is_ok: libloading::Symbol<'static, unsafe extern "C" fn(i32) -> bool>,
    pub sdk_is_done: libloading::Symbol<'static, unsafe extern "C" fn(i32) -> bool>,
    pub sdk_is_reqid: libloading::Symbol<'static, unsafe extern "C" fn(i32) -> bool>,
    pub sdk_is_error: libloading::Symbol<'static, unsafe extern "C" fn(i32) -> bool>,
    pub sdk_is_warn: libloading::Symbol<'static, unsafe extern "C" fn(i32) -> bool>,
    pub sdk_is_event: libloading::Symbol<'static, unsafe extern "C" fn(i32) -> bool>,
    // --- Code description ---
    pub sdk_error_string:
        libloading::Symbol<'static, unsafe extern "C" fn(c_int) -> *const c_char>,
    // Keep library alive
    _lib: &'static libloading::Library,
}

impl BrosdkLib {
    /// Load brosdk from the given path. The Library is leaked to 'static so symbols stay valid.
    pub unsafe fn load(path: &str) -> Result<Self, String> {
        let lib = Box::new(
            libloading::Library::new(path).map_err(|e| format!("failed to load brosdk: {e}"))?,
        );
        let lib: &'static libloading::Library = Box::leak(lib);

        macro_rules! sym {
            ($name:expr) => {
                lib.get($name)
                    .map_err(|e| format!("symbol {} not found: {e}", stringify!($name)))?
            };
        }

        Ok(Self {
            sdk_register_result_cb: sym!(b"sdk_register_result_cb"),
            sdk_register_cookies_storage_cb: sym!(b"sdk_register_cookies_storage_cb"),
            sdk_init: sym!(b"sdk_init"),
            sdk_info: sym!(b"sdk_info"),
            sdk_shutdown: sym!(b"sdk_shutdown"),
            sdk_browser_open: sym!(b"sdk_browser_open"),
            sdk_browser_close: sym!(b"sdk_browser_close"),
            sdk_token_update: sym!(b"sdk_token_update"),
            sdk_free: sym!(b"sdk_free"),
            sdk_malloc: sym!(b"sdk_malloc"),
            sdk_is_ok: sym!(b"sdk_is_ok"),
            sdk_is_done: sym!(b"sdk_is_done"),
            sdk_is_reqid: sym!(b"sdk_is_reqid"),
            sdk_is_error: sym!(b"sdk_is_error"),
            sdk_is_warn: sym!(b"sdk_is_warn"),
            sdk_is_event: sym!(b"sdk_is_event"),
            sdk_error_string: sym!(b"sdk_error_string"),
            _lib: lib,
        })
    }

    /// Decode an SDK-allocated `char*` buffer into a `String` and free it.
    pub unsafe fn take_string(&self, ptr: *mut c_char, len: usize) -> String {
        if ptr.is_null() || len == 0 {
            return String::new();
        }
        let bytes = std::slice::from_raw_parts(ptr as *const u8, len);
        let s = String::from_utf8_lossy(bytes).into_owned();
        (self.sdk_free)(ptr as *mut c_void);
        s
    }
}
