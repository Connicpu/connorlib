using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace ConnorLib
{
    [StructLayout(LayoutKind.Sequential)]
    public struct InRustStr
    {
        public static implicit operator InRustStr(string s)
        {
            var utf8 = Encoding.UTF8.GetBytes(s);
            return new InRustStr { data = utf8, len = (UIntPtr)utf8.Length };
        }
        
        [MarshalAs(UnmanagedType.LPArray)]
        private byte[] data;
        private UIntPtr len;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct OutRustStr
    {
        public static implicit operator string(OutRustStr s)
        {
            var len = (int)s.len;
            var bytes = new byte[len];
            Marshal.Copy(s.data, bytes, 0, len);
            return Encoding.UTF8.GetString(bytes, 0, len);
        }

        private IntPtr data;
        private UIntPtr len;
    }
}
