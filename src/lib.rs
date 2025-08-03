pub mod anndata;
pub mod brute_force;
pub mod error;
pub mod layers;
pub mod obs;
pub mod obsm;
pub mod var;
pub mod varm;
pub mod x_array;
pub fn add(left: u64, right: u64) -> u64 {
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
