use core::cell::{RefCell, RefMut};
pub struct UPRefCell<T> {
    pub cell: RefCell<T>,
}

unsafe impl<T> Sync for UPRefCell<T> {}

impl<T> UPRefCell<T> {
    pub unsafe fn new(t: T) -> Self {
        Self {
            cell: RefCell::new(t),
        }
    }

    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.cell.borrow_mut()
    }
}
