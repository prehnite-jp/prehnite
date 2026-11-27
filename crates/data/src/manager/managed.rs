use crate::manager::weak_inner_data_manager::WeakInnerDataManager;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Deref;

#[cfg(feature = "gpui")]
pub use _gpui_impl::PrehniteManagedEntity;

/// `PrehniteManaged<T>` で扱えることを表すマーカートレイト
pub trait PrehniteManagedItem:
    for<'de> Deserialize<'de> + Serialize + Clone + Debug + 'static
{
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound(deserialize = "T: PrehniteManagedItem"))]
pub struct PrehniteManaged<T> {
    #[serde(skip)]
    manager: WeakInnerDataManager,
    item: T,
}

impl<T: PrehniteManagedItem> Deref for PrehniteManaged<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.as_ref()
    }
}

impl<T> AsRef<T> for PrehniteManaged<T> {
    fn as_ref(&self) -> &T {
        &self.item
    }
}

impl<T: PrehniteManagedItem> PrehniteManaged<T> {
    pub(crate) fn new(manager: WeakInnerDataManager, item: T) -> Self {
        Self { manager, item }
    }

    pub(crate) fn manager(&self) -> &WeakInnerDataManager {
        &self.manager
    }

    pub(crate) fn as_mut(&mut self) -> &mut T {
        &mut self.item
    }

    pub fn unmanage(self) -> T {
        self.item
    }

    pub fn clone_item(&self) -> T {
        self.item.clone()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { item: Some(self) }
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a, T> {
    item: Option<&'a PrehniteManaged<T>>,
}

impl<'a, T: PrehniteManagedItem> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(&self.item.take()?.item)
    }
}

#[derive(Debug)]
pub struct IntoIter<T> {
    item: Option<PrehniteManaged<T>>,
}

impl<T: PrehniteManagedItem> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.item.take()?.unmanage())
    }
}

impl<T: PrehniteManagedItem> IntoIterator for PrehniteManaged<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { item: Some(self) }
    }
}

mod _gpui_impl {
    use crate::manager::weak_inner_data_manager::WeakInnerDataManager;
    use crate::manager::{PrehniteManaged, PrehniteManagedItem};
    use gpui_kit::{
        App, AppContext, AsyncApp, Context, Entity, IntoElement, Render, RenderOnce, Task, Window,
    };
    use std::ops::Deref;

    impl<T: PrehniteManagedItem> PrehniteManaged<T> {
        pub fn into_entity(self, cx: &mut App) -> PrehniteManagedEntity<T> {
            PrehniteManagedEntity::new(cx, self)
        }
    }

    #[derive(Clone, Debug)]
    pub struct PrehniteManagedEntity<T: PrehniteManagedItem> {
        item: Entity<PrehniteManaged<T>>,
        pub is_deleted: Entity<bool>,
    }

    impl<T: PrehniteManagedItem> PrehniteManagedEntity<T> {
        fn new<C: AppContext>(cx: &mut C, item: PrehniteManaged<T>) -> PrehniteManagedEntity<T> {
            Self {
                item: cx.new(move |_| item),
                is_deleted: cx.new(|_| false),
            }
        }

        pub fn manager<C: AppContext>(&self, cx: &mut C) -> WeakInnerDataManager {
            cx.read_entity(self, |x, _| x.manager().clone())
        }

        pub fn item<C: AppContext>(&self, cx: &mut C) -> T {
            cx.read_entity(self, |x, _| x.clone_item())
        }

        pub fn clone_inner<C: AppContext>(&self, cx: &mut C) -> PrehniteManaged<T> {
            cx.read_entity(self, |x, _| x.clone())
        }

        pub(crate) fn set_deleted<C: AppContext>(&self, cx: &mut C, deleted: bool) {
            self.is_deleted.update(cx, |v, cx| {
                *v = deleted;
                cx.notify()
            })
        }
    }

    impl<T: PrehniteManagedItem> Deref for PrehniteManagedEntity<T> {
        type Target = Entity<PrehniteManaged<T>>;

        fn deref(&self) -> &Self::Target {
            &self.item
        }
    }

    impl<T: PrehniteManagedItem> From<PrehniteManagedEntity<T>> for Entity<PrehniteManaged<T>> {
        fn from(value: PrehniteManagedEntity<T>) -> Self {
            value.item
        }
    }

    impl<T: PrehniteManagedItem + IntoElement> IntoElement for PrehniteManaged<T> {
        type Element = T::Element;

        fn into_element(self) -> Self::Element {
            T::into_element(self.unmanage())
        }
    }

    impl<T: PrehniteManagedItem + RenderOnce> Render for PrehniteManaged<T> {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            T::render(self.clone_item(), window, cx)
        }
    }

    impl<T: PrehniteManagedItem + RenderOnce> IntoElement for PrehniteManagedEntity<T>
    where
        PrehniteManaged<T>: Render,
    {
        type Element = <Entity<PrehniteManaged<T>> as IntoElement>::Element;

        fn into_element(self) -> Self::Element {
            self.item.into_element()
        }
    }
}
