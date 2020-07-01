use os_info::Type;

/// at some point we actually need to know where to find assets based on the
/// current OS. Note, that this is default locations.
pub fn os_type_to_string(os_type: &Type) -> &str {
  match os_type {
    Type::Linux => "linux",
    Type::Macos => "darwin",
    Type::Windows => "win",
    _ => panic!("do not know where to find sources for the given assets!"),
  }
}
