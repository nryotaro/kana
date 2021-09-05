mod wrapper;
use crate::port::DocumentRepository;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

pub struct SambaClient {
	smbcctx: *mut wrapper::_SMBCCTX,
	url: String,
}

impl DocumentRepository for SambaClient {
	fn new(url: &str) -> Option<Box<Self>> {
		let path = CString::new(url).unwrap();
		let smbcctx = unsafe {
			let smbcctx: *mut wrapper::_SMBCCTX = create_smbcctx().unwrap();
			let fd = wrapper::smbc_getFunctionOpendir(smbcctx).unwrap()(smbcctx, path.as_ptr());
			if fd.is_null() {
				delete_smbcctx(smbcctx);
				return None;
			} else {
				wrapper::smbc_getFunctionClose(smbcctx).unwrap()(smbcctx, fd);
			}
			smbcctx
		};

		Some(Box::new(SambaClient {
			smbcctx: smbcctx,
			url: url.to_string(),
		}))
	}
	fn close(&self) {
		unsafe {
			delete_smbcctx(self.smbcctx);
		}
	}
}

unsafe fn create_smbcctx() -> Option<*mut wrapper::_SMBCCTX> {
	let smbcctx = wrapper::smbc_new_context();
	if smbcctx.is_null() {
		return None;
	}
	wrapper::smbc_setFunctionAuthData(smbcctx, Some(helper));
	if wrapper::smbc_init_context(smbcctx).is_null() {
		wrapper::smbc_free_context(smbcctx, 1);
		return None;
	}
	Some(smbcctx)
}

unsafe extern "C" fn delete_smbcctx(smbcctx: *mut wrapper::_SMBCCTX) {
	wrapper::smbc_getFunctionPurgeCachedServers(smbcctx).unwrap()(smbcctx);
	wrapper::smbc_free_context(smbcctx, 1);
}

unsafe extern "C" fn helper(
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
