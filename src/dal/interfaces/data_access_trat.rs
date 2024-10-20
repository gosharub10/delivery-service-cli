pub trait DataAccessTrait<T> {
    fn create(&mut self,entity: T) -> Result<(), postgres::Error>;
    fn get_all(&mut self) -> Result<Vec<T>, String>;
}