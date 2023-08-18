use bevy::prelude::*;
use lazy_static::lazy_static;
use state_hierarchy::transition::prelude::*;
use state_hierarchy::{impl_hierarchy_root, prelude::*};

use std::time::Duration;
use std::{string::ToString, sync::Arc};
use strum::{Display, EnumIs};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_resource::<MenuState>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system);

    app.add_plugins(TransitionPlugin::<StyleLeftLens>::default());
    //app.add_plugins(TransitionPlugin::<StyleTopLens>::default());
    app.add_plugins(TransitionPlugin::<TransformScaleLens>::default());
    app.add_plugins(TransitionPlugin::<BackgroundColorLens>::default());

    app.register_state_hierarchy::<MenuRoot>();
    app.run();
}
fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<MenuState>,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            ButtonAction::OpenMenu => *state = MenuState::ShowMainMenu,
            ButtonAction::ChooseLevel => *state = MenuState::ShowLevelsPage(0),
            ButtonAction::NextLevelsPage => {
                match state.as_ref() {
                    MenuState::ShowLevelsPage(x) => *state = MenuState::ShowLevelsPage(x + 1),
                    _ => {}
                };
            }
            ButtonAction::PreviousLevelsPage => {
                match state.as_ref() {
                    MenuState::ShowLevelsPage(x) => {
                        *state = MenuState::ShowLevelsPage(x.saturating_sub(1))
                    }
                    _ => {}
                };
            }
            ButtonAction::None => {}
            _ => *state = MenuState::Closed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Resource, EnumIs)]
