use riscv::register::sstatus;

#[repr(C)]
pub struct TrapCtx {
    pub x: [usize; 32],
    pub sstatus: sstatus::Sstatus,
    pub sepc: usize,
}

impl TrapCtx {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    // 初始化 TrapCtx，通过 __restore 可以跳转至相应用户程序
    // 运行。
    pub fn init_ctx(entry: usize, sp: usize) -> Self {
        // SPP：发生异常前的权限模式，设置为 User
        // sepc:entry
        let mut st = sstatus::read();
        st.set_spp(sstatus::SPP::User);
        let mut ctx = Self {
            x: [0; 32],
            sstatus: st,
            sepc: entry,
        };
        ctx.set_sp(sp);
        ctx
    }
}
