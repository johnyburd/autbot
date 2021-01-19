//! Contains generated code from the shared service Protobuf RPC definitions

pub mod uptime {
    mod generated {
        // Ignore clippy linting on generated code
        #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
        tonic::include_proto!("logs.uptime");
    }

    pub use generated::*;
}
