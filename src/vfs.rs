use std::{collections::HashMap, path::PathBuf};

use lubengine::vfs::{module::Module, VFSError, VirtualFileSystem};

pub struct LREVFS {
    pub cache: HashMap<String, Module>,
}

impl LREVFS {
    pub fn new() -> LREVFS {
        LREVFS {
            cache: HashMap::new(),
        }
    }
}

impl VirtualFileSystem for LREVFS {
    fn get_module(&self, location: String) -> Result<Module, VFSError> {
        if let Some(module) = self.cache.get(&location) {
            return Ok(module.clone());
        }

        let mut path_buff = PathBuf::from(&location);

        path_buff.set_extension("luba");

        let file = std::fs::read_to_string(path_buff);

        if let Ok(source_code) = file {
            if let Some(items) = Module::read(source_code) {
                let module = Module::new(location, items);
                Ok(module)
            } else {
                Err(VFSError::LangError)
            }
        } else {
            Err(VFSError::NotFound)
        }
    }
}