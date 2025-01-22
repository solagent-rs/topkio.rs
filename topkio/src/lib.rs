mod agent;
mod primitives;
mod rpc;

pub use rpc::client::{Client, Provider};

/// export third lib to pub
pub use futures_util;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn ollama_works() {

//     }
// }
