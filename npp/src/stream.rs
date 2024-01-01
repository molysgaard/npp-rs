use crate::{
    error::NppResult,
    ffi,
};
use cuda_rs::stream::CuStream;

pub struct NppStreamContext(ffi::NppStreamContext);

impl NppStreamContext {
    pub fn try_default() -> NppResult<Self> {
        let mut stream_ctx: ffi::NppStreamContext = unsafe { std::mem::zeroed() };
        let res = unsafe {
            ffi::nppGetStreamContext(&mut stream_ctx as _)
        };

        wrap!(Self(stream_ctx), res)
    }

    pub fn set_global_stream(stream: &CuStream) -> NppResult<()> {
        let stream_raw = unsafe { stream.get_raw() as _ };
        let res = unsafe {
            ffi::nppSetStream(stream_raw)
        };

        wrap!((), res)
    }

    pub fn set_stream(&mut self, stream: &CuStream) {
        self.0.hStream = unsafe { stream.get_raw() as _ };
    }
}
