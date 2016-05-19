using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Test
{
    class Program
    {
        static void Main(string[] args)
        {
            var value = ConnorLib.TomlFFI.toml_new_table();
            var copy = ConnorLib.TomlFFI.toml_clone(value);
            ConnorLib.TomlFFI.toml_free_value(value);
            ConnorLib.TomlFFI.toml_free_value(copy);
        }
    }
}