pub enum MenuState {
    #[default]
    Closed,
    ShowMainMenu,
    ShowLevelsPage(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MenuRoot;

impl_hierarchy_root!(MenuRoot);

impl HasContext for MenuRoot {
    type Context = NC2<MenuState, AssetServer>;
}

impl ChildrenAspect for MenuRoot {
    fn set_children(
        &self,
        _previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    ) {
        let transition_duration: Duration = Duration::from_secs_f32(0.5);

        fn get_carousel_child(page: u32) -> Option<Either2<MainMenu, LevelMenu>> {
            Some(if let Some(page) = page.checked_sub(1) {
                Either2::Case1(LevelMenu(page))
            } else {
                Either2::Case0(MainMenu)
            })
        }

        let carousel = match context.0.as_ref() {
            MenuState::Closed => {
                commands.add_child("open_icon", menu_button_node(), &context.1);
                return;
            }
            MenuState::ShowMainMenu => Carousel::new(0, get_carousel_child, transition_duration),
            MenuState::ShowLevelsPage(n) => {
                Carousel::new(n + 1_u32, get_carousel_child, transition_duration)
            }
        };

        commands.add_child("carousel", carousel, context);
    }
}

fn menu_button_node() -> ButtonNode<ButtonAction> {
    ButtonNode {
        text: Some((ButtonAction::OpenMenu.icon(), ICON_BUTTON_TEXT_STYLE.clone())),
        image: None,
        button_node_style: OPEN_MENU_BUTTON_STYLE.clone(),
        marker: ButtonAction::OpenMenu,
    }
}

fn icon_button_node(button_action: ButtonAction) -> ButtonNode<ButtonAction> {
    ButtonNode {
        text: Some((button_action.icon(), ICON_BUTTON_TEXT_STYLE.clone())),
        image: None,
        button_node_style: ICON_BUTTON_STYLE.clone(),
        marker: button_action,
    }
}

fn text_button_node(button_action: ButtonAction) -> ButtonNode<ButtonAction> {
    ButtonNode {

        text: Some((button_action.text(), TEXT_BUTTON_TEXT_STYLE.clone())),
        image: None,
        button_node_style: TEXT_BUTTON_STYLE.clone(),
        marker: button_action,
    }
}

fn text_and_image_button_node(button_action: ButtonAction, image_path: &'static str) -> ButtonNode<ButtonAction> {
    ButtonNode {

        text: Some((button_action.text(), TEXT_BUTTON_TEXT_STYLE.clone())),
        image: Some((image_path, SMALL_IMAGE_NODE_STYLE.clone())),
        button_node_style: TEXT_BUTTON_STYLE.clone(),
        marker: button_action,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MainMenu;

impl HasContext for MainMenu {
    type Context = NC2<MenuState, AssetServer>;
}

impl StaticComponentsAspect for MainMenu {
    type B = NodeBundle;

    fn get_bundle() -> Self::B {
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),  // Val::Px(MENU_OFFSET),
                right: Val::Percent(50.0), // Val::Px(MENU_OFFSET),
                top: Val::Px(MENU_OFFSET),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,

                ..Default::default()
            },
            z_index: ZIndex::Global(10),
            ..Default::default()
        }
    }
}

impl ChildrenAspect for MainMenu {
    fn set_children(
        &self,
        _previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    ) {
        for (key, action) in ButtonAction::main_buttons().iter().enumerate() {
            let button = text_button_node(*action);
            let button: WithTransition<ButtonNode<ButtonAction>, BackgroundColorLens, ()> =
                button.with_transition_in::<BackgroundColorLens>(
                    Color::WHITE.with_a(0.0),
                    Color::WHITE,
                    Duration::from_secs_f32(1.0),
                );

            commands.add_child(key as u32, button, &context.1)
        }

        commands.add_child(
            "image",
            ImageNode {
                image_node_style: BIG_IMAGE_NODE_STYLE.clone(),
                path: r#"images\MedalsGold.png"#,
            },
            &context.1,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LevelMenu(u32);

impl HasContext for LevelMenu {
    type Context = NC2<MenuState, AssetServer>;
}

impl StaticComponentsAspect for LevelMenu {
    type B = NodeBundle;

    fn get_bundle() -> Self::B {
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                right: Val::Percent(50.0),
                top: Val::Px(MENU_OFFSET),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,

                ..Default::default()
            },
            z_index: ZIndex::Global(10),
            ..Default::default()
        }
    }
}

impl ChildrenAspect for LevelMenu {
    fn set_children(
        &self,
        _previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    ) {
        let start = self.0 * LEVELS_PER_PAGE;
        let end = start + LEVELS_PER_PAGE;

        for (key, level) in (start..end).enumerate() {
            commands.add_child(
                key as u32,
                text_and_image_button_node(ButtonAction::GotoLevel { level }, r#"images/MedalsBlack.png"#),
                &context.1,
            )
        }

        commands.add_child("buttons", LevelMenuArrows(self.0), &context.1);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LevelMenuArrows(u32);

impl HasContext for LevelMenuArrows {
    type Context = AssetServer;
}

impl StaticComponentsAspect for LevelMenuArrows {
    type B = NodeBundle;

    fn get_bundle() -> Self::B {
        NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                left: Val::Percent(0.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,

                width: Val::Px(TEXT_BUTTON_WIDTH),
                height: Val::Px(TEXT_BUTTON_HEIGHT),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_grow: 0.0,
                flex_shrink: 0.0,
                border: UiRect::all(UI_BORDER_WIDTH),

                ..Default::default()
            },
            background_color: BackgroundColor(TEXT_BUTTON_BACKGROUND),
            border_color: BorderColor(BUTTON_BORDER),
            ..Default::default()
        }
    }
}

impl ChildrenAspect for LevelMenuArrows {
    fn set_children(
        &self,
        _previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    ) {
        if self.0 == 0 {
            commands.add_child("left", icon_button_node(ButtonAction::OpenMenu), context)
        } else {
            commands.add_child(
                "left",
                icon_button_node(ButtonAction::PreviousLevelsPage),
                context,
            )
        }

        if self.0 < 4 {
            commands.add_child(
                "right",
                icon_button_node(ButtonAction::NextLevelsPage),
                context,
            )
        } else {
            commands.add_child("right", icon_button_node(ButtonAction::None), context)
        }
    }
}

pub const ICON_BUTTON_WIDTH: f32 = 65.;
pub const ICON_BUTTON_HEIGHT: f32 = 65.;

pub const TEXT_BUTTON_WIDTH: f32 = 360.;
pub const TEXT_BUTTON_HEIGHT: f32 = 60.;

pub const MENU_OFFSET: f32 = 10.;

pub const UI_BORDER_WIDTH: Val = Val::Px(3.0);

pub const FONT_PATH: &str = "fonts/merged-font.ttf";

pub const ICON_FONT_SIZE: f32 = 30.0;
pub const BUTTON_FONT_SIZE: f32 = 22.0;

const LEVELS_PER_PAGE: u32 = 8;

pub const BACKGROUND_COLOR: Color = Color::hsla(216., 0.7, 0.72, 1.0); // #86AEEA
pub const ACCENT_COLOR: Color = Color::hsla(218., 0.69, 0.62, 1.0); // #5B8BE2
pub const WARN_COLOR: Color = Color::hsla(0., 0.81, 0.51, 1.0); // #FF6E5F
pub const TIMER_COLOR: Color = Color::BLACK;

pub const FIXED_SHAPE_FILL: Color = Color::WHITE;
pub const VOID_SHAPE_FILL: Color = Color::BLACK;

pub const FIXED_SHAPE_STROKE: Color = Color::BLACK;
pub const VOID_SHAPE_STROKE: Color = WARN_COLOR;
pub const ICE_SHAPE_STROKE: Color = Color::WHITE;

pub const SHADOW_STROKE: Color = Color::BLACK;

pub const LEVEL_TEXT_COLOR: Color = Color::DARK_GRAY;
pub const LEVEL_TEXT_ALT_COLOR: Color = Color::WHITE;

pub const BUTTON_BORDER: Color = Color::BLACK;
pub const BUTTON_TEXT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const ICON_BUTTON_BACKGROUND: Color = Color::NONE;
pub const TEXT_BUTTON_BACKGROUND: Color = Color::WHITE;
pub const DISABLED_BUTTON_BACKGROUND: Color = Color::GRAY;

lazy_static! {
    static ref BIG_IMAGE_NODE_STYLE: Arc<ImageNodeStyle> = Arc::new(ImageNodeStyle {
        background_color: Color::WHITE,
        style: Style {
            width: Val::Px(TEXT_BUTTON_HEIGHT * 2.0),
            height: Val::Px(TEXT_BUTTON_HEIGHT),
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Px(5.0),
                bottom: Val::Px(5.0),
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_grow: 0.0,
            flex_shrink: 0.0,
            border: UiRect::all(UI_BORDER_WIDTH),
            ..default()
        }
    });

    static ref SMALL_IMAGE_NODE_STYLE: Arc<ImageNodeStyle> = Arc::new(ImageNodeStyle {
        background_color: Color::WHITE,
        style: Style {
            width: Val::Px((TEXT_BUTTON_HEIGHT - 10.0) * 2.0),
            height: Val::Px(TEXT_BUTTON_HEIGHT - 10.0),
            margin: UiRect {
                left: Val::Auto,
                right: Val::Px(0.0),
                top: Val::Px(5.0),
                bottom: Val::Px(5.0),
            },
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::End,
            ..default()
        }
    });
    static ref ICON_BUTTON_STYLE: Arc<ButtonNodeStyle> = Arc::new(ButtonNodeStyle {
        style: Style {
            width: Val::Px(ICON_BUTTON_WIDTH),
            height: Val::Px(ICON_BUTTON_HEIGHT),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_grow: 0.0,
            flex_shrink: 0.0,

            ..Default::default()
        },
        background_color: Color::NONE,
        ..default()
    });
    static ref OPEN_MENU_BUTTON_STYLE: Arc<ButtonNodeStyle> = Arc::new(ButtonNodeStyle {
        style: Style {
            width: Val::Px(ICON_BUTTON_WIDTH),
            height: Val::Px(ICON_BUTTON_HEIGHT),
            margin: UiRect::DEFAULT,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_grow: 0.0,
            flex_shrink: 0.0,
            left: Val::Px(40.0),
            top: Val::Px(40.0),

            ..Default::default()
        },
        background_color: Color::NONE,
        ..default()
    });
    static ref TEXT_BUTTON_STYLE: Arc<ButtonNodeStyle> = Arc::new(ButtonNodeStyle {
        style: Style {
            width: Val::Px(TEXT_BUTTON_WIDTH),
            height: Val::Px(TEXT_BUTTON_HEIGHT),
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Px(5.0),
                bottom: Val::Px(5.0),
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_grow: 0.0,
            flex_shrink: 0.0,
            border: UiRect::all(UI_BORDER_WIDTH),

            ..Default::default()
        },
        background_color: TEXT_BUTTON_BACKGROUND,
        border_color: BUTTON_BORDER,
        ..Default::default()
    });
    static ref TEXT_BUTTON_TEXT_STYLE: Arc<TextNodeStyle> = Arc::new(TextNodeStyle {
        font_size: BUTTON_FONT_SIZE,
        color: BUTTON_TEXT_COLOR,
        font: FONT_PATH,
        alignment: TextAlignment::Center,
        linebreak_behavior: bevy::text::BreakLineOn::NoWrap
    });
    static ref ICON_BUTTON_TEXT_STYLE: Arc<TextNodeStyle> = Arc::new(TextNodeStyle {
        font_size: ICON_FONT_SIZE,
        color: BUTTON_TEXT_COLOR,
        font: FONT_PATH,
        alignment: TextAlignment::Center,
        linebreak_behavior: bevy::text::BreakLineOn::NoWrap
    });
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Component)]
pub enum ButtonAction {
    OpenMenu,
    Resume,
    ChooseLevel,
    GotoLevel { level: u32 },

    NextLevelsPage,
    PreviousLevelsPage,

    None,
}

impl ButtonAction {
    pub fn main_buttons() -> &'static [Self] {
        use ButtonAction::*;
        &[Resume, ChooseLevel]
    }

    pub fn icon(&self) -> String {
        use ButtonAction::*;
        match self {
            OpenMenu => "\u{f0c9}".to_string(),    // "Menu",
            Resume => "\u{e817}".to_string(),      // "Menu",
            ChooseLevel => "\u{e812}".to_string(), // "\u{e812};".to_string(),
            GotoLevel { level } => level.to_string(),
            PreviousLevelsPage => "\u{e81b}".to_string(),
            NextLevelsPage => "\u{e81a}".to_string(),
            None => "".to_string(),
        }
    }

    pub fn text(&self) -> String {
        use ButtonAction::*;
        match self {
            OpenMenu => "Menu".to_string(),
            Resume => "Resume".to_string(),
            ChooseLevel => "Choose Level".to_string(),
            GotoLevel { level } => {
                format!("Level {level}")
            }
            NextLevelsPage => "Next Levels".to_string(),
            PreviousLevelsPage => "Previous Levels".to_string(),

            None => "".to_string(),
        }
    }
}
