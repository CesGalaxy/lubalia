use module::Module;

pub mod module;

pub trait VirtualFileSystem {
    fn get_module(&self, location: String) -> Result<Module, VFSError>;
}

#[derive(Debug)]
pub enum VFSError {
    NotFound,
    LangError,
}
