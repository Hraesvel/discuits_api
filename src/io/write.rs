pub trait Write<T> {
    type Data;

    fn create(data: T) -> Self::Data;

    fn update(data: T) -> Self::Data;
}
