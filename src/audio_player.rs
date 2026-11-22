use crate::sound;
use rodio::{Decoder, DeviceSinkBuilder, Player};
use sound::AudioNotification;
use std::io::Cursor;
use std::sync::mpsc::{self, Sender};
use std::sync::OnceLock;
use std::thread;

static AUDIO_THREAD: OnceLock<Sender<AudioNotification>> = OnceLock::new();

pub fn bell() {
    play_sound(AudioNotification::Bell);
}

pub fn ding() {
    play_sound(AudioNotification::Ding);
}

pub fn end_of_round() {
    play_sound(AudioNotification::RoundDone);
}

pub fn finish() {
    play_sound(AudioNotification::Finish);
}

pub fn get_ready() {
    play_sound(AudioNotification::GetReady);
}

fn play_sound(sound: AudioNotification) {
    if let Err(error) = audio_sender().send(sound) {
        eprintln!("Failed to queue audio notification: {error}");
    }
}

fn audio_sender() -> &'static Sender<AudioNotification> {
    AUDIO_THREAD.get_or_init(|| {
        let (sender, receiver) = mpsc::channel();
        thread::Builder::new()
            .name("hangboard-audio".to_owned())
            .spawn(move || {
                while let Ok(sound) = receiver.recv() {
                    if let Err(error) = play_sound_on_audio_thread(sound) {
                        eprintln!("{error}");
                    }
                }
            })
            .expect("Failed to start audio playback thread");
        sender
    })
}

fn play_sound_on_audio_thread(sound: AudioNotification) -> Result<(), String> {
    let decoder = Decoder::try_from(Cursor::new(sound.audio_data()))
        .map_err(|error| format!("Failed to decode the sound {sound:?}: {error:?}"))?;
    let mut stream = DeviceSinkBuilder::open_default_sink()
        .map_err(|error| format!("Failed to open audio output stream: {error:?}"))?;
    stream.log_on_drop(false);
    let player = Player::connect_new(stream.mixer());
    player.append(decoder);
    player.sleep_until_end();
    Ok(())
}
