mod algorithm;
mod context;

pub use algorithm::Algorithm;
pub use context::Context;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
