#[macro_use]
extern crate lazy_static;
#[warn(missing_docs)]
#[macro_use]
extern crate serde;

pub mod io;
pub mod models;
pub mod engine;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
