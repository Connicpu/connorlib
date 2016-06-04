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
use toml::{self, Value, Array, Table};

// -----------------------------------------------------------------------------------------------
// Creation functions

#[no_mangle]
pub extern "C" fn toml_free_value(value: *mut Value) {
    if !value.is_null() {
        let _ = unsafe { Box::from_raw(value) };
    }
}

#[no_mangle]
pub extern "C" fn toml_new_string(data: &&[u8]) -> *mut Value {
    if let Ok(s) = str::from_utf8(*data) {
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
pub extern "C" fn toml_new_bool(data: bool) -> *mut Value {
    Box::into_raw(Box::new(Value::Boolean(data)))
}

#[no_mangle]
pub extern "C" fn toml_new_f64(data: f64) -> *mut Value {
    Box::into_raw(Box::new(Value::Float(data)))
}

#[no_mangle]
pub extern "C" fn toml_new_datetime(data: &&[u8]) -> *mut Value {
    if let Ok(s) = str::from_utf8(*data) {
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

#[no_mangle]
pub extern "C" fn toml_clone(value: *const Value) -> *mut Value {
    Box::into_raw(Box::new(unsafe { (*value).clone() }))
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
pub extern "C" fn toml_get_bool(value: *const Value, data: *mut bool) -> bool {
    match *unsafe { &*value } {
        Value::Boolean(b) => {
            unsafe { *data = b };
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
pub extern "C" fn toml_set_string(value: *mut Value, data: &&[u8]) -> bool {
    let data = match str::from_utf8(*data) {
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
pub extern "C" fn toml_set_datetime(value: *mut Value, data: &&[u8]) -> bool {
    let data = match str::from_utf8(*data) {
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

// -----------------------------------------------------------------------------------------------
// Array functions

#[no_mangle]
pub extern "C" fn toml_array_clear(array: *mut Array) {
    unsafe { (*array).clear() };
}

#[no_mangle]
pub extern "C" fn toml_array_len(array: *const Array) -> usize {
    unsafe { (*array).len() }
}

#[no_mangle]
pub extern "C" fn toml_array_get(
    array: *const Array, idx: usize, value: *mut *const Value
) -> bool {
    if let Some(val) = unsafe { (*array).get(idx) } {
        unsafe { *value = val as *const Value };
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn toml_array_get_mut(
    array: *mut Array, idx: usize, value: *mut *mut Value
) -> bool {
    if let Some(val) = unsafe { (*array).get_mut(idx) } {
        unsafe { *value = val as *mut Value };
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn toml_array_push(array: *mut Array, value: *mut Value) {
    unsafe {
        let value = Box::from_raw(value);
        (*array).push(*value);
    }
}

#[no_mangle]
pub extern "C" fn toml_array_pop(array: *mut Array) -> bool {
    unsafe { (*array).pop().is_some() }
}

#[no_mangle]
pub extern "C" fn toml_array_insert(array: *mut Array, idx: usize, value: *mut Value) -> bool {
    unsafe {
        if idx > (*array).len() {
            return false;
        }
        
        let value = Box::from_raw(value);
        (*array).insert(idx, *value);
        true
    }
}

#[no_mangle]
pub extern "C" fn toml_array_remove(array: *mut Array, idx: usize) -> bool {
    unsafe {
        if idx >= (*array).len() {
            return false;
        }
        
        (*array).remove(idx);
        true
    }
}

// -----------------------------------------------------------------------------------------------
// Table functions

#[no_mangle]
pub extern "C" fn toml_table_clear(table: *mut Table) {
    unsafe { (*table).clear() }
}

#[no_mangle]
pub extern "C" fn toml_table_len(table: *const Table) -> usize {
    unsafe { (*table).len() }
}

#[no_mangle]
pub extern "C" fn toml_table_keys(table: *const Table, key_list: &mut &mut [&str]) -> bool {
    let table = unsafe { &*table };
    
    if key_list.len() != table.len() {
        return false;
    }
    
    for (src, dst) in table.keys().zip(key_list.iter_mut()) {
        *dst = src;
    }
    
    true
}


#[no_mangle]
pub extern "C" fn toml_table_get(
    table: *const Table, key: &&[u8], value: *mut *const Value
) -> bool {
    let table = unsafe { &*table };
    let key = match str::from_utf8(key) {
        Ok(key) => key,
        Err(_) => return false,
    };
    
    if let Some(val) = table.get(key) {
        unsafe { *value = val as *const Value };
        true 
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn toml_table_get_mut(
    table: *mut Table, key: &&[u8], value: *mut *mut Value
) -> bool {
    let table = unsafe { &mut *table };
    let key = match str::from_utf8(key) {
        Ok(key) => key,
        Err(_) => return false,
    };
    
    if let Some(val) = table.get_mut(key) {
        unsafe { *value = val as *mut Value };
        true 
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn toml_table_insert(table: *mut Table, key: &&[u8], value: *mut Value) -> bool {
    let table = unsafe { &mut *table };
    let key = match str::from_utf8(key) {
        Ok(key) => key,
        Err(_) => return false,
    };
    
    let value = unsafe { Box::from_raw(value) };
    table.insert(key.into(), *value);
    true
}

#[no_mangle]
pub extern "C" fn toml_table_remove(table: *mut Table, key: &&[u8]) -> bool {
    let table = unsafe { &mut *table };
    let key = match str::from_utf8(key) {
        Ok(key) => key,
        Err(_) => return false,
    };
    
    table.remove(key).is_some()
}

// -----------------------------------------------------------------------------------------------
// Serialization functions

#[no_mangle]
pub extern "C" fn toml_parse_text(data: &&[u8], output: *mut *mut Value) -> bool {
    let data = match str::from_utf8(data) {
        Ok(data) => data,
        Err(_) => {
            let data = Box::new(Value::String("Invalid UTF-8 data".into()));
            unsafe { *output = Box::into_raw(data) };
            return false
        },
    };
    
    let mut parser = toml::Parser::new(data);
    match parser.parse() {
        Some(data) => {
            unsafe { *output = Box::into_raw(Box::new(Value::Table(data))) };
            true
        }
        None => {
            let err = &parser.errors[..];
            let data = Box::new(Value::String(format!("{:?}", err)));
            unsafe { *output = Box::into_raw(data) };
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn toml_serialize_text(data: &Value, output: *mut *mut Value) -> bool {
    if !data.as_table().is_some() {
        return false;
    }
    
    let result = toml::encode_str(data);
    unsafe { *output = Box::into_raw(Box::new(Value::String(result))) };
    true
}
