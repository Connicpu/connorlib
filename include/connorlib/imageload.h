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

#pragma once

#include <stdint.h>
#include <connorlib/dll.h>

namespace ImageLoad
{
    class ImageId;
    class Image;
    class MultiImage;
    class Frame;
    
    using buf_free_t = void(*)(const uint8_t *buf, size_t len);
    
    extern "C" IMG_DLL_IMPORT void image_free_id(ImageId *id);
    extern "C" IMG_DLL_IMPORT void image_free(Image *img);
    extern "C" IMG_DLL_IMPORT void image_free_multi(MultiImage *img);
    
    // Loads the image at the given path
    extern "C" IMG_DLL_IMPORT ImageId * image_open_path(const char *path);
    // NOTE: This buffer must be valid for the life of the ImageId and
    // any image/multiimage created from it!
    extern "C" IMG_DLL_IMPORT ImageId * image_open_buffer(const uint8_t *buf, size_t len);
    // If you would like imageload to maintain its own copy of the data so you can release your
    // buffer, use this!
    extern "C" IMG_DLL_IMPORT ImageId * image_open_buffer_copy(const uint8_t *buf, size_t len);
    // The ImageId will take ownership of your buffer and free it with `free` upon closing
    extern "C" IMG_DLL_IMPORT ImageId * image_open_buffer_owned(const uint8_t *buf, size_t len, buf_free_t free);
    
    // IMPORTANT NOTE: load_multi functions consume the ImageId, the regular ones do not.
    // If you pass an ImageId to one of these functions, you are no longer responsible
    // for freeing the value.
    
    extern "C" IMG_DLL_IMPORT Image * image_load_png(const ImageId *id);
    extern "C" IMG_DLL_IMPORT Image * image_load_jpg(const ImageId *id);
    extern "C" IMG_DLL_IMPORT Image * image_load_gif(const ImageId *id);
    extern "C" IMG_DLL_IMPORT Image * image_load_webp(const ImageId *id);
    extern "C" IMG_DLL_IMPORT Image * image_load_ppm(const ImageId *id);
    extern "C" IMG_DLL_IMPORT Image * image_load_bmp(const ImageId *id);
    extern "C" IMG_DLL_IMPORT Image * image_load_ico(const ImageId *id);
    extern "C" IMG_DLL_IMPORT MultiImage * image_load_multi_gif(ImageId *id);
    
    extern "C" IMG_DLL_IMPORT const Frame * image_get_frame(const Image *image);
    extern "C" IMG_DLL_IMPORT const Frame * image_get_frame_multi(MultiImage *image, uint32_t index, uint16_t *delay);
    extern "C" IMG_DLL_IMPORT void image_get_size(const Image *image, uint32_t *width, uint32_t *height);
    extern "C" IMG_DLL_IMPORT void image_get_multi_size(const MultiImage *image, uint32_t *width, uint32_t *height);
    
    // Always gives Rgba8 pixels. Size of *buffer is 4*width*height.
    extern "C" IMG_DLL_IMPORT void image_get_frame_buffer(const Frame *frame, const uint8_t **buffer);
}
