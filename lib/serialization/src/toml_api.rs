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

use std::{ptr, str};
use toml::{Value, Array, Table};

#[no_mangle]
pub extern "C" fn toml_free_value(value: *mut Value) {
    if !value.is_null() {
        let _ = unsafe { Box::from_raw(value) };
    }
}

#[no_mangle]
pub extern "C" fn toml_new_string(data: &[u8]) -> *mut Value {
    if let Ok(s) = str::from_utf8(data) {
        Box::into_raw(Box::new(Value::String(s.into())))
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn toml_new_i64(data: i64) -> *mut Value {
    Box::into_raw(Box::new(Value::Integer(data)))
}

#[no_mangle]
pub extern "C" fn toml_new_f64(data: f64) -> *mut Value {
    Box::into_raw(Box::new(Value::Float(data)))
}

#[no_mangle]
pub extern "C" fn toml_new_datetime(data: &[u8]) -> *mut Value {
    if let Ok(s) = str::from_utf8(data) {
        Box::into_raw(Box::new(Value::Datetime(s.into())))
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn toml_new_array() -> *mut Value {
    Box::into_raw(Box::new(Value::Array(Array::new())))
}

#[no_mangle]
pub extern "C" fn toml_new_table() -> *mut Value {
    Box::into_raw(Box::new(Value::Table(Table::new())))
}

