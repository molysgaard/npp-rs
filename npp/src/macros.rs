macro_rules! wrap {
    ($val:expr, $res:ident) => (
        if $res >= 0 {
            Ok($val)
        } else {
            use crate::error::{NppError, NppStatus};
            Err(NppError::Npp(NppStatus::from($res)))
        }
    )
}
