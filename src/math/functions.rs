macro_rules! generate {
    ($fn_name:ident ($($arg:ident:$typ:ty),*) $(-> $rt:ty)?) =>
    {
        #[no_mangle]
        pub extern "C" fn $fn_name($($arg:$typ,)*) $(-> $rt)? {
            libm::$fn_name($($arg:$typ,)*)
        }
    }
}


generate!(acos(x : f64) -> f64);
generate!(acosf(x : f32) -> f32);
generate!(acosh(x : f64) -> f64);
generate!(acoshf(x : f32) -> f32);
generate!(asin(x : f64) -> f64);
generate!(asinf(x : f32) -> f32);
generate!(asinh(x : f64) -> f64);
generate!(asinhf(x : f32) -> f32);
generate!(atan(x : f64) -> f64);
generate!(atan2(x : f64, y : f64) -> f64);
generate!(atan2f(x : f32, y : f32) -> f32);
generate!(atanf(x : f32) -> f32);
generate!(atanh(x : f64) -> f64);
generate!(atanhf(x : f32) -> f32);
generate!(cbrt(x: f64) -> f64);
generate!(cbrtf(x: f32) -> f32);
generate!(ceil(x: f64) -> f64);
generate!(ceilf(x: f32) -> f32);
generate!(copysign(x: f64, y: f64) -> f64);
generate!(copysignf(x: f32, y: f32) -> f32);
generate!(cos(x: f64) -> f64);
generate!(cosf(x: f32) -> f32);
generate!(cosh(x: f64) -> f64);
generate!(coshf(x: f32) -> f32);
generate!(erf(x: f64) -> f64);
generate!(erfc(x: f64) -> f64);