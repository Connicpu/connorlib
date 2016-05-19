using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;

namespace ConnorLib
{
    public static class TomlFFI
    {
        const string _dll = "serialization.dll";

        [StructLayout(LayoutKind.Sequential)]
        public struct Value { public IntPtr handle; }

        [StructLayout(LayoutKind.Sequential)]
        public struct Table { public IntPtr handle; }

        [StructLayout(LayoutKind.Sequential)]
        public struct Array { public IntPtr handle; }

        [DllImport(_dll)]
        public static extern void toml_free_value(Value value);

        [DllImport(_dll)]
        public static extern Value toml_new_string([In] ref InRustStr data);
        [DllImport(_dll)]
        public static extern Value toml_new_i64(long data);
        [DllImport(_dll)]
        public static extern Value toml_new_f64(double data);
        [DllImport(_dll)]
        public static extern Value toml_new_datetime([In] ref InRustStr data);
        [DllImport(_dll)]
        public static extern Value toml_new_array();
        [DllImport(_dll)]
        public static extern Value toml_new_table();
        [DllImport(_dll)]
        public static extern Value toml_clone(Value value);

        [DllImport(_dll)]
        public static extern bool toml_get_string(Value value, out OutRustStr data);
        [DllImport(_dll)]
        public static extern bool toml_get_i64(Value value, out long data);
        [DllImport(_dll)]
        public static extern bool toml_get_f64(Value value, out double data);
        [DllImport(_dll)]
        public static extern bool toml_get_bool(Value value, out bool data);
        [DllImport(_dll)]
        public static extern bool toml_get_datetime(Value value, out OutRustStr data);
        [DllImport(_dll)]
        public static extern bool toml_get_array_mut(Value value, out Array array);
        [DllImport(_dll)]
        public static extern bool toml_get_table_mut(Value value, out Table table);

        [DllImport(_dll)]
        public static extern void toml_set_string(Value value, [In] ref InRustStr data);
        [DllImport(_dll)]
        public static extern void toml_set_i64(Value value, long data);
        [DllImport(_dll)]
        public static extern void toml_set_f64(Value value, double data);
        [DllImport(_dll)]
        public static extern void toml_set_bool(Value value, bool data);
        [DllImport(_dll)]
        public static extern void toml_set_datetime(Value value, [In] ref InRustStr data);
        [DllImport(_dll)]
        public static extern void toml_set_array(Value value);
        [DllImport(_dll)]
        public static extern void toml_set_table(Value value);
    }
}

