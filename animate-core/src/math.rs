#[cfg(all(not(feature = "std"), not(feature = "libm")))]
compile_error!("`no_std` builds require the `libm` feature");

#[inline(always)]
pub(crate) fn sqrt(x: f32) -> f32 {
    #[cfg(feature = "libm")]
    return libm::sqrtf(x);
    #[cfg(all(not(feature = "libm"), feature = "std"))]
    return f32::sqrt(x);
    #[cfg(all(not(feature = "libm"), not(feature = "std")))]
    unimplemented!("`no_std` builds require the `libm` feature");
}

#[inline(always)]
pub(crate) fn round(x: f32) -> f32 {
    #[cfg(feature = "libm")]
    return libm::roundf(x);
    #[cfg(all(not(feature = "libm"), feature = "std"))]
    return f32::round(x);
    #[cfg(all(not(feature = "libm"), not(feature = "std")))]
    unimplemented!("`no_std` builds require the `libm` feature");
}

#[inline(always)]
pub(crate) fn ceil(x: f32) -> f32 {
    #[cfg(feature = "libm")]
    return libm::ceilf(x);
    #[cfg(all(not(feature = "libm"), feature = "std"))]
    return f32::ceil(x);
    #[cfg(all(not(feature = "libm"), not(feature = "std")))]
    unimplemented!("`no_std` builds require the `libm` feature");
}
