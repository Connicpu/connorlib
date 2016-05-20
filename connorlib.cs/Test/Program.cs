using ConnorLib;

namespace Test
{
    class Program
    {
        static void Main(string[] args)
        {
            var value = TomlFFI.toml_new_table();
            var copy = TomlFFI.toml_clone(value);
            TomlFFI.toml_free_value(value);
            TomlFFI.toml_free_value(copy);

            InRustStr src = "[test]\nx = 5";
            TomlFFI.Value root;
            TomlFFI.toml_parse_text(ref src.data, out root);
            
            TomlFFI.Table root_tbl;
            TomlFFI.toml_get_table_mut(root, out root_tbl);

            var root_count = (int)TomlFFI.toml_table_len(root_tbl);
            var key_list = new GetRustStrList(root_count);
            TomlFFI.toml_table_keys(root_tbl, ref key_list.data);
            var keys = key_list.GetStrings();

            return;
        }
    }
}
