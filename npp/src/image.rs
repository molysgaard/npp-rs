use crate::{
    color::{PixelFormat, ColorSpace, ColorRange},
    cvt_color::convert_pixel_format,
    error::NppResult,
};
use cuda_rs::{
    memory::PitchedDeviceMemory,
    stream::CuStream,
};

pub struct DeviceImage {
    pub mem: PitchedDeviceMemory,
    pub width: usize,
    pub height: usize,
    pub pixel_format: PixelFormat,
    pub color_space: ColorSpace,
    pub color_range: ColorRange,
}

impl DeviceImage {
    pub fn new(
        width: usize,
        height: usize,
        pixel_format: PixelFormat,
        color_space: ColorSpace,
        color_range: ColorRange,
        stream: &CuStream,
    ) -> NppResult<Self> {
        let (mem_width, mem_height) = get_memory_size(width, height, pixel_format);
        let mem = PitchedDeviceMemory::new(
            mem_width, mem_height, stream
        )?;
        Ok(Self {
            mem, width, height, pixel_format, color_space, color_range
        })
    }

    pub fn from_memory(
        mem: PitchedDeviceMemory,
        width: usize,
        height: usize,
        pixel_format: PixelFormat,
        color_space: ColorSpace,
        color_range: ColorRange,
    ) -> Self {
        Self {
            mem, width, height, pixel_format, color_space, color_range
        }
    }

    pub fn convert_pixel_format(
        &self, dst_pixel_format: PixelFormat, stream: &CuStream
    ) -> NppResult<DeviceImage> {
        let mut dst = DeviceImage::new(
            self.width,
            self.height,
            dst_pixel_format,
            self.color_space,
            self.color_range,
            stream,
        )?;
        convert_pixel_format(self, &mut dst)?;
        Ok(dst)
    }

    pub fn pitch(&self) -> usize {
        self.mem.pitch
    }

    pub fn get_raw(&self) -> *mut u8 {
        unsafe { self.mem.get_raw() as *mut u8 }
    }
}

pub fn get_memory_size(width: usize, height: usize, pixel_format: PixelFormat) -> (usize, usize) {
    match pixel_format {
        PixelFormat::RGB | PixelFormat::BGR | PixelFormat::HSV => (width * 3, height),
        PixelFormat::NV12 => (width, height * 3 / 2),
        _ => unimplemented!(),
    }
}
