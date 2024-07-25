//! Bindings for [API levels]
//!
//! Defines functions and constants for working with Android API levels.
//!
//! [API levels]: https://developer.android.com/ndk/reference/group/apilevels

use num_enum::{FromPrimitive, IntoPrimitive};
use thiserror::Error;

/// Android API levels, equivalent to the constants defined in `<android/api-level.h>` and the Java
/// [`Build.VERSION_CODES`] constants.
///
/// [`Build.VERSION_CODES`]: https://developer.android.com/reference/android/os/Build.VERSION_CODES
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, IntoPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum ApiLevel {
    /// Magic version number for an Android OS build which has not yet turned into an official
    /// release.
    #[doc(alias = "__ANDROID_API_FUTURE__")]
    Future = ffi::__ANDROID_API_FUTURE__,

    /// Names the Gingerbread API level (9)
    #[doc(alias = "__ANDROID_API_G__")]
    G = ffi::__ANDROID_API_G__,
    /// Names the Ice-Cream Sandwich API level (14)
    #[doc(alias = "__ANDROID_API_I__")]
    I = ffi::__ANDROID_API_I__,
    /// Names the Jellybean API level (16)
    #[doc(alias = "__ANDROID_API_J__")]
    J = ffi::__ANDROID_API_J__,
    /// Names the Jellybean MR1 API level (17)
    #[doc(alias = "__ANDROID_API_J_MR1__")]
    JMr1 = ffi::__ANDROID_API_J_MR1__,
    /// Names the Jellybean MR2 API level (18)
    #[doc(alias = "__ANDROID_API_J_MR2__")]
    JMr2 = ffi::__ANDROID_API_J_MR2__,
    /// Names the KitKat API level (19)
    #[doc(alias = "__ANDROID_API_K__")]
    K = ffi::__ANDROID_API_K__,
    /// Names the Lollipop API level (21)
    #[doc(alias = "__ANDROID_API_L__")]
    L = ffi::__ANDROID_API_L__,
    /// Names the Lollipop MR1 API level (22)
    #[doc(alias = "__ANDROID_API_L_MR1__")]
    LMr1 = ffi::__ANDROID_API_L_MR1__,
    /// Names the Marshmallow API level (23)
    #[doc(alias = "__ANDROID_API_M__")]
    M = ffi::__ANDROID_API_M__,
    /// Names the Nougat API level (24)
    #[doc(alias = "__ANDROID_API_N__")]
    N = ffi::__ANDROID_API_N__,
    /// Names the Nougat MR1 API level (25)
    #[doc(alias = "__ANDROID_API_N_MR1__")]
    NMr1 = ffi::__ANDROID_API_N_MR1__,
    /// Names the Oreo API level (26)
    #[doc(alias = "__ANDROID_API_O__")]
    O = ffi::__ANDROID_API_O__,
    /// Names the Oreo MR1 API level (27)
    #[doc(alias = "__ANDROID_API_O_MR1__")]
    OMr1 = ffi::__ANDROID_API_O_MR1__,
    /// Names the Pie API level (28)
    #[doc(alias = "__ANDROID_API_P__")]
    P = ffi::__ANDROID_API_P__,
    /// Names the Android 10 (aka "Q" or "Quince Tart") API level (29)
    #[doc(alias = "__ANDROID_API_Q__")]
    Q = ffi::__ANDROID_API_Q__,
    /// Names the Android 11 (aka "R" or "Red Velvet Cake") API level (30)
    #[doc(alias = "__ANDROID_API_R__")]
    R = ffi::__ANDROID_API_R__,
    /// Names the Android 12 (aka "S" or "Snowcone") API level (31)
    #[doc(alias = "__ANDROID_API_S__")]
    S = ffi::__ANDROID_API_S__,
    /// Names the Android 13 (aka "T" or "Tiramisu") API level (33)
    #[doc(alias = "__ANDROID_API_T__")]
    T = ffi::__ANDROID_API_T__,
    /// Names the Android 14 (aka "U" or "UpsideDownCake") API level (34)
    #[doc(alias = "__ANDROID_API_U__")]
    U = ffi::__ANDROID_API_U__,
    /// Names the Android 15 (aka "V" or "VanillaIceCream") API level (35)
    #[doc(alias = "__ANDROID_API_V__")]
    V = ffi::__ANDROID_API_V__,
    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(u32),
}

/// Returns the `targetSdkVersion` from `AndroidManifest.xml` of the caller, or [`ApiLevel::Future`]
/// if there is no known target SDK version (for code not running in the context of an app).
///
/// See also [`device_api_level()`].
#[cfg(feature = "api-level-24")]
pub fn application_target_sdk_version() -> ApiLevel {
    let version = unsafe { ffi::android_get_application_target_sdk_version() };
    u32::try_from(version)
        // Docs suggest that it would only return `Future`
        .expect("Unexpected sign bit in `application_target_sdk_version()`")
        .into()
}

/// Possible failures returned by [`device_api_level()`].
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DeviceApiLevelError {
    #[cfg(not(feature = "api-level-29"))]
    #[error(
        "`__system_property_get(\"ro.build.version.sdk\"` failed or string is empty, ret is {0}"
    )]
    FallbackPropertyGetFailed(i32),
    #[cfg(not(feature = "api-level-29"))]
    #[error("`ro.build.version.sdk` does not contain valid UTF-8")]
    FallbackUtf8Error(#[from] std::str::Utf8Error),
    #[cfg(not(feature = "api-level-29"))]
    #[error("`ro.build.version.sdk` does not contain an integer")]
    FallbackParseIntError(#[from] std::num::ParseIntError),
    #[error("device_api_level() encountered a negative version code")]
    TryFromIntError(#[from] std::num::TryFromIntError),
}

/// Returns the API level of the device we're actually running on.
///
/// The returned value is equivalent to the Java [`Build.VERSION.SDK_INT`] API.
///
/// [`Build.VERSION.SDK_INT`]: https://developer.android.com/reference/android/os/Build.VERSION#SDK_INT
///
/// Below `api-level-29` this falls back to reading the `"ro.build.version.sdk"` system property,
/// with the possibility to return more types of errors.
///
/// See also [`application_target_sdk_version()`].
pub fn device_api_level() -> Result<ApiLevel, DeviceApiLevelError> {
    #[cfg(not(feature = "api-level-29"))]
    let version = {
        use std::ffi::c_int;
        let mut value = [0; 92];
        let ret = unsafe {
            // from sys/cdefs.h, also available in the `libc` crate
            extern "C" {
                fn __system_property_get(__name: *const u8, __value: *mut u8) -> c_int;
            }
            // TODO: Switch to C-string literal since MSRV 1.77
            __system_property_get(b"ro.build.version.sdk\0".as_ptr(), value.as_mut_ptr())
        };
        if ret < 1 {
            return Err(DeviceApiLevelError::FallbackPropertyGetFailed(ret));
        }
        std::str::from_utf8(&value)?.parse::<i32>()?
    };

    #[cfg(feature = "api-level-29")]
    let version = unsafe { ffi::android_get_device_api_level() };

    Ok(u32::try_from(version)?.into())
}
