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

#![allow(dead_code)]

extern crate image;
extern crate gif;
extern crate owning_ref;
extern crate memmap;

use std::path::PathBuf;
use std::rc::Rc;
use std::{io, fs, slice};

pub mod ffi;

pub enum Buffer {
    Boxed(Box<[u8]>),
    Allocated(*const u8, usize, unsafe fn(*const u8, usize)),
}

impl Buffer {
    pub fn get(&self) -> &[u8] {
        match *self {
            Buffer::Boxed(ref data) => &**data,
            Buffer::Allocated(data, size, _) => unsafe { slice::from_raw_parts(data, size) },
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        match *self {
            Buffer::Allocated(data, size, free) => unsafe { free(data, size) },
            _ => {}
        }
    }
}

pub enum ImageId {
    File(PathBuf),
    Borrowed(*const [u8]),
    Owned(Buffer),
}

pub struct Image {
    frame: image::Frame,
}

impl Image {
    fn get_reader(id: &ImageId) -> io::Result<io::Cursor<SrcData>> {
        let id = match *id {
            ImageId::File(ref path) => ImageId::File(path.clone()),
            ImageId::Borrowed(ptr) => ImageId::Borrowed(ptr),
            ImageId::Owned(ref data) => ImageId::Borrowed(data.get() as *const [u8]),
        };
        
        Ok(io::Cursor::new(try!(ImageSrc::new(id))))
    }
    
    pub fn load(id: &ImageId, format: image::ImageFormat) -> image::ImageResult<Image> {
        let reader = try!(Image::get_reader(id));
        let dyn = try!(image::load(reader, format));
        let frame = image::Frame::new(dyn.to_rgba());
        Ok(Image {
            frame: frame,
        })
    }
}

type SrcData = owning_ref::OwningRef<Rc<ImageSrc>, [u8]>;

enum ImageSrc {
    File(memmap::Mmap),
    Borrowed(*const [u8]),
    Owned(Buffer),
}

impl ImageSrc {
    pub fn new(id: ImageId) -> io::Result<SrcData> {
        Ok(ImageSrc::make_data(try!(ImageSrc::open_src(id))))
    }
    
    fn open_src(id: ImageId) -> io::Result<ImageSrc> {
        use ImageSrc::*;
        Ok(match id {
            ImageId::File(path) => {
                let file = try!(fs::File::open(path));
                let mmap = try!(memmap::Mmap::open(&file, memmap::Protection::Read));
                File(mmap)
            },
            ImageId::Borrowed(ptr) => Borrowed(ptr),
            ImageId::Owned(data) => Owned(data),
        })
    }
    
    fn make_data(data: ImageSrc) -> SrcData {
        use owning_ref::OwningRef;
        let base_ref = OwningRef::<Rc<ImageSrc>, ImageSrc>::new(Rc::new(data));
        base_ref.map(|data| {
            use ImageSrc::*;
            match *data {
                File(ref mmap) => unsafe { mmap.as_slice() },
                Borrowed(ptr) => unsafe { &*ptr },
                Owned(ref data) => data.get(),
            }
        })
    }
}

pub struct MultiImage {
    current_frame: Option<image::Frame>,
    decoder: Option<gif::Reader<io::Cursor<SrcData>>>,
    source: SrcData,
    next_frame: usize,
    width: u32,
    height: u32,
    delay: u16,
}

impl MultiImage {
    pub fn new(id: ImageId) -> Result<MultiImage, gif::DecodingError> {
        let mut image = MultiImage {
            next_frame: 0,
            width: 0,
            height: 0,
            delay: 0,
            current_frame: None,
            decoder: None,
            source: try!(ImageSrc::new(id)),
        };
        
        try!(image.setup_decoder());
        Ok(image)
    }
    
    pub fn request_frame(&mut self, num: usize) -> Option<&image::Frame> {
        if self.current_frame.is_none() || num + 1 != self.next_frame {
            self.load_frame(num)
        } else {
            self.current_frame.as_ref()
        }
    }
    
    fn setup_decoder(&mut self) -> Result<(), gif::DecodingError> {
        use gif::{ColorOutput, SetParameter};
        let mut decoder = gif::Decoder::new(io::Cursor::new(self.source.clone()));
        decoder.set(ColorOutput::RGBA);
        let reader = try!(decoder.read_info());
        self.width = reader.width() as u32;
        self.height = reader.height() as u32;
        self.decoder = Some(reader);
        self.next_frame = 0;
        self.current_frame = None;
        Ok(())
    }
    
    fn load_frame(&mut self, num: usize) -> Option<&image::Frame> {
        use image::{ImageBuffer, DynamicImage};
        
        if self.decoder.is_none() || self.next_frame > num {
            match self.setup_decoder() {
                Ok(_) => {},
                Err(_) => return None,
            }
        }
        
        let mut reader = self.decoder.take().unwrap();
        while self.next_frame < num {
            match reader.next_frame_info() {
                Ok(Some(_)) => {},
                _ => return None,
            }
            self.next_frame += 1;
        }
        
        let (width, height, frame_buf) = match reader.read_next_frame() {
            Ok(Some(frame)) => {
                self.delay = frame.delay;
                (
                    frame.width as u32,
                    frame.height as u32,
                    frame.buffer.clone().into_owned(),
                )
            },
            _ => return None,
        };
        self.next_frame += 1;
        
        let raw_buf = ImageBuffer::from_raw(width, height, frame_buf);
        let buf = match raw_buf.map(|v| DynamicImage::ImageRgba8(v)) {
            Some(buf) => buf,
            None => return None,
        };
        
        self.decoder = Some(reader);
        self.current_frame = Some(image::Frame::new(buf.to_rgba()));
        self.current_frame.as_ref()
    }
}
