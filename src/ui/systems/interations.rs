use bevy::{prelude::*, app::AppExit};

use crate::{ui::{components::{PlayButton, QuitButton}, styles::{NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR, NONE_BUTTON_COLOR}}, AppState, game::SimulationState};

pub fn interact_with_play_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<PlayButton>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut simulation_state: ResMut<NextState<SimulationState>>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = NORMAL_BUTTON_COLOR.into();
                app_state.set(AppState::Game);
                simulation_state.set(SimulationState::Running);
            }
            Interaction::Hovered => {
                *background_color = PRESSED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NONE_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit: EventWriter<AppExit>,
    mut button_query: Query<(&Interaction, 
        &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = NORMAL_BUTTON_COLOR.into();
                app_exit.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = PRESSED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NONE_BUTTON_COLOR.into();
            }
        }
    }
}