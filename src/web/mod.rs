//! This module provides various middlewares/handlers for popular web framework to
//! help implementing write/read procotols of remote storage.

#[cfg(feature = "actix")]
pub mod actix;
#[cfg(feature = "warp")]
pub mod warp;
