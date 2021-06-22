use x86_64::VirtAddr;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct KernelEntry(pub VirtAddr);

pub type kernel_entry_fun = extern "sysv64" fn () -> !;
