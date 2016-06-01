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

#include <cassert>
#include <cstdint>
#include <string>

using i8 = std::int8_t;
using u8 = std::uint8_t;
using i16 = std::int16_t;
using u16 = std::uint16_t;
using i32 = std::int32_t;
using u32 = std::uint32_t;
using i64 = std::int64_t;
using u64 = std::uint64_t;
using usize = size_t;
using f32 = float;
using f64 = double;

#define RUST_STR(x) (Rust::Slice<const char>{ "" x, sizeof("" x)-1 })

namespace Rust
{
    template <typename T>
    struct Slice
    {
        using string_type = std::basic_string<std::remove_const_t<T>>;
        
        inline Slice()
            : data(nullptr), len(0)
        {
        }
        
        inline Slice(T *data, size_t len)
            : data(data), len(len)
        {
            assert(data != nullptr || len == 0);
        }
        
        inline Slice(const string_type &str)
            : data(str.c_str()), len(str.size())
        {
        }
        
        operator string_type() const
        {
            return { data, data + len };
        }
        
        T *data;
        usize len;
    };
}
