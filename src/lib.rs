#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate doc_comment;

#[macro_use]
extern crate paste;

pub use hyprland_macros::*;

/// This module provides shared things throughout the crate
pub mod shared;

/// This module provides functions for getting information on the compositor
#[cfg(feature = "data")]
pub mod data;

/// This module provides the EventListener struct for listening and acting upon for events
#[cfg(feature = "listener")]
pub mod event_listener;

/// This module is for calling dispatchers and changing keywords
#[cfg(feature = "dispatch")]
pub mod dispatch;

/// This module is for calling hyprctl **commands**, for getting data use [data]
#[cfg(feature = "ctl")]
pub mod ctl;

/// This module provides the stuff needed to mutate, and read Hyprland config values
#[cfg(feature = "keyword")]
pub mod keyword;

/// This module provides helpers to easily config Hyprland
#[cfg(feature = "config")]
pub mod config;

/// The prelude module, this is to import all traits
pub mod prelude {
    pub use crate::shared::{HyprData, HyprDataActive, HyprDataActiveOptional, HyprDataVec};
    pub use hyprland_macros::async_closure;
}

pub(crate) mod unix_async {
    #[cfg(feature = "async-net")]
    pub use async_net::unix::UnixStream;
    #[cfg(feature = "async-std")]
    pub use async_std::{
        io::{ReadExt, WriteExt},
        os::unix::net::UnixStream,
    };
    #[cfg(feature = "async-net")]
    pub use futures_lite::io::{AsyncReadExt, AsyncWriteExt};
    #[cfg(feature = "tokio")]
    pub use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::UnixStream,
    };
}

/// This type provides the result type used everywhere in Hyprland-rs
pub type Result<T> = std::result::Result<T, shared::HyprError>;
