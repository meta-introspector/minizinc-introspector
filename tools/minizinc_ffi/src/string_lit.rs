impl MiniZincStringLit {
    pub fn value(&self) -> String {
        let c_str = unsafe { stringlit_get_value(self.0) };
        unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }
}