pub trait Checkpoint<Save> {
    fn checkpoint(&self) -> Save;

    fn rollback(&mut self, save: Save);
}