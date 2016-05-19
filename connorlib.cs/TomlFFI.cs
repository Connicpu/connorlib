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
    }
}

