use super::provider::Provider;

pub trait Namespace<T: Provider>: Clone {
    fn new(provider: T) -> Self;
    fn provider(&self) -> &T;
}
