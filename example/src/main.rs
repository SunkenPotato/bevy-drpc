use bevy::{app::{App, Update}, input::ButtonInput, prelude::{EventWriter, KeyCode, Res, ResMut, Resource}, DefaultPlugins};

use birdy::{config::{RPCConfig, RPCPlugin}, discord_presence::models::Activity, plugin::ActivityUpdate};

#[derive(Resource)]
struct Counter(u32);

fn main() {
    let rpc_plugin = RPCPlugin::from(RPCConfig::default());
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(rpc_plugin)
        .add_systems(Update, update_activity)
        .insert_resource(Counter(0))
        .run();
}

fn update_activity(kb_input: Res<ButtonInput<KeyCode>>, mut event_writer: EventWriter<ActivityUpdate>, mut ctr: ResMut<Counter>) {
    if kb_input.just_pressed(KeyCode::Space) {
        ctr.0 += 1;
        let activity = ActivityUpdate(Activity::new().state(format!("Counter: {}", ctr.0)).details("Developing a game"));

        event_writer.send(activity);
    }
}