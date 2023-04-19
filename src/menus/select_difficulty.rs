use super::{ButtonBuilder, LogoBundle, MenuButtonAction};
use crate::{despawn, ScreenState, WindowSize};
use bevy::prelude::*;

#[derive(Component)]
struct Difficulty(u8);

#[derive(Component)]
struct OnSelectDifficultyMenuScreen;

pub struct SelectDifficultyPlugin;

impl Plugin for SelectDifficultyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            select_difficulty_setup.in_schedule(OnEnter(ScreenState::SelectDifficulty)),
            despawn::<OnSelectDifficultyMenuScreen>
                .in_schedule(OnExit(ScreenState::SelectDifficulty)),
        ));
    }
}
fn select_difficulty_setup(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    asset_server: Res<AssetServer>,
) {
    // Logo.
    commands.spawn((
        LogoBundle::new(&asset_server, &window_size),
        OnSelectDifficultyMenuScreen,
    ));

    // Buttons.
    let button_builder = ButtonBuilder::new(&asset_server, &window_size);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(22.0 * window_size.vmin_scale),
                        left: Val::Auto,
                        right: Val::Px(7.0 * window_size.vmin_scale),
                        bottom: Val::Auto,
                    },
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnSelectDifficultyMenuScreen,
        ))
        .with_children(|parent| {
            use MenuButtonAction::*;
            button_builder.add_with_text_and_action(parent, "Easy", StartGameAtDifficulty(1));
            button_builder.add_with_text_and_action(parent, "Medium", StartGameAtDifficulty(2));
            button_builder.add_with_text_and_action(parent, "Advanced", StartGameAtDifficulty(3));
            button_builder.add_with_text_and_action(parent, "Expert", StartGameAtDifficulty(4));
            button_builder.add_back_with_text(parent, "Cancel");
        });
}
