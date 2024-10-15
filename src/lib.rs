//! A simple unfinished bevy plugin for Discord RPC

pub use discord_presence;

pub mod config;

/// The main library crate containing bevy-specific structs/impls/functions
pub mod plugin {
    use std::ops::DerefMut;

    use bevy::{app::{Plugin, Startup, Update}, log::{debug, error, warn}, prelude::{Deref, Event, EventReader, ResMut, Resource}};
    use discord_presence::{models::Activity, Client as DiscordClient, DiscordError};

    use crate::config::RPCPlugin;

    #[derive(Resource, Deref)]
    struct Client(DiscordClient);

    /// Event which updates the current Discord activity
    #[derive(Event, Deref)]
    pub struct ActivityUpdate(pub Activity);

    /// Generate a default ActivityUpdate instance with an empty Activity struct
    impl Default for ActivityUpdate {
        fn default() -> Self {
            Self(Activity::default())
        }
    }

    impl DerefMut for Client {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Client {
        pub fn new(client_id: u64) -> Self {
            let rpc = DiscordClient::new(client_id);
            rpc.on_error(|e| error!("DiscordClient got an error: {e:#?}")).persist();

            Client(rpc)
        }
    }

    impl Plugin for RPCPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            let cc = &self.config;

            app.add_systems(Startup, startup_client);
            app.add_systems(Update, update_activity);
            
            app.insert_resource(Client::new(cc.app_id));
            app.add_event::<ActivityUpdate>();
        }
        
        fn name(&self) -> &str {
            "Discord Presence"
        }
    }

    fn startup_client(mut client: ResMut<Client>) {
        client.0.start();
    }

    fn update_activity(mut client: ResMut<Client>, mut activity_update: EventReader<ActivityUpdate>) {
        let event = match activity_update.read().next() {
            Some(v) => v,
            None => return,
        };
    
        let activity = &event.0; // Clone activity if needed for async use
    
        __update_activity(&mut client, activity)
    }

    fn __update_activity(client: &mut DiscordClient, activity: &Activity) {
        let update_err = match client.set_activity(|_| { activity.clone() }) {
            Ok(v) => {
                debug!("Activity updated: {v:#?}");
                return;
            },
            Err(e) => e
        };

        match update_err {
            DiscordError::IoError(error) => error!("Discord RPC error occurred: {error}"),
            DiscordError::SendMessage(error) => error!("An MPSC send error occurred: {error}"),
            DiscordError::CloseError(error) => error!("An MPSC send error occurred: {error}"),
            DiscordError::ReceiveError(error) => error!("An MPSC receive error occurred: {error}"),
            DiscordError::MPSCReceiveError(error) => error!("An MPSC receive error occurred: {error}"),
            DiscordError::MPSCTimeout(error) => error!("The MPSC channel timed out: {error}"),
            DiscordError::TimeoutError(error) => error!("The MPSC channel timed out: {error}"),
            DiscordError::JsonError(error) => error!("A JSON serialization error occurred: {error}"),
            DiscordError::ThreadError => error!("A discord_presence thread ran into an unexpected error."),
            DiscordError::NoneError(_) => error!("The library discord_presence unwrapped an Option<T> on a None value"),
            DiscordError::Conversion => error!("An unspecified conversion error occurred"),
            DiscordError::SubscriptionFailed => error!("An event subscription error occurred"),
            DiscordError::ConnectionClosed => error!("The connection was unexpectedly closed"),
            DiscordError::NotStarted => error!("The client has not started yet!"),
            DiscordError::EventLoopError => error!("The MPSC send & receive loop ran into an error"),
            DiscordError::NoChangesMade => warn!("No changes were made to the event handle"),
            DiscordError::ThreadInUse => error!("The RPC thread is in use"),
        }
    }
}