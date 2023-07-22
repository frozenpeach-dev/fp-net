use std::net::{Ipv4Addr, Ipv6Addr};

use crate::interface::flags;

#[derive(Debug)]
pub enum InterfaceAddress {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
}

#[derive(Debug)]
pub struct NetworkInterface {
    pub if_name: String,
    pub flags: u32,
    pub(crate) address: Option<InterfaceAddress>,
    pub netmask: Option<InterfaceAddress>,
}

impl NetworkInterface {

    pub fn is_up(&self) -> bool {
        self.flags & flags::IFF_UP != 0
    }

    pub fn is_loopback(&self) -> bool {
        self.flags & flags::IFF_LOOPBACK != 0
    }

    pub fn address(&self) -> Option<&InterfaceAddress> {
        self.address.as_ref()
    }

}
