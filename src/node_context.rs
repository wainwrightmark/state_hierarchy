use std::marker::PhantomData;

use bevy:: prelude::*;


pub struct NC2<T0, T1>(PhantomData<(T0, T1)>);

pub trait NodeContext {
    type Ref<'r>; //TODO two params here
    type Wrapper<'c>;

    fn from_wrapper<'c>(wrapper: &Self::Wrapper<'c>) -> Self::Ref<'c>;
    fn has_changed<'c>(wrapper: &Self::Wrapper<'c>) -> bool;


    //fn from_item<'w: 'c,'s,'c>(item: <Self::Wrapper<'c> as SystemParam>::Item<'w, 's>)->Self::Wrapper<'c>;
}

impl<R: Resource> NodeContext for R {
    type Wrapper<'c> = Res<'c, R>;
    type Ref<'r> = &'r R;

    fn from_wrapper<'c>(wrapper: & Self::Wrapper<'c>) -> &'c R {
        let w = Res::clone(wrapper);
        w.into_inner()
    }

    fn has_changed<'c>(wrapper: &'c Self::Wrapper<'c>) -> bool {
        DetectChanges::is_changed(wrapper)
    }

    // fn from_item<'w: 'c,'s,'c>(item: <Self::Wrapper<'c> as SystemParam>::Item<'w, 's>)->Self::Wrapper<'c> {
    //     item
    // }
}

impl<N0: NodeContext, N1: NodeContext> NodeContext for NC2<N0, N1> {
    type Ref<'r> = (N0::Ref<'r>, N1::Ref<'r>);
    type Wrapper<'c> = (N0::Wrapper<'c>, N1::Wrapper<'c>);

    fn has_changed<'c>(wrapper: & Self::Wrapper<'c>) -> bool {
        let (w0, w1) = wrapper;
        N0::has_changed(w0) || N1::has_changed(w1)
    }

    fn from_wrapper<'c>(wrapper: &Self::Wrapper<'c>) -> Self::Ref<'c> {
        let (w0, w1) = wrapper;

        (N0::from_wrapper(w0), N1::from_wrapper(w1))
    }

    // fn from_item<'w: 'c,'s,'c>(item: <Self::Wrapper<'c> as SystemParam>::Item<'w, 's>)->Self::Wrapper<'c> {
    //     let (i0, i1) = item;
    //     let w0 = N0::from_item(i0);
    //     let w1 = N1::from_item(i1);
    //     (w0, w1)
    // }
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

// pub trait HasDetectChanges {
//     fn has_changed(&self) -> bool;
// }

// impl<'c, R: Resource> HasDetectChanges for Res<'c, R> {
//     fn has_changed(&self) -> bool {
//         self.is_changed()
//     }
// }

// macro_rules! impl_state_tree_args {
//     ($(($T:ident, $t:ident)),*) => {
//         impl<$($T : DetectChanges),*> HasDetectChanges for ($($T,)*)  {


//         fn has_changed(
//             &self,
//         ) -> bool {
//             let &($($t,)*) = &self;
//             false $(|| $t.is_changed())*
//         }
//         }
//     }
// }

// bevy::utils::all_tuples!(impl_state_tree_args, 0, 15, T, t);