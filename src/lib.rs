use bevy::prelude::*;

pub struct PausePlugin {
    toggle_button: KeyCode,
}

#[derive(Resource)]
pub struct PausePluginConfig {
    pub toggle_button: KeyCode,
}

impl PausePlugin {
    pub fn new(toggle_button: KeyCode) -> Self {
        Self { toggle_button }
    }
}

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_pause)
            .insert_resource(PausePluginConfig {
                toggle_button: self.toggle_button,
            });
    }
}

fn toggle_pause(
    mut time: ResMut<Time<Virtual>>,
    input: Res<ButtonInput<KeyCode>>,
    config: Res<PausePluginConfig>,
) {
    if input.just_pressed(config.toggle_button) {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}
