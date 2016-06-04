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

#pragma once

#include <connorlib/serialization/toml_ffi.h>

namespace TOML
{
    class Value;
    class ValueRef;
    class Table;
    class Array;

    class TomlError : public std::exception
    {
    public:
        TomlError(const Rust::Slice<const char> &err)
            : error(err.data, err.data + err.len)
        {
        }

        const char *what() const override
        {
            return error.c_str();
        }

    private:
        std::string error;
    };

    class Value
    {
        TOML::FFI::Value *value;
        
    public:
        Value() : value(nullptr) {}

        ~Value()
        {
            if (value)
                FFI::toml_free_value(value);
        }

        Value(const Value &copy)
        {
            value = FFI::toml_clone(copy.value);
        }

        Value(Value &&move)
            : Value()
        {
            std::swap(value, move.value);
        }

        const ValueRef *operator->() const
        {
            return Ref();
        }

        ValueRef *operator->()
        {
            return Ref();
        }

        bool Valid() const
        {
            return !!value;
        }

        const ValueRef *Ref() const
        {
            return (const ValueRef *)value;
        }

        ValueRef *Ref()
        {
            return (ValueRef *)value;
        }

        static Value Parse(const std::string &str)
        {
            FFI::Value *output;
            if (!FFI::toml_parse_text({ str }, &output))
            {
                Rust::Slice<const char> error;
                FFI::toml_get_string(output, &error);
                auto ex = TomlError(error);
                FFI::toml_free_value(output);
                throw ex;
            }

            Value result;
            result.value = output;
            return result;
        }

        static Value String(const std::string &str)
        {
            auto value = FFI::toml_new_string({ str.data(), str.size() });
            if (!value)
                throw TomlError(RUST_STR("Invalid string data"));
            Value result;
            result.value = value;
            return std::move(result);
        }

        static Value Datetime(const std::string &str)
        {
            auto value = FFI::toml_new_datetime({ str.data(), str.size() });
            if (!value)
                throw TomlError(RUST_STR("Invalid string data"));
            Value result;
            result.value = value;
            return std::move(result);
        }

        static Value I64(int64_t i)
        {
            auto value = FFI::toml_new_i64(i);
            Value result;
            result.value = value;
            return std::move(result);
        }

        static Value F64(double f)
        {
            auto value = FFI::toml_new_f64(f);
            Value result;
            result.value = value;
            return std::move(result);
        }

        static Value Bool(bool b)
        {
            auto value = FFI::toml_new_bool(b);
            Value result;
            result.value = value;
            return std::move(result);
        }

        static Value Array()
        {
            Value result;
            result.value = FFI::toml_new_array();
            return std::move(result);
        }

        static Value Table()
        {
            Value result;
            result.value = FFI::toml_new_table();
            return std::move(result);
        }
    };
    
    class ValueRef
    {
        const TOML::FFI::Value *ptr() const { return (const TOML::FFI::Value *)this; }
        TOML::FFI::Value *ptr_mut() { return (TOML::FFI::Value *)this; }
        ValueRef() = default;

    public:
        ValueRef(const ValueRef &) = delete;
        ValueRef &operator=(const ValueRef &) = delete;

        bool IsString() const { Rust::Slice<const char> x; return FFI::toml_get_string(ptr(), &x); }
        bool IsI64() const { i64 x; return FFI::toml_get_i64(ptr(), &x); }
        bool IsF64() const { f64 x; return FFI::toml_get_f64(ptr(), &x); }
        bool IsDatetime() const { Rust::Slice<const char> x; return FFI::toml_get_datetime(ptr(), &x); }
        bool IsBool() const { bool x; return FFI::toml_get_bool(ptr(), &x); }
        bool IsArray() const { const FFI::Array *x; return FFI::toml_get_array(ptr(), &x); }
        bool IsTable() const { const FFI::Table *x; return FFI::toml_get_table(ptr(), &x); }

        i64 GetI64() const
        {
            i64 value;
            if (!FFI::toml_get_i64(ptr(), &value))
                throw TomlError(RUST_STR("Value was not i64"));
            return value;
        }

        const Table *GetTable() const
        {
            const FFI::Table *table;
            if (!FFI::toml_get_table(ptr(), &table))
                return nullptr;
            return (const Table *)table;
        }

        std::string Serialize() const
        {
            if (!IsTable())
                throw TomlError(RUST_STR("Can't serialize non-tables"));
            FFI::Value *output;
            FFI::toml_serialize_text(ptr(), &output);
            Rust::Slice<const char> text;
            FFI::toml_get_string(output, &text);
            std::string text_str = text;
            FFI::toml_free_value(output);
            return std::move(text_str);
        }
    };

    class Table
    {
        const TOML::FFI::Table *ptr() const { return (const TOML::FFI::Table *)this; }
        TOML::FFI::Table *ptr_mut() { return (TOML::FFI::Table *)this; }

    public:
        const ValueRef *Get(const std::string &key) const
        {
            const FFI::Value *val;
            if (!FFI::toml_table_get(ptr(), key, &val))
                return nullptr;

            return (const ValueRef *)val;
        }
    };
}

