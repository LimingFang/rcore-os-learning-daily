use crate::loader::*;
use crate::sync::UPRefCell;
use crate::trap::TrapCtx;
use lazy_static::*;
mod task;

pub struct AppManager {
    num_app: usize,
    current_app: usize,
}

impl AppManager {
    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }
}

lazy_static! {
    pub static ref APP_MANAGER: UPRefCell<AppManager> = unsafe {
        UPRefCell::new({
            AppManager {
                num_app: load_all_apps(),
                current_app: 0,
            }
        })
    };
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    drop(app_manager);
    init_app_ctx(current_app);
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(init_app_ctx(current_app));
    };
    panic!("Unreachable in batch::run_current_app!");
}
