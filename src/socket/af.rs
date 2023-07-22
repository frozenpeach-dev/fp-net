macro_rules! define_af {
    ($name:tt, $value:tt) => {
        pub const $name: u8 = $value;
    };
}

define_af!(AF_UNSPEC, 0);
define_af!(AF_UNIX, 1);
define_af!(AF_INET, 2);

define_af!(AF_INET6, 24);
