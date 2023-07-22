macro_rules! definef {
    ($name:tt, $lsh:tt) => {
       pub const $name: u32 = 1 << $lsh;
    };
}

definef!(IFF_UP, 0);
definef!(IFF_BROADCAST, 1);
definef!(IFF_DEBUG, 2);
definef!(IFF_LOOPBACK, 3);
definef!(IFF_MULTICAST, 12);
