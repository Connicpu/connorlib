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
        }

        static Value Parse(const char *str, size_t len)
        {
            FFI::Value *output;
            if (!FFI::toml_parse_text({ str, len }, &output))
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

        std::string GetString() const
        {
            assert(this);
            Rust::Slice<const char> slice;
            if (!FFI::toml_get_string(ptr(), &slice))
                throw TomlError(RUST_STR("Value was not String"));
            return std::string{ slice.data, slice.len };
        }

        i64 GetI64() const
        {
            assert(this);
            i64 value;
            if (!FFI::toml_get_i64(ptr(), &value))
                throw TomlError(RUST_STR("Value was not i64"));
            return value;
        }

        f64 GetF64() const
        {
            assert(this);
            f64 value;
            if (!FFI::toml_get_f64(ptr(), &value))
                throw TomlError(RUST_STR("Value was not f64"));
            return value;
        }

        std::string GetDatetime() const
        {
            assert(this);
            Rust::Slice<const char> slice;
            if (!FFI::toml_get_datetime(ptr(), &slice))
                throw TomlError(RUST_STR("Value was not Datetime"));
            return std::string{ slice.data, slice.len };
        }

        bool GetBool() const
        {
            assert(this);
            bool value;
            if (!FFI::toml_get_bool(ptr(), &value))
                throw TomlError(RUST_STR("Value was not Bool"));
            return value;
        }

        const Array &GetArray() const
        {
            assert(this);
            const FFI::Array *array;
            if (!FFI::toml_get_array(ptr(), &array))
                throw TomlError(RUST_STR("Value was not Array"));
            return *(const Array *)array;
        }

        Array &GetArray()
        {
            assert(this);
            FFI::Array *array;
            if (!FFI::toml_get_array_mut(ptr_mut(), &array))
                throw TomlError(RUST_STR("Value was not Array"));
            return *(Array *)array;
        }

        const Table &GetTable() const
        {
            assert(this);
            const FFI::Table *table;
            if (!FFI::toml_get_table(ptr(), &table))
                throw TomlError(RUST_STR("Value was not Table"));
            return *(const Table *)table;
        }

        Table &GetTable()
        {
            assert(this);
            FFI::Table *table;
            if (!FFI::toml_get_table_mut(ptr_mut(), &table))
                throw TomlError(RUST_STR("Value was not Table"));
            return *(Table *)table;
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
        class iterator
        {
        public:
            using value_type = std::pair<std::string, const ValueRef *>;
            using reference_type = const value_type &;
            using pointer_type = const value_type *;

            iterator() = default;

            ~iterator()
            {
                if (iter)
                    FFI::toml_table_iterator_free(iter);
            }

            iterator(const iterator &) = delete;
            iterator(iterator &&move)
                : iter(move.iter)
            {
                move.iter = nullptr;
            }

            iterator(const Table *table)
            {
                iter = FFI::toml_table_get_iterator((const FFI::Table *)table);
                ++*this;
            }

            iterator &operator=(const iterator &) = delete;
            iterator &operator=(iterator &&move)
            {
                iter = move.iter;
                move.iter = nullptr;
                return *this;
            }

            iterator &operator++()
            {
                assert(iter);

                Rust::Slice<const char> key;
                const FFI::Value *value;
                if (FFI::toml_table_iterator_next(iter, &key, &value))
                {
                    current.first.assign(key.data, key.len);
                    current.second = (const ValueRef *)value;
                }
                else
                {
                    iter = nullptr;
                }

                return *this;
            }

            reference_type operator*() const
            {
                return current;
            }

            pointer_type operator->() const
            {
                return &current;
            }

            bool operator!=(const iterator &rhs) const
            {
                return iter != rhs.iter;
            }

            bool operator==(const iterator &rhs) const
            {
                return iter == rhs.iter;
            }

        private:
            FFI::TableIterator *iter;
            value_type current;
        };

        const ValueRef *Get(const char *key, size_t len) const
        {
            const FFI::Value *val;
            if (!FFI::toml_table_get(ptr(), { key, len }, &val))
                return nullptr;

            return (const ValueRef *)val;
        }
        ValueRef *Get(const char *key, size_t len)
        {
            FFI::Value *val;
            if (!FFI::toml_table_get_mut(ptr_mut(), { key, len }, &val))
                return nullptr;

            return (ValueRef *)val;
        }

        const ValueRef *Get(const std::string &key) const
        {
            return Get(key.c_str(), key.size());
        }
        ValueRef *Get(const std::string &key)
        {
            return Get(key.c_str(), key.size());
        }

        template <size_t len>
        const ValueRef *Get(const char(&key)[len]) const
        {
            return Get(key, len - 1);
        }
        template <size_t len>
        ValueRef *Get(const char(&key)[len])
        {
            return Get(key, len - 1);
        }

        template <typename Arg>
        const ValueRef *operator[](Arg &&arg) const
        {
            return Get(std::forward<Arg>(arg));
        }

        template <typename Arg>
        ValueRef *operator[](Arg &&arg)
        {
            return Get(std::forward<Arg>(arg));
        }

        inline iterator begin() const
        {
            return iterator{ this };
        }

        inline iterator end() const
        {
            return iterator{};
        }
    };

    class Array
    {
        const TOML::FFI::Array *ptr() const { return (const TOML::FFI::Array *)this; }
        TOML::FFI::Array *ptr_mut() { return (TOML::FFI::Array *)this; }

    public:
        class const_iterator
        {
        public:
            using self = const_iterator;
            using owner = const Array *;
            using owner_ffi = const FFI::Array *;
            using value_type = ValueRef;
            using reference_type = const value_type &;
            using pointer_type = const value_type *;
            using pointer_ffi = const FFI::Value *;

            const_iterator(owner array, usize index)
                : array(array), index(index)
            {
            }

            self &operator++()
            {
                ++index;
                return *this;
            }

            bool operator==(const self &rhs) const
            {
                return array == rhs.array && index == rhs.index;
            }

            bool operator!=(const self &rhs) const
            {
                return !(*this == rhs);
            }

            reference_type operator*() const
            {
                return *(operator->());
            }

            pointer_type operator->() const
            {
                pointer_ffi value;
                FFI::toml_array_get((owner_ffi)array, index, &value);
                return (pointer_type)value;
            }

        private:
            owner array;
            usize index;
        };

        class iterator
        {
        public:
            using self = iterator;
            using owner = Array *;
            using owner_ffi = FFI::Array *;
            using value_type = ValueRef;
            using reference_type = value_type &;
            using pointer_type = value_type *;
            using pointer_ffi = FFI::Value *;

            iterator(owner array, usize index)
                : array(array), index(index)
            {
            }

            self &operator++()
            {
                ++index;
                return *this;
            }

            bool operator==(const self &rhs) const
            {
                return array == rhs.array && index == rhs.index;
            }

            bool operator!=(const self &rhs) const
            {
                return !(*this == rhs);
            }

            reference_type operator*() const
            {
                return *(operator->());
            }

            pointer_type operator->() const
            {
                pointer_ffi value;
                FFI::toml_array_get_mut((owner_ffi)array, index, &value);
                return (pointer_type)value;
            }

        private:
            owner array;
            usize index;
        };

        size_t GetLength() const
        {
            return FFI::toml_array_len(ptr());
        }

        const ValueRef *Get(size_t index) const
        {
            const FFI::Value *value;
            if (!FFI::toml_array_get(ptr(), index, &value))
                return nullptr;
            return (const ValueRef *)value;
        }
        ValueRef *Get(size_t index)
        {
            const FFI::Value *value;
            if (!FFI::toml_array_get(ptr(), index, &value))
                return nullptr;
            return (ValueRef *)value;
        }

        const ValueRef *operator[](size_t index) const
        {
            return Get(index);
        }
        ValueRef *operator[](size_t index)
        {
            return Get(index);
        }

        const_iterator begin() const
        {
            return const_iterator{ this, 0 };
        }

        const_iterator end() const
        {
            return const_iterator{ this, GetLength() };
        }

        iterator begin()
        {
            return iterator{ this, 0 };
        }

        iterator end()
        {
            return iterator{ this, GetLength() };
        }
    };
}

