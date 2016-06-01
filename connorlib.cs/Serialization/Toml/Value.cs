using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ConnorLib.Serialization.Toml
{
    public class Value
    {
        private TomlFFI.Value data;

        internal Value(TomlFFI.Value data)
        {
            this.data = data;
        }

        public Value(string value)
        {
            InRustStr src = value;
            data = TomlFFI.toml_new_string(ref src.data);
        }

        public Value(DateTime dt)
        {
            InRustStr src = dt.ToString("yyyy-MM-dd'T'HH:mm:ssK", DateTimeFormatInfo.InvariantInfo);
            data = TomlFFI.toml_new_datetime(ref src.data);
        }

        public Value(long x)
        {
            data = TomlFFI.toml_new_i64(x);
        }

        public Value(double x)
        {
            data = TomlFFI.toml_new_f64(x);
        }

        public Value(bool b)
        {
            data = TomlFFI.toml_new_bool(b);
        }

        public static Value NewTable()
        {
            var data = TomlFFI.toml_new_table();
            return new Value(data);
        }

        public static Value NewArray()
        {
            var data = TomlFFI.toml_new_array();
            return new Value(data);
        }

        public string String
        {
            get
            {
                OutRustStr str;
                if (!TomlFFI.toml_get_string(data, out str))
                    return null;
                return str;
            }
        }

        public DateTime? DateTime
        {
            get
            {
                OutRustStr dt;
                if (!TomlFFI.toml_get_datetime(data, out dt))
                    return null;
                return System.DateTime.Parse(dt, DateTimeFormatInfo.InvariantInfo);
            }
        }

        public long? Integer
        {
            get
            {
                long x;
                if (!TomlFFI.toml_get_i64(data, out x))
                    return null;
                return x;
            }
        }

        public double? Float
        {
            get
            {
                double x;
                if (!TomlFFI.toml_get_f64(data, out x))
                    return null;
                return x;
            }
        }

        public bool? Bool
        {
            get
            {
                bool b;
                if (!TomlFFI.toml_get_bool(data, out b))
                    return null;
                return b;
            }
        }

        
    }
}
