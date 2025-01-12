#![warn(missing_docs)]

//! # raur
//!
//! raur is a library for interacting with the
//! [aurweb RPC Interface](https://aur.archlinux.org/rpc).
//!
//! See also the [Arch wiki page](https://aur.archlinux.org/rpc.php) for more information.
//!
//! # Example
//!
//! ```
//!
//! # #[cfg(not(feature = "async"))]
//! # fn main() {}
//! # #[cfg(feature = "async")]
//! # #[tokio::main]
//! # async fn main() -> Result<(), raur::Error> {
//! use raur::Raur;
//!
//! let raur = raur::Handle::new();
//!
//! // Use `search` to search using keywords (multiple strategies available)
//! let pkgs = raur.search("pacman").await?;
//! assert!(pkgs.len() > 10);
//!
//! for pkg in pkgs {
//!     println!("{:<30}{}", pkg.name, pkg.version);
//! }
//!
//! // Use `info` to get info about a list of packages. Not-found packages are silently ignored.
//! let pkgs = raur.info(&["spotify", "discord-ptb"]).await?;
//! assert_eq!(pkgs.len(), 2);
//!
//! for pkg in pkgs {
//!     println!("{:<30}{}", pkg.name, pkg.version);
//! }
//! # Ok(())
//! # }
//! ```

/// Non async handle
#[cfg(feature = "blocking-trait")]
pub mod blocking;
mod cache;
#[cfg(any(feature = "async", feature = "blocking"))]
mod error;
#[cfg(feature = "async")]
mod handle;
mod raur;
#[cfg(feature = "trait")]
mod traits;

pub use crate::cache::*;
#[cfg(any(feature = "async", feature = "blocking"))]
pub use crate::error::*;
#[cfg(feature = "async")]
pub use crate::handle::*;
pub use crate::raur::*;

#[cfg(feature = "trait")]
pub use crate::traits::*;
