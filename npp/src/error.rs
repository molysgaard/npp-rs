use crate::{
    color::{PixelFormat, ColorSpace, ColorRange},
    ffi,
};
use cuda_rs::error::CuError;
use thiserror::Error;
use num_traits::FromPrimitive;

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum NppStatus {
        // NppStatus_NPP_NOT_SUPPORTED_MODE_ERROR
        NotSupportedModeError = ffi::NppStatus_NPP_NOT_SUPPORTED_MODE_ERROR as _,
        // NppStatus_NPP_INVALID_HOST_POINTER_ERROR
        InvalidHostPointerError = ffi::NppStatus_NPP_INVALID_HOST_POINTER_ERROR as _,
        // NppStatus_NPP_INVALID_DEVICE_POINTER_ERROR
        InvalidDevicePointerError = ffi::NppStatus_NPP_INVALID_DEVICE_POINTER_ERROR as _,
        // NppStatus_NPP_LUT_PALETTE_BITSIZE_ERROR
        LutPaletteBitsizeError = ffi::NppStatus_NPP_LUT_PALETTE_BITSIZE_ERROR as _,
        // NppStatus_NPP_ZC_MODE_NOT_SUPPORTED_ERROR
        ZcModeNotSupportedError = ffi::NppStatus_NPP_ZC_MODE_NOT_SUPPORTED_ERROR as _,
        // NppStatus_NPP_NOT_SUFFICIENT_COMPUTE_CAPABILITY
        NotSufficientComputeCapability = ffi::NppStatus_NPP_NOT_SUFFICIENT_COMPUTE_CAPABILITY as _,
        // NppStatus_NPP_TEXTURE_BIND_ERROR
        TextureBindError = ffi::NppStatus_NPP_TEXTURE_BIND_ERROR as _,
        // NppStatus_NPP_WRONG_INTERSECTION_ROI_ERROR
        WrongIntersectionRoiError = ffi::NppStatus_NPP_WRONG_INTERSECTION_ROI_ERROR as _,
        // NppStatus_NPP_HAAR_CLASSIFIER_PIXEL_MATCH_ERROR
        HaarClassifierPixelMatchError = ffi::NppStatus_NPP_HAAR_CLASSIFIER_PIXEL_MATCH_ERROR as _,
        // NppStatus_NPP_MEMFREE_ERROR
        MemfreeError = ffi::NppStatus_NPP_MEMFREE_ERROR as _,
        // NppStatus_NPP_MEMSET_ERROR
        MemsetError = ffi::NppStatus_NPP_MEMSET_ERROR as _,
        // NppStatus_NPP_MEMCPY_ERROR
        MemcpyError = ffi::NppStatus_NPP_MEMCPY_ERROR as _,
        // NppStatus_NPP_ALIGNMENT_ERROR
        AlignmentError = ffi::NppStatus_NPP_ALIGNMENT_ERROR as _,
        // NppStatus_NPP_CUDA_KERNEL_EXECUTION_ERROR
        CudaKernelExecutionError = ffi::NppStatus_NPP_CUDA_KERNEL_EXECUTION_ERROR as _,
        // NppStatus_NPP_ROUND_MODE_NOT_SUPPORTED_ERROR
        RoundModeNotSupportedError = ffi::NppStatus_NPP_ROUND_MODE_NOT_SUPPORTED_ERROR as _,
        // NppStatus_NPP_QUALITY_INDEX_ERROR
        QualityIndexError = ffi::NppStatus_NPP_QUALITY_INDEX_ERROR as _,
        // NppStatus_NPP_RESIZE_NO_OPERATION_ERROR
        ResizeNoOperationError = ffi::NppStatus_NPP_RESIZE_NO_OPERATION_ERROR as _,
        // NppStatus_NPP_OVERFLOW_ERROR
        OverflowError = ffi::NppStatus_NPP_OVERFLOW_ERROR as _,
        // NppStatus_NPP_NOT_EVEN_STEP_ERROR
        NotEvenStepError = ffi::NppStatus_NPP_NOT_EVEN_STEP_ERROR as _,
        // NppStatus_NPP_HISTOGRAM_NUMBER_OF_LEVELS_ERROR
        HistogramNumberOfLevelsError = ffi::NppStatus_NPP_HISTOGRAM_NUMBER_OF_LEVELS_ERROR as _,
        // NppStatus_NPP_LUT_NUMBER_OF_LEVELS_ERROR
        LutNumberOfLevelsError = ffi::NppStatus_NPP_LUT_NUMBER_OF_LEVELS_ERROR as _,
        // NppStatus_NPP_CORRUPTED_DATA_ERROR
        CorruptedDataError = ffi::NppStatus_NPP_CORRUPTED_DATA_ERROR as _,
        // NppStatus_NPP_CHANNEL_ORDER_ERROR
        ChannelOrderError = ffi::NppStatus_NPP_CHANNEL_ORDER_ERROR as _,
        // NppStatus_NPP_ZERO_MASK_VALUE_ERROR
        ZeroMaskValueError = ffi::NppStatus_NPP_ZERO_MASK_VALUE_ERROR as _,
        // NppStatus_NPP_QUADRANGLE_ERROR
        QuadrangleError = ffi::NppStatus_NPP_QUADRANGLE_ERROR as _,
        // NppStatus_NPP_RECTANGLE_ERROR
        RectangleError = ffi::NppStatus_NPP_RECTANGLE_ERROR as _,
        // NppStatus_NPP_COEFFICIENT_ERROR
        CoefficientError = ffi::NppStatus_NPP_COEFFICIENT_ERROR as _,
        // NppStatus_NPP_NUMBER_OF_CHANNELS_ERROR
        NumberOfChannelsError = ffi::NppStatus_NPP_NUMBER_OF_CHANNELS_ERROR as _,
        // NppStatus_NPP_COI_ERROR
        CioError = ffi::NppStatus_NPP_COI_ERROR as _,
        // NppStatus_NPP_DIVISOR_ERROR
        DivisorError = ffi::NppStatus_NPP_DIVISOR_ERROR as _,
        // NppStatus_NPP_CHANNEL_ERROR
        ChannelError = ffi::NppStatus_NPP_CHANNEL_ERROR as _,
        // NppStatus_NPP_STRIDE_ERROR
        StrideError = ffi::NppStatus_NPP_STRIDE_ERROR as _,
        // NppStatus_NPP_ANCHOR_ERROR
        AnchorError = ffi::NppStatus_NPP_ANCHOR_ERROR as _,
        // NppStatus_NPP_MASK_SIZE_ERROR
        MaskSizeError = ffi::NppStatus_NPP_MASK_SIZE_ERROR as _,
        // NppStatus_NPP_RESIZE_FACTOR_ERROR
        ResizeFactorError = ffi::NppStatus_NPP_RESIZE_FACTOR_ERROR as _,
        // NppStatus_NPP_INTERPOLATION_ERROR
        InterpolationError = ffi::NppStatus_NPP_INTERPOLATION_ERROR as _,
        // NppStatus_NPP_MIRROR_FLIP_ERROR
        MirrorFlipError = ffi::NppStatus_NPP_MIRROR_FLIP_ERROR as _,
        // NppStatus_NPP_MOMENT_00_ZERO_ERROR
        Moment00ZeroError = ffi::NppStatus_NPP_MOMENT_00_ZERO_ERROR as _,
        // NppStatus_NPP_THRESHOLD_NEGATIVE_LEVEL_ERROR
        ThresholdNegativeLevelError = ffi::NppStatus_NPP_THRESHOLD_NEGATIVE_LEVEL_ERROR as _,
        // NppStatus_NPP_THRESHOLD_ERROR
        ThresholdError = ffi::NppStatus_NPP_THRESHOLD_ERROR as _,
        // NppStatus_NPP_CONTEXT_MATCH_ERROR
        ContextMatchError = ffi::NppStatus_NPP_CONTEXT_MATCH_ERROR as _,
        // NppStatus_NPP_FFT_FLAG_ERROR
        FftFlagError = ffi::NppStatus_NPP_FFT_FLAG_ERROR as _,
        // NppStatus_NPP_FFT_ORDER_ERROR
        FftOrderError = ffi::NppStatus_NPP_FFT_ORDER_ERROR as _,
        // NppStatus_NPP_STEP_ERROR
        StepError = ffi::NppStatus_NPP_STEP_ERROR as _,
        // NppStatus_NPP_SCALE_RANGE_ERROR
        ScaleRangeError = ffi::NppStatus_NPP_SCALE_RANGE_ERROR as _,
        // NppStatus_NPP_DATA_TYPE_ERROR
        DataTypeError = ffi::NppStatus_NPP_DATA_TYPE_ERROR as _,
        // NppStatus_NPP_OUT_OFF_RANGE_ERROR
        OutOffRangeError = ffi::NppStatus_NPP_OUT_OFF_RANGE_ERROR as _,
        // NppStatus_NPP_DIVIDE_BY_ZERO_ERROR
        DivideByZeroError = ffi::NppStatus_NPP_DIVIDE_BY_ZERO_ERROR as _,
        // NppStatus_NPP_MEMORY_ALLOCATION_ERR
        MemoryAllocationError = ffi::NppStatus_NPP_MEMORY_ALLOCATION_ERR as _,
        // NppStatus_NPP_NULL_POINTER_ERROR
        NullPointerError = ffi::NppStatus_NPP_NULL_POINTER_ERROR as _,
        // NppStatus_NPP_RANGE_ERROR
        RangeError = ffi::NppStatus_NPP_RANGE_ERROR as _,
        // NppStatus_NPP_SIZE_ERROR
        SizeError = ffi::NppStatus_NPP_SIZE_ERROR as _,
        // NppStatus_NPP_BAD_ARGUMENT_ERROR
        BadArgumentError = ffi::NppStatus_NPP_BAD_ARGUMENT_ERROR as _,
        // NppStatus_NPP_NO_MEMORY_ERROR
        NoMemoryError = ffi::NppStatus_NPP_NO_MEMORY_ERROR as _,
        // NppStatus_NPP_NOT_IMPLEMENTED_ERROR
        NotImplementedError = ffi::NppStatus_NPP_NOT_IMPLEMENTED_ERROR as _,
        // NppStatus_NPP_ERROR
        Error = ffi::NppStatus_NPP_ERROR as _,
        // NppStatus_NPP_ERROR_RESERVED
        ErrorReserved = ffi::NppStatus_NPP_ERROR_RESERVED as _,
        // NppStatus_NPP_NO_OPERATION_WARNING
        NoOperationWarning = ffi::NppStatus_NPP_NO_OPERATION_WARNING as _,
        // NppStatus_NPP_DIVIDE_BY_ZERO_WARNING
        DivideByZeroWarning = ffi::NppStatus_NPP_DIVIDE_BY_ZERO_WARNING as _,
        // NppStatus_NPP_AFFINE_QUAD_INCORRECT_WARNING
        AffineQuadIncorrectWarning = ffi::NppStatus_NPP_AFFINE_QUAD_INCORRECT_WARNING as _,
        // NppStatus_NPP_WRONG_INTERSECTION_ROI_WARNING
        WrongIntersectionRoiWarning = ffi::NppStatus_NPP_WRONG_INTERSECTION_ROI_WARNING as _,
        // NppStatus_NPP_WRONG_INTERSECTION_QUAD_WARNING
        WrongIntersectionQuadWarning = ffi::NppStatus_NPP_WRONG_INTERSECTION_QUAD_WARNING as _,
        // NppStatus_NPP_DOUBLE_SIZE_WARNING
        DoubleSizeWarning = ffi::NppStatus_NPP_DOUBLE_SIZE_WARNING as _,
        // NppStatus_NPP_MISALIGNED_DST_ROI_WARNING
        MisalignedDstRoiWarning = ffi::NppStatus_NPP_MISALIGNED_DST_ROI_WARNING as _,
    }
}

impl From<ffi::NppStatus> for NppStatus {
    fn from(status: ffi::NppStatus) -> Self {
        NppStatus::from_i32(status).unwrap_or(NppStatus::Error)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum NppError {
    #[error("NPP Error: {0:?}")]
    Npp(NppStatus),
    #[error("CUDA Error: {0}")]
    Cuda(#[from] CuError),
    #[error("Unsupported pixel format conversion: {0:?} -> {1:?} (space: {2:?}, range: {3:?})")]
    UnsupportedCvtColor(PixelFormat, PixelFormat, ColorSpace, ColorRange),
}

pub type NppResult<T> = Result<T, NppError>;
