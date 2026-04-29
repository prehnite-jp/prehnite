pub mod apply;
pub mod fetch;
pub mod global_registry;
pub mod hooks;
pub mod setting_schema;
pub mod supported_languages;
pub mod theme;

use crate::app::settings::global_registry::GlobalRegistry;
use setting_schema::GlobalSettings;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use tracing_unwrap::ResultExt;

macro_rules! applied_impl {
    ($registry_type:ty, $registry_global_variable:ident) => {
        thread_local! {
            static $registry_global_variable: RefCell<Rc<GlobalRegistry<$registry_type>>> = RefCell::new(Rc::new(GlobalRegistry::new()));
        }

        paste::paste! {
            pub fn [<get_ $registry_type:snake>]() -> Rc<GlobalRegistry<$registry_type>> {
                $registry_global_variable.with(|x| x.borrow().clone())
            }

            pub async fn [<load_ $registry_type:snake>]() {
                let settings = [<get_ $registry_type:snake>]().deref().clone();
                if let Some(load) = settings.load().await.ok_or_log() {
                    $registry_global_variable.with_borrow_mut(|x| *x = Rc::new(load))
                }
            }

            pub async fn [<save_ $registry_type:snake>](registry: $registry_type) {
                let settings = [<get_ $registry_type:snake>]().deref().clone();
                if let Some(saved) = settings.save(registry).await.ok_or_log() {
                    $registry_global_variable.with_borrow_mut(|x| *x = Rc::new(saved))
                }
            }
        }
    };
}

applied_impl!(GlobalSettings, APPLIED_GLOBAL_SETTINGS);
