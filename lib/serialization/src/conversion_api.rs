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

use std::cmp::min;
use std::i64;
use toml;
use toml::Value as Toml;
use rustc_serialize::json::Json;

fn imp_toml_to_json(toml: &Toml) -> Json {
    match *toml {
        Toml::String(ref s) => Json::String(s.clone()),
        Toml::Integer(i) => Json::I64(i),
        Toml::Float(f) => Json::F64(f),
        Toml::Boolean(b) => Json::Boolean(b),
        Toml::Datetime(ref d) => Json::String(d.clone()),
        Toml::Array(ref a) => Json::Array(
            a.iter().map(|v| imp_toml_to_json(v)).collect()
        ),
        Toml::Table(ref t) => Json::Object(
            t.iter().map(|(k, v)| (k.clone(), imp_toml_to_json(v))).collect()
        ),
    }
}

fn imp_json_to_toml(json: &Json) -> Option<Toml> {
    Some(match *json {
        Json::String(ref s) => Toml::String(s.clone()),
        Json::I64(i) => Toml::Integer(i),
        Json::U64(u) => Toml::Integer(min(u, i64::MAX as u64) as i64),
        Json::F64(f) => Toml::Float(f),
        Json::Boolean(b) => Toml::Boolean(b),
        Json::Array(ref a) => Toml::Array(
            a.iter().filter_map(|v| imp_json_to_toml(v)).collect()
        ),
        Json::Object(ref o) => Toml::Table(
            o.iter().filter_map(|(k, v)| match imp_json_to_toml(v) {
                Some(v) => Some((k.clone(), v)),
                None => None, 
            }).collect()
        ),
        Json::Null => return None,
    })
}

#[no_mangle]
pub extern "C" fn toml_to_json(input: *const Toml, output: *mut *mut Json) {
    unsafe {
        let json = imp_toml_to_json(&*input);
        *output = Box::into_raw(Box::new(json));
    }
}

#[no_mangle]
pub extern "C" fn json_to_toml(input: *const Json, output: *mut *mut Toml) {
    unsafe {
        let toml = match imp_json_to_toml(&*input) {
            Some(toml) => toml,
            None => Toml::Table(toml::Table::new()),
        };
        
        *output = Box::into_raw(Box::new(toml));
    }
}


