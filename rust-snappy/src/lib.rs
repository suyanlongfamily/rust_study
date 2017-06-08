extern crate libc;
extern crate snappy_sys;

use libc::size_t;
use snappy_sys::*;

pub fn validate_compressed_buffer(src: &[u8]) -> bool {
    unsafe {
        snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t) == 0
    }
}

pub fn compress(src: &[u8]) -> Vec<u8> {
    unsafe {
        let srclen = src.len() as size_t;
        let psrc = src.as_ptr();

        let mut dstlen = snappy_max_compressed_length(srclen);
        let mut dst = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr();

        snappy_compress(psrc, srclen, pdst, &mut dstlen);
        dst.set_len(dstlen as usize);
        dst
    }
}

/// Uncompress 'src' into a newly allocated vector.
pub fn uncompress(src: &[u8]) -> Result<Vec<u8>, ()> {
    let mut out = Vec::new();
    uncompress_to(src, &mut out).map(|_| out)
}

/// Uncompress 'src' directly appended to 'dst' and return the number
/// of bytes produced. Return an error upon an invalid 'src'.
pub fn uncompress_to(src: &[u8], dst: &mut Vec<u8>) -> Result<usize, ()> {
    unsafe {
        let src_len = src.len() as size_t;
        let src_ptr = src.as_ptr();

        let dst_cur_len = dst.len();
        let mut dst_add_len: size_t = 0;
        snappy_uncompressed_length(src_ptr, src_len, &mut dst_add_len);

        // now make sure the vector is large enough
        dst.reserve(dst_add_len as usize);
        // try to uncompress
        let dst_ptr = dst[dst_cur_len..].as_mut_ptr();
        if snappy_uncompress(src_ptr, src_len, dst_ptr, &mut dst_add_len) == 0 {
            dst.set_len(dst_cur_len + dst_add_len as usize);
            Ok(dst_add_len as usize)
        } else {
            dst.set_len(dst_cur_len); // make sure not to leak partial output
            Err(()) // SNAPPY_INVALID_INPUT
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str;
    use super::*;

    #[test]
    fn valid() {
        let d = vec![0xde, 0xad, 0xd0, 0x0d];
        let c = compress(&d);
        assert!(validate_compressed_buffer(&c));
        assert!(uncompress(&c) == Ok(d));
    }

    #[test]
    fn invalid() {
        let d = vec![0, 0, 0, 0];
        assert!(!validate_compressed_buffer(&d));
        assert!(uncompress(&d).is_err());
    }

    #[test]
    fn empty() {
        let d = vec![];
        assert!(!validate_compressed_buffer(&d));
        assert!(uncompress(&d).is_err());

        let c = compress(&d);
        assert!(validate_compressed_buffer(&c));
        assert!(uncompress(&c) == Ok(d));
    }

    #[test]
    fn uncompress_to_appends() {
        // "This is test"
        let compressed = &[12, 44, 84, 104, 105, 115, 32, 105, 115, 32, 116, 101, 115, 116];

        let mut out = vec![b'a', b'b', b'c', b'>'];
        uncompress_to(compressed, &mut out).unwrap();
        let s = str::from_utf8(&out[..]).unwrap();
        assert_eq!(s, "abc>This is test");
    }
}
