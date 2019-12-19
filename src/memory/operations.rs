#![allow(unused_imports)]
use crate::types::*;
use simdeez::*;
use simdeez::scalar::*;
use simdeez::sse2::*;
use simdeez::sse41::*;
use simdeez::avx2::*;
use packed_simd::u32x4;

simd_compiletime_generate!(
fn memcpy_simd(
dst: *mut u8,
src: *const u8,
size: size_t) -> *mut u8 {
    let mask : size_t = (S::VI32_WIDTH << 2) as size_t - 1;
    let preset = mask & size;
    for i in 0..preset {
            *dst.add(i as usize) = *src.add(i as usize);
    }
    for i in (preset..size).step_by(S::VI32_WIDTH << 2) {
        let v = S::loadu_epi32(& *(src.add(i as usize) as *const i32));
        S::storeu_epi32(&mut *(dst.add(i as usize) as *mut i32), v);
    }
    dst
}
);

pub extern "C" fn memcpy (dst: *mut u8,
                      src: *const u8,
                      size: size_t) -> *mut u8 {
    memcpy_simd_compiletime(dst, src, size)
}


simd_compiletime_generate!(
fn memmove_simd (dst: *mut u8, src: *const u8, n: size_t) -> *mut u8 {
    if (dst as size_t) < (src as size_t) {
        memcpy_simd::<S>(dst, src, n)
    } else {
        let diff = dst as size_t - src as size_t;
        if diff > n {
            memcpy_simd::<S>(dst, src, n)
        } else {
            memcpy_simd::<S>(dst.add(diff as usize), dst, n - diff);
            memcpy_simd::<S>(dst, src, diff)
        }
    }
}
);

pub extern "C" fn memmove (dst: *mut u8,
                          src: *const u8,
                          size: size_t) -> *mut u8 {
    memmove_simd_compiletime(dst, src, size)
}

#[inline]
fn bit_scan(data: i32) -> i32 {
    let mut ans : i32 = 0;
    unsafe {
        asm!("bsf %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}



fn memchr_simd_avx2(s: *const char_t, c: int_t, n: size_t) -> *const char_t {
    unsafe {
        use core::arch::x86_64::*;
        let mut i = 0;
        while i + 32 < n {
            let q = _mm256_set1_epi8(c as i8);
            let x = _mm256_lddqu_si256(s.add(i as usize) as _);
            let r = _mm256_cmpeq_epi8(q, x);
            let z = _mm256_movemask_epi8(r);
            if z != 0 {
                return s.add(bit_scan(z) as usize + i as usize)
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


fn memchr_simd_sse(s: *const char_t, c: int_t, n: size_t) -> *const char_t {
    unsafe {
        use core::arch::x86_64::*;
        let mut i = 0;
        while i + 16 < n {
            let q = _mm_set1_epi8(c as i8);
            let x = _mm_lddqu_si128(s.add(i as usize) as _);
            let r = _mm_cmpeq_epi8(q, x);
            let z = _mm_movemask_epi8(r);
            if z != 0 {
                return s.add(bit_scan(z) as usize + i as usize)
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

pub extern "C" fn memchr (s: *const char_t, c: int_t, n: size_t) -> *const char_t {
    if n < 32 {memchr_simd_sse(s, c, n)}
    else  {memchr_simd_avx2(s, c, n)}
}

#[cfg(test)]
mod test {
    use crate::memory::operations::memcpy_simd;
    use crate::size_t;

    #[test]
    fn memcpy() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        for i in 0..1000 {
            a.push(i);
        }
        b.resize(a.len(), 0);
        super::memcpy(b.as_mut_ptr() as *mut u8, a.as_mut_ptr() as *mut u8, 4 * a.len() as size_t);
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
    fn bit_scan() {
        let mut a : i32 = 0b00010000;
        assert_eq!(super::bit_scan(a), 4);
    }

    #[test]
    fn memchr() {
        let mut a : i32 = 0b00010000;
        let string = b"123412341234123412341234123412341234";
        let res = super::memchr_simd_avx2(string.as_ptr() as *const i8, 0, string.len() as u64);
        println!("{}", res as isize - string.as_ptr() as isize);
    }
}


