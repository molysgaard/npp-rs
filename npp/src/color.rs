#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PixelFormat {
    RGB,
    BGR,
    NV12,
    P016,
    NV21,
    YUV,
    YUV420,
    YUV422,
    YUV444,
    YUV444_16Bit,
    YCbCr,
    TCbCr420,
    TCbCr422,
    TCbCr444,
    CbYCr422,
    HSV,
    HLS,
    Lab,
    YCC,
    LUV,
    XYZ,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorSpace {
    UNSPEC,
    BT601,
    BT709,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorRange {
    UDEF,
    MPEG,
    JPEG,
}
