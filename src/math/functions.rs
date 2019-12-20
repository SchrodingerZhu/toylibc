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

