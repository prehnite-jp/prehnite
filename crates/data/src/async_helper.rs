use std::sync::OnceLock;
use tokio::runtime::Handle;

static TOKIO_HANDLE: OnceLock<Handle> = OnceLock::new();

pub fn set_async_handle(handle: Handle) {
    TOKIO_HANDLE.set(handle).ok();
}

#[allow(unused)]
pub(crate) fn async_handle() -> Handle {
    TOKIO_HANDLE.get_or_init(Handle::current).clone()
}

#[allow(unused)]
pub(crate) fn block_on<F: Future>(future: F) -> F::Output {
    async_handle().block_on(future)
}
