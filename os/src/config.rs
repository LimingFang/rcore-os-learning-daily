pub const MAX_APP_NUM: usize = 20;
pub const APP_BASE_ADDRESS: usize = 0x8040_0000;
pub const APP_MAX_SIZE: usize = 2 << 20; // 2MB
pub const KERNEL_STACK_SIZE: usize = 8 << 10; // 8KB
pub const USER_STACK_SIZE: usize = 8 << 10; // 8KB
