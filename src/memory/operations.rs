#![allow(unused_imports)]

use simdeez::*;
use simdeez::avx2::*;
use simdeez::scalar::*;
use simdeez::sse2::*;
use simdeez::sse41::*;

use crate::types::*;
use crate::utils::*;

#[no_mangle]
pub unsafe extern "C" fn memcpy_simd_avx2(
dst: *mut u8,
src: *const u8,
size: size_t) -> *mut u8 {
    use core::arch::x86_64::*;
    let mask : size_t = 31;
    let preset = mask & size;
    for i in 0..preset {
            *dst.add(i as usize) = *src.add(i as usize);
    }
    for i in (preset..size).step_by(32) {
        let v = _mm256_lddqu_si256(& *(src.add(i as usize) as * const _));
        _mm256_storeu_si256(&mut *(dst.add(i as usize) as *mut _), v);
    }
    dst
}

#[no_mangle]
pub unsafe extern "C" fn memcpy_simd_sse(
    dst: *mut u8,
    src: *const u8,
    size: size_t) -> *mut u8 {
    use core::arch::x86_64::*;
    let mask : size_t = 31;
    let preset = mask & size;
    for i in 0..preset {
        *dst.add(i as usize) = *src.add(i as usize);
    }
    for i in (preset..size).step_by(32) {
        let v = _mm_lddqu_si128(& *(src.add(i as usize) as * const _));
        _mm_storeu_si128(&mut *(dst.add(i as usize) as *mut _), v);
    }
    dst
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn memcpy(dst: *mut u8,
                         src: *const u8,
                         size: size_t) -> *mut u8 {
    memcpy_simd_avx2(dst, src, size)
}


#[no_mangle]
pub unsafe extern "C" fn memmove_simd_avx2 (dst: *mut u8, src: *const u8, n: size_t) -> *mut u8 {
    if (dst as size_t) < (src as size_t) {
        memcpy_simd_avx2(dst, src, n)
    } else {
        let diff = dst as size_t - src as size_t;
        if diff > n {
            memcpy_simd_avx2(dst, src, n)
        } else {
            memcpy_simd_avx2(dst.add(diff as usize), dst, n - diff);
            memcpy_simd_avx2(dst, src, diff)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn memmove_simd_sse (dst: *mut u8, src: *const u8, n: size_t) -> *mut u8 {
    if (dst as size_t) < (src as size_t) {
        memcpy_simd_sse(dst, src, n)
    } else {
        let diff = dst as size_t - src as size_t;
        if diff > n {
            memcpy_simd_sse(dst, src, n)
        } else {
            memcpy_simd_sse(dst.add(diff as usize), dst, n - diff);
            memcpy_simd_sse(dst, src, diff)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dst: *mut u8,
                                 src: *const u8,
                                 size: size_t) -> *mut u8 {
    memmove_simd_avx2(dst, src, size)
}

#[no_mangle]
pub extern "C" fn memchr_simd_avx2(s: *const char_t, c: int_t, n: size_t) -> *const char_t {
    unsafe {
        use core::arch::x86_64::*;
        let mut i = 0;
        while i + 32 < n {
            let q = _mm256_set1_epi8(c as i8);
            let x = _mm256_lddqu_si256(s.add(i as usize) as _);
            let r = _mm256_cmpeq_epi8(q, x);
            let z = _mm256_movemask_epi8(r);
            if z != 0 {
                return s.add(bsf32(z) as usize + i as usize);
            }
            i += 32;
        }
        for j in i..n {
            if *s.add(j as usize) == (c as i8) {
                return s.add(j as usize);
            }
        }
        0 as _
    }
}

#[no_mangle]
pub extern "C" fn memchr_simd_sse(s: *const char_t, c: int_t, n: size_t) -> *const char_t {
    unsafe {
        use core::arch::x86_64::*;
        let mut i = 0;
        while i + 16 < n {
            let q = _mm_set1_epi8(c as i8);
            let x = _mm_lddqu_si128(s.add(i as usize) as _);
            let r = _mm_cmpeq_epi8(q, x);
            let z = _mm_movemask_epi8(r);
            if z != 0 {
                return s.add(bsf32(z) as usize + i as usize);
            }
            i += 16;
        }
        for j in i..n {
            if *s.add(j as usize) == (c as i8) {
                return s.add(j as usize);
            }
        }
        0 as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn memchr(s: *const char_t, c: int_t, n: size_t) -> *const char_t {
    if n < 32 { memchr_simd_sse(s, c, n) } else { memchr_simd_avx2(s, c, n) }
}

#[inline]
pub unsafe fn short_cmp(s: *const char_t, t: *const char_t, n: size_t) -> i32 {
    for i in 0..n {
        let res = *s.add(i as usize) - *t.add(i as usize);
        if res != 0 {
            return res as _;
        }
    }
    return 0;
}

#[no_mangle]
pub unsafe fn memcmp_simd_avx2(s: *const char_t, t: *const char_t, n: size_t) -> i32 {
    use core::arch::x86_64::*;
    static MASK: size_t = 31;
    let preset = MASK & n;
    let cmp = short_cmp(s, t, preset);
    if cmp != 0 {
        return cmp;
    }
    for i in (preset..n).step_by(32) {
        let a = _mm256_lddqu_si256(s as _);
        let b = _mm256_lddqu_si256(t as _);
        let c = _mm256_cmpeq_epi8(a, b);
        let res = !_mm256_movemask_epi8(c);
        if res != 0 {
            let offset = i as usize + bsf32(res) as usize;
            return (*s.add(offset as usize) - *t.add(offset as usize)) as _;
        }
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s: *const char_t, t: *const char_t, n: size_t) -> i32 {
    memcmp_simd_avx2(s, t, n)
}

#[cfg(test)]
mod test {
    use crate::size_t;

    #[test]
    fn memcpy() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        for i in 0..1000 {
            a.push(i);
        }
        b.resize(a.len(), 0);
        unsafe {
            super::memcpy(b.as_mut_ptr() as *mut u8, a.as_mut_ptr() as *mut u8, 4 * a.len() as size_t);
        }
        assert_eq!(a, b)
    }

    #[test]
    fn memmove() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        a.resize(1500, 0);
        b.resize(1500, 0);
        for i in 0..1000 {
            b[i] = i as i32;
            a[i] = i as i32;
        }
        for i in 500..1500 {
            b[i] = i as i32 - 500;
        }
        unsafe {
            super::memmove(a.as_mut_ptr().add(500) as *mut u8,
                           a.as_mut_ptr() as *mut u8, 4 * 1000);
        }
        assert_eq!(a, b)
    }

    #[test]
    fn memchr() {
        unsafe {
            let string = b"1234123412341234";
            let res = super::memchr(string.as_ptr() as *const i8, 0, string.len() as u64);
            println!("{}", res as isize - string.as_ptr() as isize);
        }
    }

    #[test]
    fn memcmp() {
        unsafe {
            let a = b"12345555555555555555565555555";
            let b = b"12345555555555555555555555555";
            assert!(super::memcmp(a.as_ptr() as _, b.as_ptr() as _, a.len() as _) > 0);
            assert!(super::memcmp(b.as_ptr() as _, a.as_ptr() as _, a.len() as _) < 0);
            assert_eq!(super::memcmp(a.as_ptr() as _, a.as_ptr() as _, a.len() as _), 0);
        }
    }
}


