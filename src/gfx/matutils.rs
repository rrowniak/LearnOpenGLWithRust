use ultraviolet::Mat4;

pub trait Mat4Ext {
    fn remove_translation(&mut self);
}

impl Mat4Ext for Mat4 {
    fn remove_translation(&mut self) {
        self[3][0] = 0.0;
        self[3][1] = 0.0;
        self[3][2] = 0.0;
        self[3][3] = 1.0;
        self[3][0] = 0.0;
        self[3][1] = 0.0;
        self[3][2] = 0.0;
    }
}
