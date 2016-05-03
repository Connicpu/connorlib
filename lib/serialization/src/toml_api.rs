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

// -----------------------------------------------------------------------------------------------
// Creation functions

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

// -----------------------------------------------------------------------------------------------
// Getter functions

#[no_mangle]
pub extern "C" fn toml_get_string(value: *const Value, data: *mut &[u8]) -> bool {
    match *unsafe { &*value } {
        Value::String(ref s) => {
            unsafe { *data = &*(s.as_bytes() as *const [u8]) };
            true
        }
        _ => false
    }
}

#[no_mangle]
pub extern "C" fn toml_get_i64(value: *const Value, data: *mut i64) -> bool {
    match *unsafe { &*value } {
        Value::Integer(i) => {
            unsafe { *data = i };
            true
        }
        _ => false
    }
 }

#[no_mangle]
pub extern "C" fn toml_get_f64(value: *const Value, data: *mut f64) -> bool {
    match *unsafe { &*value } {
        Value::Float(f) => {
            unsafe { *data = f };
            true
        }
        _ => false
    }
 }

#[no_mangle]
pub extern "C" fn toml_get_datetime(value: *const Value, data: *mut &[u8]) -> bool {
    match *unsafe { &*value } {
        Value::Datetime(ref s) => {
            unsafe { *data = &*(s.as_bytes() as *const [u8]) };
            true
        }
        _ => false
    }
}

#[no_mangle]
pub extern "C" fn toml_get_array(value: *const Value, data: *mut *const Array) -> bool {
    match *unsafe { &*value } {
        Value::Array(ref a) => {
            unsafe { *data = a };
            true
        }
        _ => false
    }
}

#[no_mangle]
pub extern "C" fn toml_get_array_mut(value: *mut Value, data: *mut *mut Array) -> bool {
    match *unsafe { &mut *value } {
        Value::Array(ref mut a) => {
            unsafe { *data = a };
            true
        }
        _ => false
    }
}

#[no_mangle]
pub extern "C" fn toml_get_table(value: *const Value, data: *mut *const Table) -> bool {
    match *unsafe { &*value } {
        Value::Table(ref t) => {
            unsafe { *data = t };
            true
        }
        _ => false
    }
}

#[no_mangle]
pub extern "C" fn toml_get_table_mut(value: *mut Value, data: *mut *mut Table) -> bool {
    match *unsafe { &mut *value } {
        Value::Table(ref mut t) => {
            unsafe { *data = t };
            true
        }
        _ => false
    }
}

#[no_mangle]
pub extern "C" fn toml_set_string(value: *mut Value, data: &[u8]) -> bool {
    let data = match str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    let value = unsafe { &mut *value };
    if let Value::String(ref mut s) = *value {
        s.clear();
        s.push_str(data);
    } else {
        *value = Value::String(data.into());
    }
    
    true
}

#[no_mangle]
pub extern "C" fn toml_set_i64(value: *mut Value, data: i64) {
    unsafe { *value = Value::Integer(data) };
}

#[no_mangle]
pub extern "C" fn toml_set_f64(value: *mut Value, data: f64) {
    unsafe { *value = Value::Float(data) };
}

#[no_mangle]
pub extern "C" fn toml_set_datetime(value: *mut Value, data: &[u8]) -> bool {
    let data = match str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    let value = unsafe { &mut *value };
    if let Value::Datetime(ref mut s) = *value {
        s.clear();
        s.push_str(data);
    } else {
        *value = Value::Datetime(data.into());
    }
    
    true
}

#[no_mangle]
pub extern "C" fn toml_set_bool(value: *mut Value, data: bool) {
    unsafe { *value = Value::Boolean(data) };
}

#[no_mangle]
pub extern "C" fn toml_set_array(value: *mut Value) {
    unsafe { *value = Value::Array(Array::new()) };
}

#[no_mangle]
pub extern "C" fn toml_set_table(value: *mut Value) {
    unsafe { *value = Value::Table(Table::new()) };
}

