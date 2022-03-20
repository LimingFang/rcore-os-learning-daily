// 提供一次性加载所有 app、查看 app 加载地址等功能。
use core::slice::{from_raw_parts, from_raw_parts_mut};

use crate::config::*;
use crate::trap::TrapCtx;

// Return how many apps are loaded.
pub fn load_all_apps() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe {
        // =====> Large Address
        // <app_num> + N*<app_start> + last_app_end + app_bytes.
        let app_num = (_num_app as *const usize).read_volatile();
        let app_start_ptr = (_num_app as *const usize).add(1);
        let app_start_slice = from_raw_parts(app_start_ptr, app_num);
        let last_app_end = (_num_app as *const usize).add(1 + app_num).read_volatile();
        println!("[kernel]:num_app = {}", app_num);
        // copy program instructions.
        for i in 0..app_num {
            // [app_start,app_end)
            let app_start = get_app_start(i);
            let app_length = if i == app_num - 1 {
                last_app_end - app_start_slice[i]
            } else {
                app_start_slice[i + 1] - app_start_slice[i]
            };
            // Clean program area.
            (app_start..(app_start + APP_MAX_SIZE)).for_each(|byte| {
                (byte as *mut u8).write_volatile(0);
            });
            let target_slice = from_raw_parts_mut(app_start as *mut u8, app_length);
            let src_slice = from_raw_parts(app_start_slice[i] as *const u8, app_length);
            target_slice.copy_from_slice(src_slice);
        }
        app_num
    }
}

pub fn get_app_start(idx: usize) -> usize {
    APP_BASE_ADDRESS + APP_MAX_SIZE * idx
}

// TODO
pub fn get_app_len(_idx: usize) -> usize {
    APP_MAX_SIZE
}

pub fn get_app_num() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as *const usize).read_volatile() }
}

#[repr(align(4096))]
struct KernelStack {
    stack: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    stack: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    stack: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    stack: [0; USER_STACK_SIZE],
};

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.stack.as_ptr() as usize + USER_STACK_SIZE
    }
}

impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.stack.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_ctx(&self, ctx: TrapCtx) -> &'static mut TrapCtx {
        let ctx_ptr = (self.get_sp() - core::mem::size_of::<TrapCtx>()) as *mut TrapCtx;
        unsafe {
            *ctx_ptr = ctx;
            ctx_ptr.as_mut().unwrap()
        }
    }
}

// 在 kernel 上初始化 app 上下文，返回内核栈指针
pub fn init_app_ctx(idx: usize) -> usize {
    let app_start = get_app_start(idx);
    println!("[kernel]:app_start=0x{:x}", app_start);
    let user_sp = USER_STACK.get_sp();
    let ptr = KERNEL_STACK.push_ctx(TrapCtx::init_ctx(app_start, user_sp)) as *const _ as usize;
    ptr
}
