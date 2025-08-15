impl MiniZincFloatLit {
    pub fn value(&self) -> f64 {
        unsafe { floatlit_get_value(self.0) }
    }
}