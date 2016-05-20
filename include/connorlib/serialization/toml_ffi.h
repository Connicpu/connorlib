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

#include <connorlib/dll.h>
#include <connorlib/rustop.h>

namespace TOML
{
namespace FFI
{
    struct Value;
    struct Array;
    struct Table;

    /// Free a value that you have ownership of
    extern "C" DLL_IMPORT void toml_free_value(Value *value);
    
    /// Create a new Value containing a string, you have ownership of the returned value.
    /// This function will fail and return nullptr if the data is not valid UTF-8.
    extern "C" DLL_IMPORT Value * toml_new_string(const Rust::Slice<const char> &data);
    /// Create a new Value containing an i64, you have ownership of the returned value.
    extern "C" DLL_IMPORT Value * toml_new_i64(int64_t data);
    /// Create a new Value containing an f64, you have ownership of the returned value.
    extern "C" DLL_IMPORT Value * toml_new_f64(double data);
    /// Create a new Value containing a datetime, you have ownership of the returned value.
    /// This function will fail and return nullptr if the data is not valid UTF-8.
    extern "C" DLL_IMPORT Value * toml_new_datetime(const Rust::Slice<const char> &data);
    /// Create a new Value containing an array, you have ownership of the returned value.
    extern "C" DLL_IMPORT Value * toml_new_array();
    /// Create a new Value containing a table, you have ownership of the returned value.
    extern "C" DLL_IMPORT Value * toml_new_table();
    /// Make a copy of a value so you can insert it into another structure
    extern "C" DLL_IMPORT Value * toml_clone(const Value *value);
    
    /// Get a slice containing the UTF-8 data of the string.
    /// Returns false if the value is not a string.
    extern "C" DLL_IMPORT bool toml_get_string(const Value *value, Rust::Slice<const char> *data);
    /// Get the 64-bit integer stored in the value
    /// Returns false if the value is not an i64.
    extern "C" DLL_IMPORT bool toml_get_i64(const Value *value, i64 *data);
    /// Get the 64-bit float stored in the value.
    /// Returns false if the value is not an f64.
    extern "C" DLL_IMPORT bool toml_get_f64(const Value *value, f64 *data);
    /// Get a slice containing the UTF-8 data of the datetime.
    /// Returns false if the value is not a datetime.
    extern "C" DLL_IMPORT bool toml_get_datetime(const Value *value, Rust::Slice<const char> *data);
    extern "C" DLL_IMPORT bool toml_get_bool(const Value *value, bool *data);
    
    /// Get an immutable view of the array contained in this value.
    /// Returns false if the value is not an array.
    extern "C" DLL_IMPORT bool toml_get_array(const Value *value, const Array **data);
    /// Get a mutable view of the array contained in this value.
    /// Returns false if the value is not an array.
    extern "C" DLL_IMPORT bool toml_get_array_mut(Value *value, Array **data);
    /// Get an immutable view of the table contained in this value.
    /// Returns false if the value is not a table.
    extern "C" DLL_IMPORT bool toml_get_table(const Value *value, const Table **data);
    /// Get a mutable view of the table contained in this value.
    /// Returns false if the value is not a table.
    extern "C" DLL_IMPORT bool toml_get_table_mut(Value *value, Table **data);
    
    /// Sets the value to be a String containing the data.
    /// Returns false and makes no changes to the value if the data is not valid UTF-8.
    extern "C" DLL_IMPORT bool toml_set_string(Value *value, const Rust::Slice<const char> &data);
    /// Sets the value to be an i64 of the given value
    extern "C" DLL_IMPORT void toml_set_i64(Value *value, i64 data);
    /// Sets the value to be an f64 of the given value
    extern "C" DLL_IMPORT void toml_set_f64(Value *value, f64 data);
    /// Sets the value to be a Datetime containing the data.
    /// Returns false and makes no changes to the value if the data is not valid UTF-8.
    extern "C" DLL_IMPORT bool toml_set_datetime(Value *value, const Rust::Slice<const char> &data);
    /// Sets the value to be a boolean with the given value
    extern "C" DLL_IMPORT void toml_set_bool(Value *value, bool data);
    /// Sets the value to be an empty array
    extern "C" DLL_IMPORT void toml_set_array(Value *value);
    /// Sets the value to be an empty table
    extern "C" DLL_IMPORT void toml_set_table(Value *value);
    
    /// Removes all items from an array
    extern "C" DLL_IMPORT void  toml_array_clear(Array *array);
    /// Gets the number of items in an array
    extern "C" DLL_IMPORT usize toml_array_len(const Array *array);
    /// Get the value at the given index in the array
    extern "C" DLL_IMPORT bool  toml_array_get(const Array *array, usize idx, const Value **value);
    /// Get a mutable reference to the value at the given index in the array
    extern "C" DLL_IMPORT bool  toml_array_get_mut(Array *array, usize idx, Value **value);
    /// Push a value onto the end of the array
    extern "C" DLL_IMPORT void  toml_array_push(Array *array, Value *value);
    /// Pop the value at the end of the array
    extern "C" DLL_IMPORT bool  toml_array_pop(Array *array);
    /// Insert a value at the specified index in the array
    extern "C" DLL_IMPORT bool  toml_array_insert(Array *array, usize idx, Value *value);
    /// Remove the value at the specified index in the array
    extern "C" DLL_IMPORT bool  toml_array_remove(Array *array, usize idx);
    
    /// Remove all items from the table
    extern "C" DLL_IMPORT void  toml_table_clear(Table *table);
    /// Get the number of key-value pairs in the table
    extern "C" DLL_IMPORT usize toml_table_len(const Table *table);
    /// Get a list of all of the keys in the table. key_list should
    /// point to an array with room for all of the keys in the table.
    extern "C" DLL_IMPORT bool  toml_table_keys(
        const Table *table,
        const Rust::Slice<Rust::Slice<const char>> &key_list
    );
    /// Get the value with the specified key from the table
    extern "C" DLL_IMPORT bool toml_table_get(
        const Table *table,
        const Rust::Slice<const char> &key,
        const Value **value
    );
    /// Get a mutable value with the specified key from the table
    extern "C" DLL_IMPORT bool toml_table_get_mut(
        Table *table,
        const Rust::Slice<const char> &key,
        Value **value
    );
    /// Insert a value into the table with the specified key
    extern "C" DLL_IMPORT bool toml_table_insert(
        Table *table,
        const Rust::Slice<const char> &key,
        Value *value
    );
    /// Remove the value in the table with the specified key
    extern "C" DLL_IMPORT bool toml_table_remove(
        Table *table,
        const Rust::Slice<const char> &key
    );
    
    extern "C" DLL_IMPORT bool toml_parse_text(
        const Rust::Slice<const char> &data,
        Value **output
    );
    
    extern "C" DLL_IMPORT void toml_serialize_text(
        const Value *data,
        Value **output
    );
}
}
