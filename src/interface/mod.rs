use std::{ptr, ffi::CStr, net::{Ipv4Addr, Ipv6Addr}, mem};

use c_linked_list::CLinkedListMut;
use libc::getifaddrs;
use crate::socket::af::*;

use crate::interface::interface::{NetworkInterface, InterfaceAddress};

pub mod interface;
pub mod flags;

pub fn get_interfaces() -> Result<Vec<NetworkInterface>, ()> {
    let mut interfaces: Vec<NetworkInterface> = vec![];
    let mut interfaces_raw: *mut libc::ifaddrs = ptr::null_mut();

    if unsafe { getifaddrs(&mut interfaces_raw) != 0 } {
        return Err(());
    }

    let ifs_linked_list =
        unsafe { CLinkedListMut::from_ptr(interfaces_raw, |interface| interface.ifa_next) };

    for interface in ifs_linked_list.iter() {
        let if_name = unsafe {
            CStr::from_ptr(interface.ifa_name)
                .to_string_lossy()
                .to_string()
        };
        let mut rs_if = NetworkInterface {
            if_name,
            flags: interface.ifa_flags,
            address: None,
            netmask: None,
        };

        match unsafe { (*interface.ifa_addr).sa_family } {
            AF_INET => {
                let address: libc::sockaddr_in = unsafe { mem::transmute(*interface.ifa_addr) };
                let ip_addr = Ipv4Addr::from(u32::from_be(address.sin_addr.s_addr));

                rs_if.address = Some(InterfaceAddress::Ipv4(ip_addr));
            }

            AF_INET6 => {
                let address: libc::sockaddr_in6 = unsafe { mem::transmute_copy(&*interface.ifa_addr)};
                let ip_addr = Ipv6Addr::from(address.sin6_addr.s6_addr);

                rs_if.address = Some(InterfaceAddress::Ipv6(ip_addr));
            }
            _ => {}
        }

        interfaces.push(rs_if);
    }

    Ok(interfaces)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    pub fn test() {
        dbg!(get_interfaces().unwrap());
        let mut interfaces = get_interfaces().unwrap();
        assert!(interfaces.iter().map(|x| x.is_up()).last().unwrap());
        assert!(get_interfaces().is_ok());
    }
}
