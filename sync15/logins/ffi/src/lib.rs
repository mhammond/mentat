// XXX license
//
// We take the "low road" here when returning the structs - we expose the
// items (and arrays of items) as strings, which are JSON. The rust side of
// the world gets serialization and deserialization for free and it makes
// memory management that little bit simpler.

//extern crate mentat_ffi;
extern crate serde_json;
extern crate ffi_toolkit;
extern crate mentat_sync15_logins;
extern crate chrono;

use std::os::raw::{
    c_char,
};

use ffi_toolkit::string::{
    string_to_c_char,
    c_char_to_string,
};

pub use ffi_toolkit::memory::{
    destroy_c_char,
};

use mentat_sync15_logins::{
    ServerPassword,
    FormTarget,
};

use chrono::{
    Utc,
};

#[no_mangle]
pub extern "C" fn get(id: *const c_char) -> *const c_char {
    let sp = ServerPassword {
        uuid: c_char_to_string(id).into(),
        modified: Utc::now(),
        hostname: "http://example.com".into(),
        username: Some("me".into()),
        password: "sekret".into(),
        target: FormTarget::HttpRealm("example.com".into()),
        times_used: 3,
        time_created: Utc::now(),
        time_last_used: Utc::now(),
        time_password_changed: Utc::now(),
        username_field: None,
        password_field: None,
    };
    let result = serde_json::to_string(&sp).unwrap(); // XXX - errors?
    string_to_c_char(result)
}

#[no_mangle]
pub extern "C" fn wtf_destroy_c_char(s: *mut c_char) {
    // the "pub use" above should should be enough to expose this?
    // It appears that is enough to expose it in a windows DLL, but for
    // some reason it's not expored for Android.
    // *sob* - and now that I've defined this, suddenly this *and*
    // destroy_c_char are exposed (and removing this again removes the
    // destroy_c_char)
    // Oh well, a yak for another day.
    destroy_c_char(s);
}
