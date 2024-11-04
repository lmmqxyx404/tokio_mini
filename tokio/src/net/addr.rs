// todo
pub trait ToSocketAddrs {}

// ===== impl str =====
impl ToSocketAddrs for str {}

// ===== impl String =====
impl ToSocketAddrs for String {}
