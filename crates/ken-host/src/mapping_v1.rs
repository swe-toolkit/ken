//! Confined Linux anonymous-mapping boundary.
//!
//! The mapped address never leaves this module. Callers can borrow bounded byte
//! slices and consume the owner exactly once for `munmap`, but cannot obtain or
//! encode the address itself.

#![allow(unsafe_code)]

use crate::{io_error_identity_v1, IoErrorIdentityV1, MappingProtectionV1};

#[cfg(target_os = "linux")]
pub(crate) struct MappedRegionV1 {
    address: std::ptr::NonNull<u8>,
    length: usize,
}

#[cfg(not(target_os = "linux"))]
pub(crate) struct MappedRegionV1 {
    length: usize,
}

impl MappedRegionV1 {
    pub(crate) fn length(&self) -> usize {
        self.length
    }

    pub(crate) fn as_slice(&self) -> &[u8] {
        #[cfg(target_os = "linux")]
        {
            // SAFETY: `address` names the live mapping owned by `self`, and the
            // mapping remains live for this borrow. The constructor records the
            // exact nonzero mapped length.
            unsafe { std::slice::from_raw_parts(self.address.as_ptr(), self.length) }
        }
        #[cfg(not(target_os = "linux"))]
        {
            unreachable!("non-Linux targets cannot construct a mapped region")
        }
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [u8] {
        #[cfg(target_os = "linux")]
        {
            // SAFETY: `&mut self` is the unique borrow of the owned mapping for
            // this call, and the mapping remains live for the returned borrow.
            unsafe { std::slice::from_raw_parts_mut(self.address.as_ptr(), self.length) }
        }
        #[cfg(not(target_os = "linux"))]
        {
            unreachable!("non-Linux targets cannot construct a mapped region")
        }
    }
}

pub(crate) fn map_anonymous_v1(
    length: usize,
    protection: MappingProtectionV1,
) -> Result<MappedRegionV1, IoErrorIdentityV1> {
    #[cfg(target_os = "linux")]
    {
        let protection = native_protection_flags_v1(protection);
        // SAFETY: a null address asks the kernel to select an address; the file
        // descriptor and offset are ignored for MAP_ANONYMOUS. `length` is
        // nonzero by the caller's mapping-capacity admission.
        let address = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                length,
                protection,
                anonymous_map_flags_v1(),
                -1,
                0,
            )
        };
        if address == libc::MAP_FAILED {
            return Err(io_error_identity_v1(&std::io::Error::last_os_error()));
        }
        let Some(address) = std::ptr::NonNull::new(address.cast::<u8>()) else {
            // A successful mapping at address zero cannot be represented as a
            // Rust slice. Consume it immediately rather than leaking it or
            // exposing address-shaped state to callers.
            let _ = unsafe { libc::munmap(address, length) };
            return Err(IoErrorIdentityV1::Other(libc::EFAULT));
        };
        Ok(MappedRegionV1 { address, length })
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (length, protection);
        Err(IoErrorIdentityV1::Unsupported)
    }
}

pub(crate) fn is_allocation_failure(error: IoErrorIdentityV1) -> bool {
    #[cfg(target_os = "linux")]
    {
        matches!(error, IoErrorIdentityV1::Other(code) if code == libc::ENOMEM)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = error;
        false
    }
}

pub(crate) fn unmap_v1(region: MappedRegionV1) -> Result<(), IoErrorIdentityV1> {
    #[cfg(target_os = "linux")]
    {
        // SAFETY: `region` uniquely owns this still-live mapping and is consumed
        // by this call, so no safe borrower can use it after the attempt.
        let status = unsafe { libc::munmap(region.address.as_ptr().cast(), region.length) };
        if status == 0 {
            Ok(())
        } else {
            Err(io_error_identity_v1(&std::io::Error::last_os_error()))
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = region;
        Err(IoErrorIdentityV1::Unsupported)
    }
}

#[cfg(target_os = "linux")]
const fn native_protection_flags_v1(protection: MappingProtectionV1) -> libc::c_int {
    match protection {
        MappingProtectionV1::ReadOnly => libc::PROT_READ,
        MappingProtectionV1::Writable => libc::PROT_READ | libc::PROT_WRITE,
    }
}

#[cfg(target_os = "linux")]
const fn anonymous_map_flags_v1() -> libc::c_int {
    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn anonymous_mapping_flags_are_private_not_shared() {
        let flags = anonymous_map_flags_v1();
        assert_ne!(flags & libc::MAP_PRIVATE, 0);
        assert_eq!(flags & libc::MAP_SHARED, 0);
        assert_ne!(flags & libc::MAP_ANONYMOUS, 0);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn consumed_mapping_is_actually_unmapped() {
        let region =
            map_anonymous_v1(1, MappingProtectionV1::ReadOnly).expect("one-byte anonymous mapping");
        let address = region.address.as_ptr();
        unmap_v1(region).expect("munmap succeeds exactly once");
        let mut residency = [0_u8; 1];
        // SAFETY: `mincore` only inspects the virtual-address range; the test
        // expects the just-unmapped range to be rejected with ENOMEM.
        let status = unsafe { libc::mincore(address.cast(), 1, residency.as_mut_ptr()) };
        assert_eq!(status, -1);
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ENOMEM)
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn mapping_protection_is_read_only_or_read_write_exactly() {
        assert_eq!(
            native_protection_flags_v1(MappingProtectionV1::ReadOnly),
            libc::PROT_READ
        );
        assert_eq!(
            native_protection_flags_v1(MappingProtectionV1::Writable),
            libc::PROT_READ | libc::PROT_WRITE
        );
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn anonymous_mapping_is_explicitly_unsupported_off_linux() {
        assert!(matches!(
            map_anonymous_v1(1, MappingProtectionV1::ReadOnly),
            Err(IoErrorIdentityV1::Unsupported)
        ));
    }
}
