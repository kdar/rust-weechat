include!("weechat-plugin.rs");

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static weechat_plugin_api_version: [u8; 12] = *WEECHAT_PLUGIN_API_VERSION;

#[macro_export]
macro_rules! weechat_plugin_info(
  (name: $val:expr; $length:expr) => {
    #[no_mangle]
    #[allow(non_upper_case_globals)]
    pub static weechat_plugin_name: [u8; $length] = *$val;
  };

  (author: $val:expr; $length:expr) => {
    #[no_mangle]
    #[allow(non_upper_case_globals)]
    pub static weechat_plugin_author: [u8; $length] = *$val;
  };

  (description: $val:expr; $length:expr) => {
    #[no_mangle]
    #[allow(non_upper_case_globals)]
    pub static weechat_plugin_description: [u8; $length] = *$val;
  };

  (version: $val:expr; $length:expr) => {
    #[no_mangle]
    #[allow(non_upper_case_globals)]
    pub static weechat_plugin_version: [u8; $length] = *$val;
  };

  (license: $val:expr; $length:expr) => {
    #[no_mangle]
    #[allow(non_upper_case_globals)]
    pub static weechat_plugin_license: [u8; $length] = *$val;
  };

  (priority: $val:expr) => {
    #[no_mangle]
    #[allow(non_upper_case_globals)]
    pub static weechat_plugin_priority: ::std::os::raw::c_int = $priority;
  };
);

#[macro_export]
macro_rules! weechat_plugin(
  ($($name:ident: $val:expr; $length:expr),+) => {
    $(
      weechat_plugin_info!($name: $val; $length);
    )+
  };
  ($($name:ident: $val:expr; $length:expr),+,) => {
    weechat_plugin!($($name: $val; $length),+);
  };
);