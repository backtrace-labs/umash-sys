umash-sys: rust FFI bindings for UMASH
======================================

[![Build Status](https://travis-ci.com/backtrace-labs/umash-sys.svg?branch=main)](https://travis-ci.com/backtrace-labs/umash-sys) [![crates.io](https://img.shields.io/crates/v/umash-sys.svg)](https://crates.io/crates/umash-sys)

UMASH is a family of fast hash / fingerprinting functions with
collision bounds.  This crate builds on x86-64 (with CLMUL) and
little-endian aarch64 (with VMULL).  The UMASH family of functions is
defined independently of hardware specific features, so computes the
same values on both architecture.

See https://github.com/backtrace-labs/umash for more details.
