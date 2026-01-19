pub mod config;
pub mod mpesa;
pub mod error;

pub use config::{Config, Environment};
pub use mpesa::Mpesa;
pub use error::MpesaError;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
