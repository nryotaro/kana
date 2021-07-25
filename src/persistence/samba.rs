mod wrapper;
use crate::port::DocumentRepository;
use std::os::raw::{c_char, c_int, c_void};

pub struct SambaClient {
	smbcctx: *mut wrapper::_SMBCCTX,
	url: String,
}

impl DocumentRepository for SambaClient {
	fn new(url: &'static str) -> Option<Box<Self>> {
		let smbcctx = unsafe {
			let smbcctx = wrapper::smbc_new_context();
			if smbcctx.is_null() {
				()
			}
			wrapper::smbc_setFunctionAuthDataWithContext(smbcctx, Some(helper));
			if wrapper::smbc_init_context(smbcctx).is_null() {
				wrapper::smbc_free_context(smbcctx, 0);
				()
			}
			Some(smbcctx)
		};
		match smbcctx {
			Some(context) => Some(Box::new(SambaClient {
				smbcctx: context,
				url: url.to_string(),
			})),
			None => None,
		}
	}
}

extern "C" fn helper(
	c: *mut wrapper::_SMBCCTX,
	server: *const c_char,
	share: *const c_char,
	workgroup: *mut c_char,
	workgroup_length: c_int,
	username: *mut c_char,
	username_length: c_int,
	password: *mut c_char,
	password_length: c_int,
) {
}
