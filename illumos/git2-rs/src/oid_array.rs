//! Bindings to libgit2's raw git_strarray type

use std::ops::Deref;

use oid::Oid;
use raw;
use util::Binding;
use std::slice;
use std::mem;

/// An oid array structure used by libgit2
///
/// Some apis return arrays of oids which originate from libgit2. This
/// wrapper type behaves a little like `Vec<&Oid>` but does so without copying
/// the underlying Oids until necessary.
pub struct OidArray {
    raw: raw::git_oidarray,
}

impl Deref for OidArray {
    type Target = [Oid];

    fn deref(&self) -> &[Oid] {
        unsafe {
            debug_assert_eq!(mem::size_of::<Oid>(), mem::size_of_val(&*self.raw.ids));
            
            slice::from_raw_parts(self.raw.ids as *const Oid, self.raw.count as usize)
        }
    }
}

impl Binding for OidArray {
    type Raw = raw::git_oidarray;
    unsafe fn from_raw(raw: raw::git_oidarray) -> OidArray {
        OidArray { raw: raw }
    }
    fn raw(&self) -> raw::git_oidarray { self.raw }
}

impl Drop for OidArray {
    fn drop(&mut self) {
        unsafe { raw::git_oidarray_free(&mut self.raw) }
    }
}
