#![allow(dead_code)]

use packed_simd::{u32x4, u8x4};

#[inline]
pub extern "C" fn bsf16(data: i16) -> i16 {
    let mut ans: i16 = 0;
    unsafe {
        asm!("bsf %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}

#[inline]
pub extern "C" fn bsf32(data: i32) -> i32 {
    let mut ans: i32 = 0;
    unsafe {
        asm!("bsf %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}

#[inline]
pub extern "C" fn bsf64(data: i64) -> i64 {
    let mut ans: i64 = 0;
    unsafe {
        asm!("bsf %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}

#[inline]
pub extern "C" fn bsr32(data: i32) -> i32 {
    let mut ans: i32 = 0;
    unsafe {
        asm!("bsr %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}


#[inline]
pub extern "C" fn bsr64(data: i64) -> i64 {
    let mut ans: i64 = 0;
    unsafe {
        asm!("bsr %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}

#[inline]
pub extern "C" fn bsr16(data: i16) -> i16 {
    let mut ans: i16 = 0;
    unsafe {
        asm!("bsr %eax, %ebx" : "={ebx}"(ans) : "{eax}"(data));
    }
    ans
}

#[inline]
pub extern "C" fn byte_parity(b: u8) -> bool {
    let b = b as u64;
    (((b * 0x0101010101010101) & 0x8040201008040201) % 0x1FF) & 1 != 0
}

#[inline]
pub extern "C" fn pop_parity(mut b: u64) -> bool {
    b ^= b >> 1;
    b ^= b >> 2;
    b = (b & 0x1111111111111111) * 0x1111111111111111;
    (b >> 60) & 1 != 0
}

#[inline]
pub extern "C" fn pop_reverse8(b: u8) -> u8 {
    let b = b as u64;
    (((b * 0x80200802) & 0x0884422110) * 0x0101010101 >> 32) as u8
}



#[inline]
pub extern "C" fn pop_reverse(mut b: u64) -> u64 {
    let mut s = 32;
    let mut mask = 18446744073709551615;
    while s > 0 {
        mask ^= (mask << s);
        b = ((b >> s) & mask) | ((b << s) & !mask);
        s >>= 1;
    }
    b
}

#[inline]
pub extern "C" fn pop_count(v: u32) -> u32 {
    let mut c : u64 = 0;
    let v = v as u64;
    c =  ((v & 0xfff) * 0x1001001001001 & 0x84210842108421) % 0x1f;
    c += (((v & 0xfff000) >> 12) * 0x1001001001001 & 0x84210842108421) % 0x1f;
    (c + (((v >> 24) * 0x1001001001001 & 0x84210842108421) % 0x1f)) as u32
}



#[cfg(test)]
mod test {
    #[test]
    fn bsf32() {
        let a: i32 = 0b00010000;
        assert_eq!(super::bsf32(a), 4);
    }


    #[test]
    fn pop_count() {
        let a: u32 = 0b00000000;
        assert_eq!(super::pop_count(a), 0);
        let a: u32 = 0b00010000;
        assert_eq!(super::pop_count(a), 1);
        let a: u32 = 0b00011000;
        assert_eq!(super::pop_count(a),  2);
        let a: u32 = 0b10011000111101111;
        assert_eq!(super::pop_count(a),  11);
    }

    #[test]
    fn byte_parity() {
        assert!(super::byte_parity(1));
        assert!(!super::byte_parity(0b11));
        assert!(!super::byte_parity(0b0));
        assert!(!super::byte_parity(0b1100));
        assert!(!super::byte_parity(0b11));
    }

    #[test]
    fn pop_parity() {
        assert!(super::pop_parity(1));
        assert!(!super::pop_parity(0b11));
        assert!(!super::pop_parity(0b0));
        assert!(!super::pop_parity(0b110011111111));
        assert!(!super::pop_parity(0b11));
    }

    #[test]
    fn pop_reverse() {
        assert_eq!(super::pop_reverse8(0b11101101), 0b10110111);
        assert_eq!(super::pop_reverse(18446744073709551615), 18446744073709551615);
        assert_eq!(super::pop_reverse(0), 0);
        assert_eq!(super::pop_reverse(
            0b1100001110001111100010101001010101000010101010101010101010000001),
                   0b1000000101010101010101010100001010101001010100011111000111000011);
    }

}
