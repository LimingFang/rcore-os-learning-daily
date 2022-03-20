use crate::loader::{get_app_num, init_app_ctx};
use crate::sync::UPRefCell;
use lazy_static::*;
mod task;

pub struct AppManager {
    num_app: usize,
    current_app: usize,
}

impl AppManager {
    pub fn get_current_app(&self) -> usize {
        self.current_app
    }
    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
    pub fn get_app_num(&self) -> usize {
        self.num_app
    }
}

lazy_static! {
    pub static ref APP_MANAGER: UPRefCell<AppManager> = unsafe {
        UPRefCell::new({
            let app_num = get_app_num();
            AppManager {
                num_app: app_num,
                current_app: 0,
            }
        })
    };
}

pub fn run_next_app() -> ! {
    let app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    println!("[kernel]:current_app = {}", current_app);
    if current_app >= app_manager.get_app_num() {
        panic!("All applications completed!");
    }
    drop(app_manager);
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(init_app_ctx(current_app));
    };
    panic!("Unreachable in batch::run_current_app!");
}
