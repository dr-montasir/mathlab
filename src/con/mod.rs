/// ### E
///
/// Mathematical constant
///
/// The Number e (Euler's number)
///
/// ### Example
/// ```rust
/// use mathlab::math::E;
/// assert_eq!(E, 2.718281828459045);
/// ```
pub const E: f64 = 2.718281828459045;

/// ### PI
///
/// Mathematical constant
///
/// The Number Pi
///
/// (21.991148575128552 / 7) = 3.141592653589793
///
/// ### Example
/// ```rust
/// use mathlab::math::PI;
/// assert_eq!(PI, 3.141592653589793);
/// ```
pub const PI: f64 = 3.141592653589793;

/// ### PHI
///
/// Mathematical constant
///
/// The Golden Ratio (Phi)
///
/// (1 + sqrt(5)) / 2 = 1.618033988749895
///
/// ### Example
/// ```rust
/// use mathlab::math::PHI;
/// assert_eq!(PHI, 1.618033988749895);
/// ```
pub const PHI: f64 = 1.618033988749895;

/// ### TAU
///
/// Mathematical constant
///
/// Tau is a circle constant and the value is equivalent to 2π
///
/// (2 * PI) = 6.283185307179586
///
/// ### Example
/// ```rust
/// use mathlab::math::TAU;
/// assert_eq!(TAU, 6.283185307179586);
/// ```
pub const TAU: f64 = 6.283185307179586;

/// ### LN2
///
/// Mathematical constant
///
/// The natural logarithm of 2
///
/// ### Example
/// ```rust
/// use mathlab::math::LN2;
/// assert_eq!(LN2, 0.693147180559945);
/// ```
pub const LN2: f64 = 0.693147180559945;

/// ### LN10
///
/// Mathematical constant
///
/// The natural logarithm of 10
///
/// ### Example
/// ```rust
/// use mathlab::math::LN10;
/// assert_eq!(LN10, 2.302585092994046);
/// ```
pub const LN10: f64 = 2.302585092994046;

/// ### LOG2E
///
/// Mathematical constant
///
/// The base 2 logarithm of E
///
/// ### Example
/// ```rust
/// use mathlab::math::LOG2E;
/// assert_eq!(LOG2E, 1.442695040888963);
/// ```
pub const LOG2E: f64 = 1.442695040888963;

/// ### LOG10E
///
/// Mathematical constant
///
/// The base 10 logarithm of E
///
/// ### Example
/// ```rust
/// use mathlab::math::LOG10E;
/// assert_eq!(LOG10E, 0.434294481903252);
/// ```
pub const LOG10E: f64 = 0.434294481903252;

/// ### NAN_F32
///
/// IEEE 754 Standard
///
/// The `NAN_F32` constant conforms to the `IEEE 754` specification for `single-precision` floating-point
/// representation of `NaN`, denoting undefined outcomes encountered during particular operations.
///
/// ### Example
/// ```rust
/// use mathlab::math::{NAN_F64, is_nan_f32, f64_to_f32, add};
/// assert!(is_nan_f32(add(NAN_F64, 1.0) as f32));
/// assert!(is_nan_f32(f64_to_f32(add(NAN_F64, 1.0))));
/// assert_eq!(assert!(is_nan_f32(add(NAN_F64, 1.0) as f32)), assert!(is_nan_f32(f64_to_f32(add(NAN_F64, 1.0)))));
/// ```
pub const NAN_F32: f32 = 0.0_f32 / 0.0_f32;

/// ### INF_F64
///
/// IEEE 754 Standard
///
/// The `INF_F32` constant conforms to the `IEEE 754` guideline governing `single-precision` floating-point `positive infinity` depiction.
///
/// ### Example
/// ```rust
/// use mathlab::math::INF_F32;
/// assert_eq!(2.0 / 0.0, INF_F32);
/// ```
pub const INF_F32: f32 = 1.0_f32 / 0.0_f32;

/// ### NINF_F32
///
/// IEEE 754 Standard
///
/// The `NINF_F32` constant conforms to the `IEEE 754` guideline governing `single-precision` floating-point `negative infinity` depiction.
///
/// ### Example
/// ```rust
/// use mathlab::math::NINF_F32;
/// assert_eq!(-2.0 / 0.0, NINF_F32);
/// ```
pub const NINF_F32: f32 = -1.0_f32 / 0.0_f32;

/// ### NAN_F64
///
/// IEEE 754 Standard
///
/// The `NAN_F64` constant conforms to the `IEEE 754` specification for `double-precision` floating-point
/// representation of `NaN`, denoting undefined outcomes encountered during particular operations.
///
/// ### Example
/// ```rust
/// use mathlab::math::{NAN_F64, is_nan_f64, add};
/// assert!(is_nan_f64(add(NAN_F64, 1.0)));
/// ```
pub const NAN_F64: f64 = 0.0_f64 / 0.0_f64;

/// ### INF_F64
///
/// IEEE 754 Standard
///
/// The `INF_F64` constant conforms to the `IEEE 754` guideline governing `double-precision` floating-point `positive infinity` depiction.
///
/// ### Example
/// ```rust
/// use mathlab::math::INF_F64;
/// assert_eq!(2.0 / 0.0, INF_F64);
/// ```
pub const INF_F64: f64 = 1.0_f64 / 0.0_f64;

/// ### NINF_F64
///
/// IEEE 754 Standard
///
/// The `NINF_F64` constant conforms to the `IEEE 754` guideline governing double-precision floating-point `negative infinity` depiction.
///
/// ### Example
/// ```rust
/// use mathlab::math::NINF_F64;
/// assert_eq!(-2.0 / 0.0, NINF_F64);
/// ```
pub const NINF_F64: f64 = -1.0_f64 / 0.0_f64;
