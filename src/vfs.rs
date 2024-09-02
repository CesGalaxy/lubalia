use std::path::PathBuf;

use lubengine::vfs::{module::Module, VirtualFileSystem};

pub struct LREVFS {

}

impl VirtualFileSystem for LREVFS {
    fn get_module(&self, location: &str) -> Option<Result<Module, ()>> {
        let mut path_buff = PathBuf::from(location);

        path_buff.set_extension("luba");

        let file = std::fs::read_to_string(path_buff);

        if let Ok(source_code) = file {
            let module = Module::new(location.to_string(), Module::parse(source_code));
            Some(Ok(module))
        } else {
            None
        }
    }
}