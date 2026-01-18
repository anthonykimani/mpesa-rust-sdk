mod config;
mod mpesa;
mod error;

pub use config::Config;
pub use mpesa::Mpesa;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
