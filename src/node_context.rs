use std::marker::PhantomData;

use bevy::prelude::*;

pub struct NC2<T0, T1>(PhantomData<(T0, T1)>);

pub trait NodeContext {
    type Wrapper<'c>;

    fn has_changed<'c>(wrapper: &Self::Wrapper<'c>) -> bool;
}

impl<R: Resource> NodeContext for R {
    type Wrapper<'c> = Res<'c, R>;

    fn has_changed<'c>(wrapper: &'c Self::Wrapper<'c>) -> bool {
        DetectChanges::is_changed(wrapper)
    }
}

impl<N0: NodeContext, N1: NodeContext> NodeContext for NC2<N0, N1> {
    type Wrapper<'c> = (N0::Wrapper<'c>, N1::Wrapper<'c>);

    fn has_changed<'c>(wrapper: &Self::Wrapper<'c>) -> bool {
        let (w0, w1) = wrapper;
        N0::has_changed(w0) || N1::has_changed(w1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoContext;

impl NodeContext for NoContext {

    type Wrapper<'c> = ();

    fn has_changed<'c>(wrapper: &Self::Wrapper<'c>) -> bool {
        false
    }
}

// // macro_rules! impl_node_context_resource_tuples {
// //     ($(($T:ident, $t:ident)),*) => {
// //         impl<$($T : DetectChanges),*> HasDetectChanges for ($($T,)*)  {

// //         fn has_changed(
// //             &self,
// //         ) -> bool {
// //             let &($($t,)*) = &self;
// //             false $(|| $t.is_changed())*
// //         }
// //         }
// //     }
// // }

// bevy::utils::all_tuples!(impl_state_tree_args, 0, 15, T, t);
