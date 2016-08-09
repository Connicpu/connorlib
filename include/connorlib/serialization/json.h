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

#include <connorlib/serialization/json_ffi.h>

namespace TOML { class Value; }
namespace JSON
{
    class Value;
    class ValueRef;

    class JsonError : public std::exception
    {
    public:
        JsonError(const Rust::Slice<const char> &err)
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
        JSON::FFI::Value *value;
        
    public:
        Value() : value(nullptr) {}

        ~Value()
        {
            if (value)
                FFI::json_free_value(value);
        }

        Value(const Value &copy)
        {
            value = FFI::json_clone(copy.value);
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
            return Parse(str.c_str(), str.size());
        }

        static Value Parse(const char *str, size_t len)
        {
            FFI::Value *output;
            if (!FFI::json_parse_text({ str, len }, &output))
            {
                Rust::Slice<const char> error;
                FFI::json_get_string(output, &error);
                auto ex = JsonError(error);
                FFI::json_free_value(output);
                throw ex;
            }

            Value result;
            result.value = output;
            return result;
        }
    };
    
    class ValueRef
    {
        const JSON::FFI::Value *ptr() const { return (const JSON::FFI::Value *)this; }
        JSON::FFI::Value *ptr_mut() { return (JSON::FFI::Value *)this; }
        ValueRef() = default;

    public:
        ValueRef(const ValueRef &) = delete;
        ValueRef &operator=(const ValueRef &) = delete;

        std::string Serialize() const
        {
            FFI::Value *output;
            FFI::json_serialize_text(ptr(), &output);
            Rust::Slice<const char> text;
            FFI::json_get_string(output, &text);
            std::string text_str = text;
            FFI::json_free_value(output);
            return std::move(text_str);
        }

        inline TOML::Value ToToml() const;
    };
}
