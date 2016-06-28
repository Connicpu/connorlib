using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace ConnorLib.ImageLoad
{
    public static class FFI
    {
        [StructLayout(LayoutKind.Sequential)]
        public struct ImageId { public IntPtr handle; }

        [StructLayout(LayoutKind.Sequential)]
        public struct Image { public IntPtr handle; }

        [StructLayout(LayoutKind.Sequential)]
        public struct MultiImage { public IntPtr handle; }

        [StructLayout(LayoutKind.Sequential)]
        public struct Frame { public IntPtr handle; }

        [DllImport("imageload.dll")]
        public static extern void image_free_id(ImageId id);
        [DllImport("imageload.dll")]
        public static extern void image_free(Image id);
        [DllImport("imageload.dll")]
        public static extern void image_free_multi(MultiImage id);

        [DllImport("imageload.dll")]
        public static extern ImageId image_open_path(
            [MarshalAs(UnmanagedType.LPStr)] string path
            );
        // This is the only buffer one that I would consider "safe" to use without explicit management of the buffer
        [DllImport("imageload.dll")]
        public static extern ImageId image_open_buffer_copy(
            [MarshalAs(UnmanagedType.LPArray, SizeParamIndex = 1)] byte[] buf,
            UIntPtr len
            );

        [DllImport("imageload.dll")]
        public static extern Image image_load_png(ImageId id);
        [DllImport("imageload.dll")]
        public static extern Image image_load_jpg(ImageId id);
        [DllImport("imageload.dll")]
        public static extern Image image_load_gif(ImageId id);
        [DllImport("imageload.dll")]
        public static extern Image image_load_webp(ImageId id);
        [DllImport("imageload.dll")]
        public static extern Image image_load_ppm(ImageId id);
        [DllImport("imageload.dll")]
        public static extern Image image_load_bmp(ImageId id);
        [DllImport("imageload.dll")]
        public static extern Image image_load_ico(ImageId id);
        [DllImport("imageload.dll")]
        public static extern MultiImage image_load_multi_gif(ImageId id);
        
        [DllImport("imageload.dll")]
        public static extern Frame image_get_frame(Image image);
        [DllImport("imageload.dll")]
        public static extern Frame image_get_frame_multi(MultiImage image, uint index, out ushort delay);
        [DllImport("imageload.dll")]
        public static extern void image_get_size(Image image, out uint width, out uint height);
        [DllImport("imageload.dll")]
        public static extern void image_get_multi_size(MultiImage image, out uint width, out uint height);

        [DllImport("imageload.dll")]
        public static extern void image_get_frame_buffer(Frame frame, out IntPtr buffer);
    }
}
