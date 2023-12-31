pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod io_helper;
mod meta_structure;
mod json_handler;
mod crypto_handler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
