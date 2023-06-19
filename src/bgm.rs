use bevy::prelude::*;

pub struct BgmPlugin;

#[derive(Resource)]
struct Bgm(Handle<AudioSource>);

fn load_bgm(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bgm = asset_server.load("little-dark-age.ogg");
    commands.insert_resource(Bgm(bgm));
}

fn play_bgm(audio: Res<Audio>, bgm: Res<Bgm>, mut is_playing: Local<bool>) {
    if *is_playing {
        return;
    }

    audio.play_with_settings(
        bgm.0.clone(),
        PlaybackSettings {
            repeat: true,
            volume: 0.08,
            ..default()
        },
    );
    *is_playing = true;
}

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_bgm).add_system(play_bgm);
    }
}
