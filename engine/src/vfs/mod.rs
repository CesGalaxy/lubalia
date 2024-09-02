use module::Module;

pub mod module;

pub trait VirtualFileSystem {
    fn get_module(&self, location: &str) -> Option<Result<Module, ()>>;
}