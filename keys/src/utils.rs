// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{self, Formatter};

pub fn fmt_byte_slice(slice: &[u8], fmt: &mut Formatter) -> fmt::Result {
    write!(fmt, "\"")?;

    for b in slice.iter() {
        write!(fmt, "{:02x}", b)?;
    }

    write!(fmt, "\"")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use std::fmt;

    #[test]
    fn fmt_byte_slice_() {
        struct Blob([u8; 32]);
        impl fmt::Debug for Blob {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt_byte_slice(&self.0, fmt)
            }
        }

        let blob = Blob([0u8; 32]);


        let blob_str = format!("{:?}", blob);

        assert_eq!(&*blob_str,
                   "\"0000000000000000000000000000000000000000000000000000000000000000\"");
    }
}
