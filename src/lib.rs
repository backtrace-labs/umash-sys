/* Simplified version of output generated by rust-bindgen 0.57.0 */
#![allow(warnings)]

#[doc = " A single UMASH params struct stores the parameters for a pair of"]
#[doc = " independent `UMASH` functions."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct umash_params {
    pub poly: [[u64; 2usize]; 2usize],
    pub oh: [u64; 34usize],
}
#[test]
fn bindgen_test_layout_umash_params() {
    assert_eq!(
        ::std::mem::size_of::<umash_params>(),
        304usize,
        concat!("Size of: ", stringify!(umash_params))
    );
    assert_eq!(
        ::std::mem::align_of::<umash_params>(),
        8usize,
        concat!("Alignment of ", stringify!(umash_params))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_params>())).poly as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_params),
            "::",
            stringify!(poly)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_params>())).oh as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_params),
            "::",
            stringify!(oh)
        )
    );
}
#[doc = " A fingerprint consists of two independent `UMASH` hash values."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct umash_fp {
    pub hash: [u64; 2usize],
}
#[test]
fn bindgen_test_layout_umash_fp() {
    assert_eq!(
        ::std::mem::size_of::<umash_fp>(),
        16usize,
        concat!("Size of: ", stringify!(umash_fp))
    );
    assert_eq!(
        ::std::mem::align_of::<umash_fp>(),
        8usize,
        concat!("Alignment of ", stringify!(umash_fp))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_fp>())).hash as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_fp),
            "::",
            stringify!(hash)
        )
    );
}
#[doc = " This struct holds the state for incremental UMASH hashing or"]
#[doc = " fingerprinting."]
#[doc = ""]
#[doc = " A sink owns no allocation, and simply borrows a pointer to its"]
#[doc = " `umash_params`.  It can be byte-copied to snapshot its state."]
#[doc = ""]
#[doc = " The layout works best with alignment to 64 bytes, but does not"]
#[doc = " require it."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct umash_sink {
    pub poly_state: [umash_sink__bindgen_ty_1; 2usize],
    pub buf: [::std::os::raw::c_char; 32usize],
    pub oh: *const u64,
    pub oh_iter: u32,
    pub bufsz: u8,
    pub block_size: u8,
    pub large_umash: bool,
    pub hash_wanted: u8,
    pub oh_acc: umash_sink_umash_oh,
    pub oh_twisted: umash_sink_umash_twisted_oh,
    pub seed: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct umash_sink__bindgen_ty_1 {
    pub mul: [u64; 2usize],
    pub acc: u64,
}
#[test]
fn bindgen_test_layout_umash_sink__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<umash_sink__bindgen_ty_1>(),
        24usize,
        concat!("Size of: ", stringify!(umash_sink__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<umash_sink__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(umash_sink__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink__bindgen_ty_1>())).mul as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink__bindgen_ty_1),
            "::",
            stringify!(mul)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink__bindgen_ty_1>())).acc as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink__bindgen_ty_1),
            "::",
            stringify!(acc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct umash_sink_umash_oh {
    pub bits: [u64; 2usize],
}
#[test]
fn bindgen_test_layout_umash_sink_umash_oh() {
    assert_eq!(
        ::std::mem::size_of::<umash_sink_umash_oh>(),
        16usize,
        concat!("Size of: ", stringify!(umash_sink_umash_oh))
    );
    assert_eq!(
        ::std::mem::align_of::<umash_sink_umash_oh>(),
        8usize,
        concat!("Alignment of ", stringify!(umash_sink_umash_oh))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink_umash_oh>())).bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink_umash_oh),
            "::",
            stringify!(bits)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct umash_sink_umash_twisted_oh {
    pub lrc: [u64; 2usize],
    pub prev: [u64; 2usize],
    pub acc: umash_sink_umash_oh,
}
#[test]
fn bindgen_test_layout_umash_sink_umash_twisted_oh() {
    assert_eq!(
        ::std::mem::size_of::<umash_sink_umash_twisted_oh>(),
        48usize,
        concat!("Size of: ", stringify!(umash_sink_umash_twisted_oh))
    );
    assert_eq!(
        ::std::mem::align_of::<umash_sink_umash_twisted_oh>(),
        8usize,
        concat!("Alignment of ", stringify!(umash_sink_umash_twisted_oh))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink_umash_twisted_oh>())).lrc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink_umash_twisted_oh),
            "::",
            stringify!(lrc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<umash_sink_umash_twisted_oh>())).prev as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink_umash_twisted_oh),
            "::",
            stringify!(prev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink_umash_twisted_oh>())).acc as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink_umash_twisted_oh),
            "::",
            stringify!(acc)
        )
    );
}
#[test]
fn bindgen_test_layout_umash_sink() {
    assert_eq!(
        ::std::mem::size_of::<umash_sink>(),
        168usize,
        concat!("Size of: ", stringify!(umash_sink))
    );
    assert_eq!(
        ::std::mem::align_of::<umash_sink>(),
        8usize,
        concat!("Alignment of ", stringify!(umash_sink))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink>())).poly_state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink),
            "::",
            stringify!(poly_state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink>())).buf as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink>())).oh as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink),
            "::",
            stringify!(oh)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink>())).oh_iter as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink),
            "::",
            stringify!(oh_iter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink>())).bufsz as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink),
            "::",
            stringify!(bufsz)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink>())).block_size as *const _ as usize },
        93usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink),
            "::",
            stringify!(block_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink>())).large_umash as *const _ as usize },
        94usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink),
            "::",
            stringify!(large_umash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink>())).hash_wanted as *const _ as usize },
        95usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink),
            "::",
            stringify!(hash_wanted)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink>())).oh_acc as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink),
            "::",
            stringify!(oh_acc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink>())).oh_twisted as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink),
            "::",
            stringify!(oh_twisted)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_sink>())).seed as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_sink),
            "::",
            stringify!(seed)
        )
    );
}
#[doc = " The `umash_state` struct wraps a sink in a type-safe interface: we"]
#[doc = " don't want to try and extract a fingerprint from a sink configured"]
#[doc = " for hashing."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct umash_state {
    pub sink: umash_sink,
}
#[test]
fn bindgen_test_layout_umash_state() {
    assert_eq!(
        ::std::mem::size_of::<umash_state>(),
        168usize,
        concat!("Size of: ", stringify!(umash_state))
    );
    assert_eq!(
        ::std::mem::align_of::<umash_state>(),
        8usize,
        concat!("Alignment of ", stringify!(umash_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_state>())).sink as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_state),
            "::",
            stringify!(sink)
        )
    );
}
#[doc = " Similarly, the `umash_fp_state` struct wraps a sink from which we"]
#[doc = " should extract a fingerprint."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct umash_fp_state {
    pub sink: umash_sink,
}
#[test]
fn bindgen_test_layout_umash_fp_state() {
    assert_eq!(
        ::std::mem::size_of::<umash_fp_state>(),
        168usize,
        concat!("Size of: ", stringify!(umash_fp_state))
    );
    assert_eq!(
        ::std::mem::align_of::<umash_fp_state>(),
        8usize,
        concat!("Alignment of ", stringify!(umash_fp_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<umash_fp_state>())).sink as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(umash_fp_state),
            "::",
            stringify!(sink)
        )
    );
}
extern "C" {
    #[doc = " Converts a `umash_params` struct filled with random values into"]
    #[doc = " something usable by the UMASH functions below."]
    #[doc = ""]
    #[doc = " When it succeeds, this function is idempotent.  Failure happens"]
    #[doc = " with probability < 2**-110 is `params` is filled with uniformly"]
    #[doc = " distributed random bits.  That's an astronomically unlikely event,"]
    #[doc = " and most likely signals an issue with the caller's (pseudo-)random"]
    #[doc = " number generator."]
    #[doc = ""]
    #[doc = " @return false on failure, probably because the input was not random."]
    pub fn umash_params_prepare(params: *mut umash_params) -> bool;
}
extern "C" {
    #[doc = " Deterministically derives a `umash_params` struct from `bits` and"]
    #[doc = " `key`.  The `bits` values do not have to be particularly well"]
    #[doc = " distributed, and can be generated sequentially."]
    #[doc = ""]
    #[doc = " @param key a pointer to exactly 32 secret bytes.  NULL will be"]
    #[doc = "   replaced with \"Do not use UMASH VS adversaries.\", the default"]
    #[doc = "   UMASH secret."]
    pub fn umash_params_derive(
        arg1: *mut umash_params,
        bits: u64,
        key: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " Updates a `umash_sink` to take into account `data[0 ... n_bytes)`."]
    pub fn umash_sink_update(
        arg1: *mut umash_sink,
        data: *const ::std::os::raw::c_void,
        n_bytes: ::std::os::raw::c_ulong,
    );
}
extern "C" {
    #[doc = " Computes the UMASH hash of `data[0 ... n_bytes)`."]
    #[doc = ""]
    #[doc = " Randomly generated `param` lead to independent UMASH values and"]
    #[doc = " associated worst-case collision bounds; changing the `seed` comes"]
    #[doc = " with no guarantee."]
    #[doc = ""]
    #[doc = " @param which 0 to compute the first UMASH defined by `params`, 1"]
    #[doc = "   for the second."]
    pub fn umash_full(
        params: *const umash_params,
        seed: u64,
        which: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
        n_bytes: ::std::os::raw::c_ulong,
    ) -> u64;
}
extern "C" {
    #[doc = " Computes the UMASH fingerprint of `data[0 ... n_bytes)`."]
    #[doc = ""]
    #[doc = " Randomly generated `param` lead to independent UMASH values and"]
    #[doc = " associated worst-case collision bounds; changing the `seed` comes"]
    #[doc = " with no guarantee."]
    pub fn umash_fprint(
        params: *const umash_params,
        seed: u64,
        data: *const ::std::os::raw::c_void,
        n_bytes: ::std::os::raw::c_ulong,
    ) -> umash_fp;
}
extern "C" {
    #[doc = " Prepares a `umash_state` for computing the `which`th UMASH function in"]
    #[doc = " `params`."]
    pub fn umash_init(
        arg1: *mut umash_state,
        params: *const umash_params,
        seed: u64,
        which: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[doc = " Returns the UMASH value for the bytes that have been"]
    #[doc = " `umash_sink_update`d into the state."]
    pub fn umash_digest(arg1: *const umash_state) -> u64;
}
extern "C" {
    #[doc = " Prepares a `umash_fp_state` for computing the UMASH fingerprint in"]
    #[doc = " `params`."]
    pub fn umash_fp_init(arg1: *mut umash_fp_state, params: *const umash_params, seed: u64);
}
extern "C" {
    #[doc = " Returns the UMASH fingerprint for the bytes that have been"]
    #[doc = " `umash_sink_update`d into the state."]
    pub fn umash_fp_digest(arg1: *const umash_fp_state) -> umash_fp;
}

#[cfg(test)]
mod tests {
    use crate::{umash_fp, umash_fprint, umash_full, umash_params, umash_params_derive};
    use std::ffi::CString;
    use std::os::raw::c_ulong;
    use std::os::raw::c_void;

    #[test]
    fn test_example_case() {
        let mut key = CString::new("hello example.c").unwrap().into_bytes();
        key.resize(32, 0u8);

        let input = CString::new("the quick brown fox").unwrap();
        let seed = 42u64;

        let mut my_params: umash_params = unsafe { std::mem::zeroed() };
        unsafe { umash_params_derive(&mut my_params, 0, key.as_ptr() as *const c_void) };

        let fprint = unsafe {
            umash_fprint(
                &my_params,
                seed,
                input.as_bytes().as_ptr() as *const c_void,
                input.as_bytes().len() as c_ulong,
            )
        };
        assert_eq!(fprint.hash, [0x398c5bb5cc113d03, 0x3a52693519575aba]);

        let hash0 = unsafe {
            umash_full(
                &my_params,
                seed,
                0,
                input.as_bytes().as_ptr() as *const c_void,
                input.as_bytes().len() as c_ulong,
            )
        };
        assert_eq!(hash0, 0x398c5bb5cc113d03);

        let hash1 = unsafe {
            umash_full(
                &my_params,
                seed,
                1,
                input.as_bytes().as_ptr() as *const c_void,
                input.as_bytes().len() as c_ulong,
            )
        };
        assert_eq!(hash1, 0x3a52693519575aba);
    }
}
