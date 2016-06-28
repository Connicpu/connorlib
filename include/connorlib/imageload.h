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

#include <connorlib/dll.h>
#include <stdint.h>
#include <cassert>
#include <stdexcept>
#include <chrono>

namespace ImageLoad
{
    using duration = std::chrono::duration<uint32_t, std::milli>;

    namespace FFI
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

    class ImageId
    {
    public:
        static ImageId Path(const char *path)
        {
            auto id = FFI::image_open_path(path);
            if (!id)
            {
                throw std::logic_error{ "Invalid path for ImageId" };
            }
            return ImageId{ id };
        }

        static ImageId Borrowed(const uint8_t *buf, size_t len)
        {
            auto id = FFI::image_open_buffer(buf, len);
            assert(id);
            return ImageId{ id };
        }

        static ImageId CopiedBuffer(const uint8_t *buf, size_t len)
        {
            auto id = FFI::image_open_buffer_copy(buf, len);
            assert(id);
            return ImageId{ id };
        }

        static ImageId OwnedBuffer(const uint8_t *buf, size_t len, FFI::buf_free_t free)
        {
            auto id = FFI::image_open_buffer_owned(buf, len, free);
            assert(id);
            return ImageId{ id };
        }

        ImageId(const ImageId &) = delete;
        ImageId(ImageId &&move)
            : id(move.id)
        {
            move.id = nullptr;
        }

        ImageId &operator=(const ImageId &) = delete;
        ImageId &operator=(ImageId &&move)
        {
            id = move.id;
            move.id = nullptr;
            return *this;
        }

        ~ImageId()
        {
            if (id)
            {
                FFI::image_free_id(id);
            }
        }

        operator const FFI::ImageId *() const
        {
            return id;
        }

        FFI::ImageId *_Release()
        {
            auto ptr = id;
            id = nullptr;
            return ptr;
        }

    private:
        ImageId(FFI::ImageId *id)
            : id(id)
        {
        }

        FFI::ImageId *id;
    };

    class Frame
    {
    public:
        Frame()
            : frame(frame)
        {
        }

        explicit Frame(const FFI::Frame *frame)
            : frame(frame)
        {
        }

        void GetBuffer(const uint8_t **buffer) const
        {
            FFI::image_get_frame_buffer(frame, buffer);
        }

    private:
        const FFI::Frame *frame;
    };

    class Image
    {
    public:
        static Image LoadPng(const ImageId &id)
        {
            auto img = FFI::image_load_png(id);
            if (!img)
                throw std::runtime_error{ "Bad PNG file" };
            return Image{ img };
        }
        static Image LoadJpg(const ImageId &id)
        {
            auto img = FFI::image_load_jpg(id);
            if (!img)
                throw std::runtime_error{ "Bad JPEG file" };
            return Image{ img };
        }
        static Image LoadGif(const ImageId &id)
        {
            auto img = FFI::image_load_gif(id);
            if (!img)
                throw std::runtime_error{ "Bad GIF file" };
            return Image{ img };
        }
        static Image LoadWebp(const ImageId &id)
        {
            auto img = FFI::image_load_webp(id);
            if (!img)
                throw std::runtime_error{ "Bad WEBP file" };
            return Image{ img };
        }
        static Image LoadPpm(const ImageId &id)
        {
            auto img = FFI::image_load_ppm(id);
            if (!img)
                throw std::runtime_error{ "Bad PPM file" };
            return Image{ img };
        }
        static Image LoadBmp(const ImageId &id)
        {
            auto img = FFI::image_load_bmp(id);
            if (!img)
                throw std::runtime_error{ "Bad BMP file" };
            return Image{ img };
        }
        static Image LoadIco(const ImageId &id)
        {
            auto img = FFI::image_load_ico(id);
            if (!img)
                throw std::runtime_error{ "Bad ICO file" };
            return Image{ img };
        }

        Image(const Image &) = delete;
        Image(Image &&move)
            : img(move.img)
        {
            move.img = nullptr;
        }

        Image &operator=(const Image &) = delete;
        Image &operator=(Image &&move)
        {
            img = move.img;
            move.img = nullptr;
        }

        void GetSize(uint32_t *width, uint32_t *height)
        {
            FFI::image_get_size(img, width, height);
        }

        Frame GetFrame()
        {
            return Frame{ FFI::image_get_frame(img) };
        }

        ~Image()
        {
            if (img)
            {
                FFI::image_free(img);
            }
        }

    private:
        Image(FFI::Image *img)
            : img(img)
        {
        }

        FFI::Image *img;
    };

    class GifIter
    {
    public:
        GifIter()
            : state(nullptr), next_index(0)
        {
        }

        GifIter(FFI::MultiImage *state)
            : state(state), next_index(0)
        {
            ++*this;
        }

        GifIter &operator++()
        {
            assert(state);

            uint16_t delay;
            auto ptr = FFI::image_get_frame_multi(state, next_index++, &delay);
            if (!ptr)
            {
                state = nullptr;
                next_index = 0;
            }

            value.first = Frame{ ptr };
            value.second = std::chrono::milliseconds(uint32_t(delay) * 10);

            return *this;
        }

        const std::pair<Frame, duration> &operator*() const
        {
            return value;
        }

        const std::pair<Frame, duration> *operator->() const
        {
            return &value;
        }

        bool operator==(const GifIter &rhs) const
        {
            return state == rhs.state && next_index == rhs.next_index;
        }

        bool operator!=(const GifIter &rhs) const
        {
            return !(*this == rhs);
        }

    private:
        FFI::MultiImage *state;
        uint32_t next_index;
        std::pair<Frame, duration> value;
    };

    class AnimatedGif
    {
    public:
        static AnimatedGif Load(ImageId &&id)
        {
            auto ptr = id._Release();
            auto state = FFI::image_load_multi_gif(ptr);
            if (!state)
                throw std::runtime_error{ "Bad GIF file" };
            return AnimatedGif{ state };
        }

        AnimatedGif(const AnimatedGif &) = delete;
        AnimatedGif(AnimatedGif &&move)
            : state(move.state)
        {
            move.state = nullptr;
        }

        AnimatedGif &operator=(const AnimatedGif &) = delete;
        AnimatedGif &operator=(AnimatedGif &&move)
        {
            state = move.state;
            move.state = nullptr;
        }

        void GetSize(uint32_t *width, uint32_t *height)
        {
            FFI::image_get_multi_size(state, width, height);
        }

        bool GetFrame(uint32_t index, Frame *frame, duration *delay)
        {
            uint16_t udelay;
            auto pframe = image_get_frame_multi(state, index, &udelay);
            if (!pframe)
                return false;

            *frame = Frame{ pframe };
            *delay = std::chrono::milliseconds(uint32_t(udelay) * 10);
            return true;
        }

        GifIter begin()
        {
            return GifIter{ state };
        }

        GifIter end()
        {
            return GifIter{};
        }

        ~AnimatedGif()
        {
            if (state)
            {
                FFI::image_free_multi(state);
            }
        }

    private:
        AnimatedGif(FFI::MultiImage *state)
            : state(state)
        {
        }

        FFI::MultiImage *state;
    };
}
