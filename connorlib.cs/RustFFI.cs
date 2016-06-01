using System;
using System.Runtime.InteropServices;
using System.Text;

namespace ConnorLib
{
    internal class InRustStr
    {
        private InRustStr()
        {
        }

        public static implicit operator InRustStr(string s)
        {
            var utf8 = Encoding.UTF8.GetBytes(s);
            var ary = Marshal.AllocHGlobal(utf8.Length);
            Marshal.Copy(utf8, 0, ary, utf8.Length);

            return new InRustStr
            {
                ary = ary,
                data = new Data(ary, (UIntPtr)utf8.Length),
            };
        }

        ~InRustStr()
        {
            if (ary != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(ary);
                ary = IntPtr.Zero;
            }
        }

        [StructLayout(LayoutKind.Sequential)]
        internal struct Data
        {
            internal Data(IntPtr d, UIntPtr l)
            {
                data = d;
                len = l;
            }

            internal IntPtr data;
            internal UIntPtr len;
        }

        private IntPtr ary;
        internal Data data;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct OutRustStr
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

    [StructLayout(LayoutKind.Sequential)]
    internal class GetRustStrList
    {
        internal GetRustStrList(int c)
        {
            count = c;
            ary = Marshal.AllocHGlobal(count * Marshal.SizeOf<OutRustStr>());
            data = new Data(ary, (UIntPtr)count);
        }

        internal string[] GetStrings()
        {
            var strings = new string[count];
            for (int i = 0; i < count; ++i)
            {
                var offset = i * Marshal.SizeOf<Data>();
                strings[i] = Marshal.PtrToStructure<OutRustStr>((IntPtr)((ulong)ary + (ulong)offset));
            }
            return strings;
        }

        ~GetRustStrList()
        {
            if (ary != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(ary);
                ary = IntPtr.Zero;
            }
        }

        [StructLayout(LayoutKind.Sequential)]
        internal struct Data
        {
            internal Data(IntPtr d, UIntPtr l)
            {
                data = d;
                len = l;
            }

            IntPtr data;
            UIntPtr len;
        }

        private IntPtr ary;
        private int count;
        internal Data data;
    }
}
