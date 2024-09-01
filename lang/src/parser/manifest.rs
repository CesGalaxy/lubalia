use std::collections::HashSet;

pub struct ProgramManifest {
    pub root_vars: usize,
    pub root_constants: HashSet<String>
}

impl Default for ProgramManifest {
    fn default() -> Self {
        Self {
            root_vars: 0,
            root_constants: HashSet::new()
        }
    }
}