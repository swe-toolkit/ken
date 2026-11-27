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
            // SAFETY: `address` and `length` are exactly what the successful
            // `mmap` above returned, and nothing else holds that range.
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

    /// Promise class: durable invariant (38 §1.9, typed allocation failure).
    /// MEASURED: only ENOMEM classifies as native mapping allocation failure;
    /// EINVAL and EACCES remain I/O identities. CLAIMED: mapping failure
    /// classification does not mislabel non-allocation host errors. THE GAP:
    /// ProcessHost composition and dispatch each have a separate named test.
    #[cfg(target_os = "linux")]
    #[test]
    fn allocation_failure_classifies_only_enomem() {
        assert!(is_allocation_failure(IoErrorIdentityV1::Other(
            libc::ENOMEM
        )));
        assert!(!is_allocation_failure(IoErrorIdentityV1::Other(
            libc::EINVAL
        )));
        assert!(!is_allocation_failure(IoErrorIdentityV1::Other(
            libc::EACCES
        )));
    }

    /// Exit codes of the single-threaded unmap observer. Each failure mode has
    /// its own code so a red row names the step that went wrong.
    #[cfg(target_os = "linux")]
    mod unmap_observer {
        pub(super) const RELEASED: i32 = 0;
        pub(super) const MAP_FAILED: i32 = 10;
        pub(super) const LIVE_PROBE_REFUSED: i32 = 11;
        pub(super) const UNMAP_FAILED: i32 = 12;
        pub(super) const RANGE_STILL_MAPPED: i32 = 13;
        pub(super) const WRONG_REFUSAL: i32 = 14;
        pub(super) const PANICKED: i32 = 15;
    }

    /// Runs in the forked child, which has exactly one thread. Every step is
    /// a raw system call or allocation-free code, so nothing in the child can
    /// take a lock another parent thread held at fork time, and nothing in
    /// the child maps memory except `map_anonymous_v1` itself.
    #[cfg(target_os = "linux")]
    fn observe_unmap_in_single_threaded_child() -> i32 {
        let Ok(region) = map_anonymous_v1(1, MappingProtectionV1::ReadOnly) else {
            return unmap_observer::MAP_FAILED;
        };
        let address = region.address.as_ptr();
        let mut residency = [0_u8; 1];
        // SAFETY: `mincore` only inspects the virtual-address range. The range
        // is the live one-page mapping `region` owns.
        let live = unsafe { libc::mincore(address.cast(), 1, residency.as_mut_ptr()) };
        if live != 0 {
            return unmap_observer::LIVE_PROBE_REFUSED;
        }
        if unmap_v1(region).is_err() {
            return unmap_observer::UNMAP_FAILED;
        }
        // SAFETY: as above; the range is no longer owned by anything, and
        // `mincore` reads no memory in it.
        let freed = unsafe { libc::mincore(address.cast(), 1, residency.as_mut_ptr()) };
        if freed == 0 {
            return unmap_observer::RANGE_STILL_MAPPED;
        }
        match std::io::Error::last_os_error().raw_os_error() {
            Some(code) if code == libc::ENOMEM => unmap_observer::RELEASED,
            _ => unmap_observer::WRONG_REFUSAL,
        }
    }

    /// Promise class: durable invariant (`unmap_v1` consumes the mapping and
    /// releases its range).
    /// MEASURED: in a forked child that has exactly one thread, `mincore` on
    /// the page `map_anonymous_v1` returned succeeds while the mapping is
    /// live, and fails with ENOMEM immediately after `unmap_v1` returns Ok;
    /// the parent asserts the child exited normally with code 0.
    /// CLAIMED: `unmap_v1` released the range. The child's address space has
    /// no other thread, and between `unmap_v1` and the second `mincore` the
    /// child makes no call that maps memory, so no mapper can occupy the
    /// freed page: ENOMEM holds if and only if the range was released. The
    /// parent's other libtest threads map into the parent's address space,
    /// which the child stopped sharing at fork. No outcome is inconclusive:
    /// a still-mapped range is code 13 and fails the row, never retried.
    /// The live probe first pins that `mincore` sees this mapping at all, so a
    /// probe that always failed could not pass the row.
    /// THE GAP: this proves release of a one-page read-only mapping on Linux
    /// only; it says nothing about multi-page lengths or writable mappings.
    #[cfg(target_os = "linux")]
    #[test]
    fn consumed_mapping_is_actually_unmapped() {
        // SAFETY: the child runs only `observe_unmap_in_single_threaded_child`,
        // which allocates nothing and takes no lock, then leaves with `_exit`,
        // so it never returns into the test harness or runs atexit handlers.
        let child = unsafe { libc::fork() };
        assert!(
            child >= 0,
            "fork failed: {}",
            std::io::Error::last_os_error()
        );
        if child == 0 {
            let code = std::panic::catch_unwind(observe_unmap_in_single_threaded_child)
                .unwrap_or(unmap_observer::PANICKED);
            // SAFETY: `_exit` ends the child immediately without unwinding.
            unsafe { libc::_exit(code) };
        }
        let mut status: libc::c_int = 0;
        loop {
            // SAFETY: `child` is this process's own child, and `status` is a
            // valid out-pointer for the wait status.
            let waited = unsafe { libc::waitpid(child, &mut status, 0) };
            if waited == child {
                break;
            }
            let error = std::io::Error::last_os_error();
            // EINTR restarts the wait for the same child; it does not rerun
            // the observation.
            assert_eq!(
                error.raw_os_error(),
                Some(libc::EINTR),
                "waitpid failed: {error}"
            );
        }
        assert!(
            libc::WIFEXITED(status),
            "unmap observer did not exit normally: wait status {status:#x}"
        );
        assert_eq!(
            libc::WEXITSTATUS(status),
            unmap_observer::RELEASED,
            "unmap observer exit code (10 map failed, 11 live probe refused, \
             12 unmap failed, 13 range still mapped, 14 refusal not ENOMEM, \
             15 panicked)"
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
