mod wrapper;
use crate::port::DocumentRepository;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

pub struct SambaClient {
	smbcctx: *mut wrapper::_SMBCCTX,
	url: String,
}

impl DocumentRepository for SambaClient {
	fn new(url: &'static str) -> Option<Box<Self>> {
		let smbcctx: *mut wrapper::_SMBCCTX = unsafe {
			let smbcctx = wrapper::smbc_new_context();
			if smbcctx.is_null() {
				return None;
			}
			wrapper::smbc_setFunctionAuthDataWithContext(smbcctx, Some(helper));
			if wrapper::smbc_init_context(smbcctx).is_null() {
				wrapper::smbc_free_context(smbcctx, 0);
				return None;
			}
			// いらないかも
			wrapper::smbc_set_context(smbcctx);
			smbcctx
		};
		unsafe {
			let path = CString::new(url).unwrap();
			let fd = wrapper::smbc_getFunctionOpendir(smbcctx).unwrap()(smbcctx, path.as_ptr());
			println!("doge");
			if fd.is_null() {
				close_smbcctx(smbcctx);
				return None;
			}
		};

		Some(Box::new(SambaClient {
			smbcctx: smbcctx,
			url: url.to_string(),
		}))
	}
}

extern "C" fn close_smbcctx(smbcctx: *mut wrapper::_SMBCCTX) {
	unsafe {
		wrapper::smbc_getFunctionPurgeCachedServers(smbcctx).unwrap()(smbcctx);
		wrapper::smbc_free_context(smbcctx, 1);
	}
}

extern "C" fn helper(
	_context: *mut wrapper::_SMBCCTX,
	_server: *const c_char,
	_share: *const c_char,
	_workgroup: *mut c_char,
	_workgroup_length: c_int,
	_username: *mut c_char,
	_username_length: c_int,
	_password: *mut c_char,
	_password_length: c_int,
) {
}
