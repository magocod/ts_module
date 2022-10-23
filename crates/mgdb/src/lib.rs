pub mod db;
pub mod populate_v1;
pub mod populate_v2;
pub mod populate_v3;
pub mod populate_v4;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
