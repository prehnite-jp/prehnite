use crate::bundle_license::BundleLicense;
use crate::set_icon::SetIcon;

mod bundle_license;
mod set_icon;
pub mod util;

#[macro_export]
macro_rules! build_process {
    ($t:ident) => {
        #[derive(Default)]
        pub struct $t;
    };
}

#[macro_export]
macro_rules! build_process_empty_impl {
    ($t:ty) => {
        impl BuildProcess for $t {
            fn execute(&self) -> anyhow::Result<()> { Ok(()) }
        }
    };
}

pub trait BuildProcess {
    fn new() -> Box<Self>
    where
        Self: Default,
    {
        Box::new(Self::default())
    }

    fn execute(&self) -> anyhow::Result<()>;
}

fn process() -> Vec<Box<dyn BuildProcess>> {
    vec![BundleLicense::new(), SetIcon::new()]
}

pub fn execute_all_build_process() -> anyhow::Result<()> {
    for i in process() {
        i.execute()?;
    }
    Ok(())
}
