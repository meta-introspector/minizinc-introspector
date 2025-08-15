use crate::base_type::MiniZincBaseType;
use crate::types::MiniZincTypeInst;
use crate::ffi_bindings::{typeinst_get_base_type, typeinst_is_var, typeinst_is_par, typeinst_is_opt, typeinst_is_present, typeinst_is_set, typeinst_is_int, typeinst_is_bool, typeinst_is_float, typeinst_is_string, typeinst_is_ann, typeinst_is_unknown, typeinst_is_plain, typeinst_is_bot, typeinst_is_top, typeinst_is_int_set, typeinst_is_bool_set, typeinst_is_float_set, typeinst_is_int_array, typeinst_is_bool_array, typeinst_is_int_set_array};

impl MiniZincTypeInst {
    pub fn base_type(&self) -> MiniZincBaseType {
        let id = unsafe { typeinst_get_base_type(self.0) };
        id.into()
    }

    pub fn is_var(&self) -> bool {
        unsafe { typeinst_is_var(self.0) }
    }

    pub fn is_par(&self) -> bool {
        unsafe { typeinst_is_par(self.0) }
    }

    pub fn is_opt(&self) -> bool {
        unsafe { typeinst_is_opt(self.0) }
    }

    pub fn is_present(&self) -> bool {
        unsafe { typeinst_is_present(self.0) }
    }

    pub fn is_set(&self) -> bool {
        unsafe { typeinst_is_set(self.0) }
    }

    pub fn is_int(&self) -> bool {
        unsafe { typeinst_is_int(self.0) }
    }

    pub fn is_bool(&self) -> bool {
        unsafe { typeinst_is_bool(self.0) }
    }

    pub fn is_float(&self) -> bool {
        unsafe { typeinst_is_float(self.0) }
    }

    pub fn is_string(&self) -> bool {
        unsafe { typeinst_is_string(self.0) }
    }

    pub fn is_ann(&self) -> bool {
        unsafe { typeinst_is_ann(self.0) }
    }

    pub fn is_unknown(&self) -> bool {
        unsafe { typeinst_is_unknown(self.0) }
    }

    pub fn is_plain(&self) -> bool {
        unsafe { typeinst_is_plain(self.0) }
    }

    pub fn is_bot(&self) -> bool {
        unsafe { typeinst_is_bot(self.0) }
    }

    pub fn is_top(&self) -> bool {
        unsafe { typeinst_is_top(self.0) }
    }

    pub fn is_int_set(&self) -> bool {
        unsafe { typeinst_is_int_set(self.0) }
    }

    pub fn is_bool_set(&self) -> bool {
        unsafe { typeinst_is_bool_set(self.0) }
    }

    pub fn is_float_set(&self) -> bool {
        unsafe { typeinst_is_float_set(self.0) }
    }

    pub fn is_int_array(&self) -> bool {
        unsafe { typeinst_is_int_array(self.0) }
    }

    pub fn is_bool_array(&self) -> bool {
        unsafe { typeinst_is_bool_array(self.0) }
    }

    pub fn is_int_set_array(&self) -> bool {
        unsafe { typeinst_is_int_set_array(self.0) }
    }
}