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

use super::{ImageId, Image, MultiImage, Buffer};
use std::{slice, ptr};
use std::path::PathBuf;
use std::os::raw::c_char;
use std::ffi::CStr;
use image;

#[no_mangle]
pub extern "C" fn image_free_id(id: *mut ImageId) {
    let _ = unsafe { Box::from_raw(id) };
}

#[no_mangle]
pub extern "C" fn image_free(id: *mut Image) {
    let _ = unsafe { Box::from_raw(id) };
}

#[no_mangle]
pub extern "C" fn image_free_multi(id: *mut MultiImage) {
    let _ = unsafe { Box::from_raw(id) };
}

#[no_mangle]
pub extern "C" fn image_open_path(path: *const c_char) -> *mut ImageId {
    let path: &CStr = unsafe { CStr::from_ptr(path) };
    let path: &str = match path.to_str() {
        Ok(path) => path,
        Err(_) => return ptr::null_mut(),
    };
    Box::into_raw(Box::new(ImageId::File(PathBuf::from(path))))
}

#[no_mangle]
pub extern "C" fn image_open_buffer(buf: *const u8, len: usize) -> *mut ImageId {
    let slice = unsafe { slice::from_raw_parts(buf, len) };
    let id = ImageId::Borrowed(slice as *const [u8]);
    Box::into_raw(Box::new(id))
}

#[no_mangle]
pub extern "C" fn image_open_buffer_copy(buf: *const u8, len: usize) -> *mut ImageId {
    let slice = unsafe { slice::from_raw_parts(buf, len) };
    let mut vec = Vec::with_capacity(len);
    unsafe { vec.set_len(len) };
    (&mut vec[..]).copy_from_slice(slice);
    let id = ImageId::Owned(Buffer::Boxed(vec.into_boxed_slice()));
    Box::into_raw(Box::new(id))
}

#[no_mangle]
pub extern "C" fn image_open_buffer_owned(buf: *const u8, len: usize, free: fn(*const u8, usize)) -> *mut ImageId {
    let id = ImageId::Owned(Buffer::Allocated(buf, len, free));
    Box::into_raw(Box::new(id))
}

#[no_mangle]
pub extern "C" fn image_load_png(id: *const ImageId) -> *mut Image {
    let id = unsafe { &*id };
    Box::into_raw(match Image::load(id, image::PNG) {
        Ok(img) => Box::new(img),
        Err(_) => return ptr::null_mut(),
    })
}

#[no_mangle]
pub extern "C" fn image_load_jpg(id: *const ImageId) -> *mut Image {
    let id = unsafe { &*id };
    Box::into_raw(match Image::load(id, image::JPEG) {
        Ok(img) => Box::new(img),
        Err(_) => return ptr::null_mut(),
    })
}

#[no_mangle]
pub extern "C" fn image_load_gif(id: *const ImageId) -> *mut Image {
    let id = unsafe { &*id };
    Box::into_raw(match Image::load(id, image::GIF) {
        Ok(img) => Box::new(img),
        Err(_) => return ptr::null_mut(),
    })
}

#[no_mangle]
pub extern "C" fn image_load_webp(id: *const ImageId) -> *mut Image {
    let id = unsafe { &*id };
    Box::into_raw(match Image::load(id, image::WEBP) {
        Ok(img) => Box::new(img),
        Err(_) => return ptr::null_mut(),
    })
}

#[no_mangle]
pub extern "C" fn image_load_ppm(id: *const ImageId) -> *mut Image {
    let id = unsafe { &*id };
    Box::into_raw(match Image::load(id, image::PPM) {
        Ok(img) => Box::new(img),
        Err(_) => return ptr::null_mut(),
    })
}

#[no_mangle]
pub extern "C" fn image_load_bmp(id: *const ImageId) -> *mut Image {
    let id = unsafe { &*id };
    Box::into_raw(match Image::load(id, image::BMP) {
        Ok(img) => Box::new(img),
        Err(_) => return ptr::null_mut(),
    })
}

#[no_mangle]
pub extern "C" fn image_load_ico(id: *const ImageId) -> *mut Image {
    let id = unsafe { &*id };
    Box::into_raw(match Image::load(id, image::ICO) {
        Ok(img) => Box::new(img),
        Err(_) => return ptr::null_mut(),
    })
}

#[no_mangle]
pub extern "C" fn image_load_multi_gif(id: *mut ImageId) -> *mut MultiImage {
    let id = unsafe { Box::from_raw(id) };
    Box::into_raw(match MultiImage::new(*id) {
        Ok(img) => Box::new(img),
        Err(_) => return ptr::null_mut(),
    })
}

#[no_mangle]
pub extern "C" fn image_get_frame(image: *const Image) -> *const image::Frame {
    unsafe { &(*image).frame }
}

#[no_mangle]
pub extern "C" fn image_get_frame_multi(image: *mut MultiImage, index: u32, delay: *mut u16) -> *const image::Frame {
    let image: &mut MultiImage = unsafe { &mut *image };
    let frame = match image.request_frame(index as usize) {
        Some(frame) => {
            frame as *const _
        },
        None => return ptr::null(),
    };
    unsafe { *delay = image.delay };
    frame
}

#[no_mangle]
pub extern "C" fn image_get_size(image: *const Image, width: *mut u32, height: *mut u32) {
    unsafe {
        let image: &Image = &*image;
        *width = image.frame.buffer().width();
        *height = image.frame.buffer().height();
    }
}

#[no_mangle]
pub extern "C" fn image_get_multi_size(image: *const MultiImage, width: *mut u32, height: *mut u32) {
    unsafe {
        let image: &MultiImage = &*image;
        *width = image.width;
        *height = image.height;
    }
}

#[no_mangle]
pub extern "C" fn image_get_frame_buffer(frame: *const image::Frame, buffer: *mut *const u8) {
    unsafe {
        let frame: &image::Frame = &*frame;
        *buffer = (&frame.buffer()[(0, 0)]) as *const image::Rgba<u8> as *const u8
    }
}

