use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

use super::get_or_load_asset;

#[derive(PartialEq, Debug)]
pub struct TextNode {
    pub text: String,
    pub style: Arc<TextNodeStyle>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TextNodeStyle {
    pub font_size: f32,
    pub color: Color,
    pub alignment: TextAlignment,
    pub font: &'static str,
    pub linebreak_behavior: bevy::text::BreakLineOn,
}

impl HierarchyNode for TextNode {
    type Context = AssetServer;

    fn set_components<'r>(
        &self,
        previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        event: SetComponentsEvent,
    ) {
        let font = get_or_load_asset(self.style.font, &context);

        let mut bundle = TextBundle::from_section(
            self.text.clone(),
            TextStyle {
                font,
                font_size: self.style.font_size,
                color: self.style.color,
            },
        )
        .with_text_alignment(self.style.alignment);

        bundle.text.linebreak_behavior = self.style.linebreak_behavior;

        //TODO only update text and node components
        commands.insert(bundle);
    }

    fn set_children<'r>(
        &self,
        previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    ) {
    }

    const CHILDREN_TYPE: ChildrenType = ChildrenType::Unordered;
}
