use crate::{
    color::{PixelFormat, ColorSpace, ColorRange},
    error::{NppResult, NppError},
    ffi,
    image::DeviceImage,
};

pub fn convert_pixel_format(
    src_image: &DeviceImage,
    dst_image: &mut DeviceImage,
) -> NppResult<()> {
    let res = match (src_image.pixel_format, dst_image.pixel_format) {
        (PixelFormat::NV12, PixelFormat::RGB) => unsafe {
            let y_ptr = src_image.get_raw();
            let offset = src_image.pitch() * src_image.height;
            let uv_ptr = y_ptr.add(offset);
            let mut src = [y_ptr, uv_ptr];

            match (src_image.color_space, src_image.color_range) {
                (ColorSpace::BT709, ColorRange::JPEG) =>
                    Some(ffi::nppiNV12ToRGB_709HDTV_8u_P2C3R(
                        src.as_mut_ptr() as _,
                        src_image.pitch() as _,
                        dst_image.get_raw(),
                        dst_image.pitch() as _,
                        ffi::NppiSize {
                            width: src_image.width as _,
                            height: src_image.height as _,
                        },
                    )),
                (ColorSpace::BT709, _) =>
                    Some(ffi::nppiNV12ToRGB_709CSC_8u_P2C3R(
                        src.as_mut_ptr() as _,
                        src_image.pitch() as _,
                        dst_image.get_raw(),
                        dst_image.pitch() as _,
                        ffi::NppiSize {
                            width: src_image.width as _,
                            height: src_image.height as _,
                        },
                    )),
                (ColorSpace::BT601, ColorRange::JPEG) =>
                    Some(ffi::nppiNV12ToRGB_8u_P2C3R(
                        src.as_mut_ptr() as _,
                        src_image.pitch() as _,
                        dst_image.get_raw(),
                        dst_image.pitch() as _,
                        ffi::NppiSize {
                            width: src_image.width as _,
                            height: src_image.height as _,
                        },
                    )),
                _ => None
            }
        },
        _ => None
    };

    if let Some(res) = res {
        wrap!((), res)
    } else {
        Err(NppError::UnsupportedCvtColor(
            src_image.pixel_format,
            dst_image.pixel_format,
            src_image.color_space,
            src_image.color_range,
        ))
    }
}
