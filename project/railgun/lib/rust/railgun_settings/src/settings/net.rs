//! Thin wrappers around [`std::net`] address types that are commonly
//! used within configuration files.
//!
//! The wrappers provide support for use within [`Settings`].
//!
//! [`Settings`]: super::Settings
use serde::{Deserialize, Serialize};
use std::net::ToSocketAddrs;
use std::ops::{Deref, DerefMut};
use std::option::IntoIter;

use super::Settings;

macro_rules! wrap_net_type {
    ($type:ident, default = $default:expr) => {
        /// A thin wrapper around
        #[doc = concat!("[`std::net::", stringify!($type), "`]")]
        /// that implements [`Settings`] and [`Default`] traits.
        ///
        /// [`Settings`]: super::Settings
        #[derive(Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
        pub struct $type(std::net::$type);

        impl Default for $type {
            fn default() -> Self {
                Self($default)
            }
        }

        impl From<std::net::$type> for $type {
            fn from(addr: std::net::$type) -> Self {
                Self(addr)
            }
        }

        impl From<$type> for std::net::$type {
            fn from(addr: $type) -> Self {
                addr.0
            }
        }

        impl std::fmt::Debug for $type {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.0, fmt)
            }
        }

        impl std::fmt::Display for $type {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, fmt)
            }
        }

        impl Deref for $type {
            type Target = std::net::$type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl PartialEq<std::net::$type> for $type {
            fn eq(&self, other: &std::net::$type) -> bool {
                self.0 == *other
            }
        }

        impl Settings for $type {}
    };
}

macro_rules! impl_to_socket_addrs {
    ($type:ident) => {
        impl ToSocketAddrs for $type {
            type Iter = IntoIter<std::net::SocketAddr>;

            fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
                self.0.to_socket_addrs()
            }
        }
    };
}

wrap_net_type!(
    SocketAddr,
    default = std::net::SocketAddr::new(std::net::Ipv4Addr::LOCALHOST.into(), 0)
);
wrap_net_type!(
    SocketAddrV4,
    default = std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 0)
);
wrap_net_type!(
    SocketAddrV6,
    default = std::net::SocketAddrV6::new(std::net::Ipv6Addr::LOCALHOST, 0, 0, 0)
);
wrap_net_type!(IpAddr, default = std::net::Ipv4Addr::LOCALHOST.into());
wrap_net_type!(Ipv4Addr, default = std::net::Ipv4Addr::LOCALHOST);
wrap_net_type!(Ipv6Addr, default = std::net::Ipv6Addr::LOCALHOST);

impl_to_socket_addrs!(SocketAddr);
impl_to_socket_addrs!(SocketAddrV4);
impl_to_socket_addrs!(SocketAddrV6);
