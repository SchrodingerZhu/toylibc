#![allow(dead_code)]
#[inline]
pub extern "C" fn bsf16(data: i16) -> i16 {
    let mut ans : i16 = 0;
    unsafe {
        asm!("bsf %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}

#[inline]
pub extern "C" fn bsf32(data: i32) -> i32 {
    let mut ans : i32 = 0;
    unsafe {
        asm!("bsf %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}

#[inline]
pub extern "C" fn bsf64(data: i64) -> i64 {
    let mut ans : i64 = 0;
    unsafe {
        asm!("bsf %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}

#[inline]
pub extern "C" fn bsr32(data: i32) -> i32 {
    let mut ans : i32 = 0;
    unsafe {
        asm!("bsr %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}


#[inline]
pub extern "C" fn bsr64(data: i64) -> i64 {
    let mut ans : i64 = 0;
    unsafe {
        asm!("bsr %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}

#[inline]
pub extern "C" fn bsr16(data: i16) -> i16 {
    let mut ans : i16 = 0;
    unsafe {
        asm!("bsr %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}

#[cfg(test)]
mod test {
    #[test]
    fn bit_scan() {
        let a : i32 = 0b00010000;
        assert_eq!(super::bsf32(a), 4);
    }
}
