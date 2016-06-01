using ConnorLib;
using ConnorLib.Serialization.Toml;
using System;
using System.Diagnostics;

namespace Test
{
    class Program
    {
        static void Main(string[] args)
        {
            TomlFFI.Value root;
            TomlFFI.Table root_tbl;
            TomlFFI.Value test;
            TomlFFI.Table test_tbl;
            TomlFFI.Value x_val;
            long x;

            var value = TomlFFI.toml_new_table();
            var copy = TomlFFI.toml_clone(value);
            TomlFFI.toml_free_value(value);
            TomlFFI.toml_free_value(copy);

            InRustStr src = "[test]\nx = 5";
            TomlFFI.toml_parse_text(ref src.data, out root);
            
            TomlFFI.toml_get_table_mut(root, out root_tbl);

            var root_count = (int)TomlFFI.toml_table_len(root_tbl);
            var key_list = new GetRustStrList(root_count);
            TomlFFI.toml_table_keys(root_tbl, ref key_list.data);
            var keys = key_list.GetStrings();
            
            InRustStr key = keys[0];
            TomlFFI.toml_table_get_mut(root_tbl, ref key.data, out test);
            TomlFFI.toml_get_table_mut(test, out test_tbl);

            InRustStr x_key = "x";
            TomlFFI.toml_table_get_mut(test_tbl, ref x_key.data, out x_val);
            TomlFFI.toml_get_i64(x_val, out x);

            Debug.Assert(x == 5);

            var now = DateTime.Now.ToUniversalTime();
            var date = new Value(now);
            Debug.Assert((now - date.DateTime) < TimeSpan.FromSeconds(1));


            return;
        }
    }
}
