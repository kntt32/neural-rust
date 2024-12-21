#[cfg(target_arch = "x86_64")]
pub mod x64_rdrand {
    use std::arch::global_asm;

    global_asm!(
        "
rdrand_row:
    rdrand rax
    ret
"
    );

    extern "sysv64" {
        fn rdrand_row() -> usize;
    }

    pub fn rdrand() -> usize {
        unsafe { rdrand_row() }
    }
}

#[cfg(target_arch = "x86_64")]
pub use x64_rdrand::rdrand;
