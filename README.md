Sounds downloaded from
https://mixkit.co/free-sound-effects/notification/

## Usage

Run the app to open the graphical hangboard timer:

```sh
cargo run
```

Set the hang and rest durations, repeats, and number of sets, then select
**Start workout**. The timer plays the existing audio cues throughout the
workout. Use **Stop workout** to end a session early.

The original terminal-based workflow is also available:

```sh
cargo run -- --cli
```

## Releases

Push a version tag (for example, `v0.1.0`) to build macOS and Windows
distributions and publish them as assets on a GitHub release.
