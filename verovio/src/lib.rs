use std::{
    ffi::{CString, CStr},
    path::Path,
};

mod bindings;

pub use bindings::*;

pub struct Verovio {
    toolkit: Option<*mut Toolkit>,
}

impl Default for Verovio {
    fn default() -> Self {
        let toolkit = unsafe { vrvToolkit_constructor() as *mut Toolkit };
        Self {
            toolkit: Some(toolkit)
        }
    }
}

impl Verovio {
    pub fn new(data_path: impl AsRef<Path>) -> Self {
        let path: &Path = data_path.as_ref();
        let path = CString::new(path.to_str().unwrap()).unwrap();
        let toolkit = unsafe { vrvToolkit_constructorResourcePath(path.as_ptr()) };
        Self {
            toolkit: Some(toolkit)
        }
    }

    pub fn render_data(&mut self, data: &str) -> String {
        let data = CString::new(data).unwrap();
        let options = CString::new(r#"{"footer": "none"}"#).unwrap();
        let result = unsafe { vrvToolkit_renderData(self.toolkit.unwrap(), data.as_ptr(), options.as_ptr()) };
        unsafe { CStr::from_ptr(result) }.to_str().unwrap().to_string()
    }
}

impl Drop for Verovio {
    fn drop(&mut self) {
        if let Some(ptr) = self.toolkit.take() {
            unsafe {
                vrvToolkit_destructor(ptr);
            }
        }
    }
}
