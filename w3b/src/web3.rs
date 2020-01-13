use super::{api::*, namespace::Namespace, provider::Provider};

pub struct Web3<T: Provider> {
    provider: T,
}

impl<T: Provider> Web3<T> {
    #[inline]
    pub fn new(provider: T) -> Self {
        Self { provider }
    }
}

impl<T: Provider> Web3<T> {
    pub fn namespace<N: Namespace<T>>(&self) -> N {
        N::new(self.provider.clone())
    }

    pub fn eth(&self) -> eth::Eth<T> {
        self.namespace()
    }
}
