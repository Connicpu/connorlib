// The MIT License (MIT) 
// Copyright (c) 2016 Connor Hilarides
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::str;
use rustc_serialize::json::Json as Value;

#[no_mangle]
pub extern "C" fn json_free_value(value: *mut Value) {
    if !value.is_null() {
        let _ = unsafe { Box::from_raw(value) };
    }
}

#[no_mangle]
pub extern "C" fn json_clone(value: *const Value) -> *mut Value {
    Box::into_raw(Box::new(unsafe { (*value).clone() }))
}

// -----------------------------------------------------------------------------------------------
// Getter functions

#[no_mangle]
pub extern "C" fn json_get_string(value: *const Value, data: *mut &[u8]) -> bool {
    match *unsafe { &*value } {
        Value::String(ref s) => {
            unsafe { *data = &*(s.as_bytes() as *const [u8]) };
            true
        }
        _ => false
    }
}

// -----------------------------------------------------------------------------------------------
// Serialization functions

#[no_mangle]
pub extern "C" fn json_parse_text(data: &&[u8], output: *mut *mut Value) -> bool {
    let data = match str::from_utf8(data) {
        Ok(data) => data,
        Err(_) => {
            let data = Box::new(Value::String("Invalid UTF-8 data".into()));
            unsafe { *output = Box::into_raw(data) };
            return false
        },
    };
    
    match Value::from_str(data) {
        Ok(data) => {
            unsafe { *output = Box::into_raw(Box::new(data)) };
            true
        }
        Err(err) => {
            let data = Box::new(Value::String(format!("{}", err)));
            unsafe { *output = Box::into_raw(data) };
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn json_serialize_text(data: &Value, output: *mut *mut Value) -> bool {
    let result = data.to_string();
    unsafe { *output = Box::into_raw(Box::new(Value::String(result))) };
    true
}

#[no_mangle]
pub extern "C" fn json_serialize_text_pretty(data: &Value, output: *mut *mut Value) -> bool {
    let result = format!("{}", data.pretty());
    unsafe { *output = Box::into_raw(Box::new(Value::String(result))) };
    true
}

