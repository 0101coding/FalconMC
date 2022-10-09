use thiserror::Error;

mod macros;

packet_modules! {
    extern pub mod v1_8_9;
    extern pub mod v1_12_2;
    extern pub mod v1_9;
}

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("The player could not be found")]
    PlayerNotFound,
}
