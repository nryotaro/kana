#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::copy_nonoverlapping;
use std::io::prelude::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern "C" fn helper(
	c: *mut _SMBCCTX,
	srv: *const c_char,
	shr: *const c_char,
	wg: *mut c_char,
	wglen: c_int,
	un: *mut c_char,
	unlen: c_int,
	pw: *mut c_char,
	pwlen: c_int,
) {
	unsafe {
		// srv -> 192.168.1.2
		/*
		println!("{}, {}", wglen, CStr::from_ptr(wg).to_str().unwrap());
		println!("{}, {}", unlen, CStr::from_ptr(un).to_str().unwrap());
		println!("{}, {}", pwlen, CStr::from_ptr(pw).to_str().unwrap());
		let c_str: &CStr = CStr::from_ptr(shr);
		let str_slice: &str = c_str.to_str().unwrap();
		println!("{}", str_slice);
		//let workgroup = CString::new("WORKGROUP").unwrap().into_raw();
		let workgroup = CString::new("WORKGROUP").unwrap();
		copy_nonoverlapping(workgroup.as_ptr(), wg, 9);
		let user = CString::new("guest").unwrap();
		copy_nonoverlapping(user.as_ptr(), un, 5);
		let pass = CString::new("").unwrap();
		copy_nonoverlapping(pass.as_ptr(), pw, 0);

		println!("{}, {}", wglen, CStr::from_ptr(wg).to_str().unwrap());
		println!("{}, {}", unlen, CStr::from_ptr(un).to_str().unwrap());
		println!("{}, {}", pwlen, CStr::from_ptr(pw).to_str().unwrap());
		*/
	}
}

pub fn temp() {
	unsafe {
		let _smbcctx: *mut _SMBCCTX = smbc_new_context();
		smbc_setDebug(_smbcctx, 1);
		smbc_setFunctionAuthDataWithContext(_smbcctx, Some(helper));
		if smbc_init_context(_smbcctx).is_null() {
			smbc_free_context(_smbcctx, 0);
			panic!("Could not initialize smbc context.");
		}
		smbc_set_context(_smbcctx);
		let path = CString::new(
			"smb://192.168.1.2/share/documents/manga/001.jpg",
		)
		.unwrap();
		let fd = smbc_open(path.as_ptr(), O_RDONLY as i32, 0);

		if fd < 0 {
			panic!("cannot open");
		}
		let dstlen = 80000000;
		let mut a = vec![0_u8; dstlen];
		let buffer: *mut c_void;
		buffer = a.as_mut_ptr() as *mut c_void;
		let ret = smbc_read(fd, buffer, dstlen as u64);
		println!("{}, ", ret);
		let mut c = File::create("doge.jpg").unwrap();
		c.write(&a[0..(ret as usize)]).unwrap();
		c.flush().unwrap();
	};
}

pub fn temp1() {
	unsafe {
		let _smbcctx: *mut _SMBCCTX = smbc_new_context();
		smbc_setDebug(_smbcctx, 1);
		smbc_setFunctionAuthDataWithContext(_smbcctx, Some(helper));
		//smbc_setOptionUseKerberos(_smbcctx, 1);
		//smbc_setOptionFallbackAfterKerberos(_smbcctx, 1);
		if smbc_init_context(_smbcctx).is_null() {
			smbc_free_context(_smbcctx, 0);
			panic!("Could not initialize smbc context.");
		}
		smbc_set_context(_smbcctx);
		let path =
			CString::new("smb://192.168.1.2/share/documents/manga/")
				.unwrap();
		let a: *const c_char = path.as_ptr();
		let fd = smbc_getFunctionOpendir(_smbcctx).unwrap()(_smbcctx, a);
		if fd.is_null() {
			panic!("failed to open");
		}
		let mut dirent = smbc_getFunctionReaddir(_smbcctx).unwrap()(_smbcctx, fd);
		while !dirent.is_null() {
			let c_str: &CStr = CStr::from_ptr(&(*dirent).name[0]);
			println!("{}", c_str.to_str().unwrap());
			dirent = smbc_getFunctionReaddir(_smbcctx).unwrap()(_smbcctx, fd);
		}

		/*
		let d = smbc_opendir(a);
		if d < 0 {
			panic!("failed to opendir");
		}
		let dirent = smbc_readdir(d as u32);
		if dirent.is_null() {
			panic!("null@@");
		}
		*/
	};
}

