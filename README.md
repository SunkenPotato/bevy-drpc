# bevy-drpc
Discord RPC plugin for [bevy](https://bevyengine.org/). (Unstable) \
Check `examples` on how to get started. :)

> 
> This project is currently still under development and blocks the main thread on an update.

## Example:
```rs no_run
#[derive(Resource)]
struct Counter(u32);

fn update_activity(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut activity_events: EventWriter<ActivityEvent>,
    mut counter: ResMut<Counter>
) {
    if (keyboard_input.pressed(KeyCode::Space)) {
        ctr.0 += 1;

        let new_activity = ActivityUpdate(
            Activity::new()
                .state(format!("Counter: {}", ctr.0))
                .details("Developing a game!");
        );

        activity_events.send(new_activity);
    }
}
```

Please report any issues on the GitHub repo. \
Feature requests are very welcome!