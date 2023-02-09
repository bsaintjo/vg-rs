use autocxx::prelude::*;

include_cpp! {
    #include "index_registry.hpp"

    safety!(unsafe)

    generate!("vg::AliasGraph")
    generate!("vg::IndexName")
}

pub use ffi::vg::AliasGraph;
pub use ffi::vg::IndexName;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
