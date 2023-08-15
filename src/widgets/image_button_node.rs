use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug)]
pub struct ImageButtonNode<Marker: Component + PartialEq + Clone> {
    pub image_handle: &'static str,
    pub button_node_style: Arc<ButtonNodeStyle>,
    pub marker: Marker,
}

impl<Marker: Component + PartialEq + Clone> HasContext for ImageButtonNode<Marker> {
    type Context = AssetServer;
}

impl<Marker: Component + PartialEq + Clone> HasNoChildren for ImageButtonNode<Marker> {}

impl<Marker: Component + PartialEq + Clone> ComponentsAspect for ImageButtonNode<Marker> {
    fn set_components<'r>(
        &self,
        _previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        event: SetComponentsEvent,
    ) {
        let texture = context.load(self.image_handle);

        commands.insert(ButtonBundle {
            style: self.button_node_style.style.clone(),
            border_color: BorderColor(self.button_node_style.border_color),
            background_color: BackgroundColor(self.button_node_style.background_color),
            image: UiImage {
                texture,
                flip_x: false,
                flip_y: false,
            },
            ..default()
        });

        if event == SetComponentsEvent::Created {
            commands.insert(self.marker.clone());
        }
    }
}
