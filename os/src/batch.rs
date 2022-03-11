use crate::sync::UPRefCell;
use crate::trap::TrapCtx;
#[allow(dead_code)]
use core::{arch::asm, slice};
use lazy_static::*;

const MAX_APP_NUM: usize = 20;
const APP_BASE_ADDRESS: usize = 0x8040_0000;
const APP_MAX_SIZE: usize = 2 << 20; // 2MB
const KERNEL_STACK_SIZE: usize = 1 << 10; // 8KB
const USER_STACK_SIZE: usize = 1 << 10; // 8KB

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

pub struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start_addr: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    unsafe fn load_next_app(&mut self) {
        // 镜像代码拷贝到APP_BASE_ADDRESS
        if self.current_app >= self.num_app {
            panic!("no more app");
        }
        println!("[kernel]:loading app_{}", self.current_app);
        asm!("fence.i");
        (APP_BASE_ADDRESS..(APP_BASE_ADDRESS + APP_MAX_SIZE)).for_each(|addr| unsafe {
            (addr as *mut u8).write_volatile(0);
        });
        let app_start = self.app_start_addr[self.current_app];
        let app_end = self.app_start_addr[self.current_app + 1];
        let app_length = app_end - app_start;
        let app_bytes_src = slice::from_raw_parts(app_start as *const u8, app_length);
        let app_bytes_dst = slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_length);
        app_bytes_dst.copy_from_slice(app_bytes_src);
        self.current_app += 1;
    }
}

lazy_static! {
    static ref APP_MANAGER: UPRefCell<AppManager> = unsafe {
        UPRefCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let app_start_raw = slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            let mut app_start = [0; MAX_APP_NUM + 1];
            app_start[0..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start_addr: app_start,
            }
        })
    };
}

pub fn init() {
    let app_manager = APP_MANAGER.exclusive_access();
    println!("[kernel]:num_app = {}", app_manager.num_app);
    for i in 0..app_manager.num_app {
        println!(
            "[kernel]num_{}_app:start{:#x},end{:#x}",
            i,
            app_manager.app_start_addr[i],
            app_manager.app_start_addr[i + 1] - 1,
        );
    }
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    unsafe {
        app_manager.load_next_app();
    }
    drop(app_manager);
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        let init_ctx = TrapCtx::init_ctx(APP_BASE_ADDRESS, USER_STACK.get_sp());
        let ptr = KERNEL_STACK.push_ctx(init_ctx) as *const _ as usize;
        __restore(ptr);
    };
    panic!("Unreachable in batch::run_current_app!");
}
