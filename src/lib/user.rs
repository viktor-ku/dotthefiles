#[derive(Debug)]
pub struct User {
  pub uid: u32,
  pub euid: u32,
  pub gid: u32,
}

impl User {
  pub fn new() -> Self {
    let uid = unsafe { libc::getuid() };
    let euid = unsafe { libc::geteuid() };
    let gid = unsafe { libc::getgid() };

    Self { uid, euid, gid }
  }
}
