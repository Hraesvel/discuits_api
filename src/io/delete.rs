pub trait Delete<T> {
    fn remove(id: &str) -> std::io::Result<()>;
}