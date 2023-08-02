use std::rc::Rc;

use bevy::{
    ecs::system::{EntityCommands, StaticSystemParam},
    prelude::*,
    utils::hashbrown::HashMap,
};
use crate::prelude::*;



#[derive(Debug, Default)]
pub struct StateTreePlugin;

impl Plugin for StateTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, handle_scheduled_for_removal);
    }
}

pub fn register_state_tree<R: HierarchyRoot>(app: &mut App) {
    app.add_plugins(StateTreePlugin);
    app.add_systems(First, sync_state::<R>);
}

fn handle_scheduled_for_removal(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ScheduledForDeletion)>,
) {
    for (entity, mut schedule) in query.iter_mut() {
        schedule.timer.tick(time.delta());
        if schedule.timer.finished() {
            commands.entity(entity).despawn_recursive()
        }
    }
}

fn sync_state<'a, R: HierarchyRoot>(
    mut commands: Commands,
    param: StaticSystemParam<R::ContextParam<'a>>,
    root_query: Query<Entity, (Without<Parent>, With<HierarchyChildComponent<R>>)>,
    tree: Query<(EntityRef, &HierarchyChildComponent<R>)>,
) {
    let context = R::get_context(param);

    let changed = <R::Context as NodeContext>::has_changed(&context);
    if !changed {
        return;
    }

    let all_child_nodes: HashMap<Entity, (EntityRef, HierarchyChildComponent<R>)> =
        tree.iter().map(|(e, c)| (e.id(), (e, c.clone()))).collect();

    let all_child_nodes = Rc::new(all_child_nodes);

    let mut root_commands =
        RootCommands::new(&mut commands, &context, all_child_nodes, root_query);

    R::set_children(&(),&context, &mut root_commands);
    root_commands.finish();
}
