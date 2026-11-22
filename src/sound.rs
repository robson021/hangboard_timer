#[derive(Eq, PartialEq, Hash, Debug)]
pub enum AudioNotification {
    Bell,
    Ding,
    Finish,
    RoundDone,
    GetReady,
}
impl AudioNotification {
    pub fn audio_data(&self) -> &'static [u8] {
        match self {
            AudioNotification::Bell => include_bytes!("../assets/sound/bell.wav"),
            AudioNotification::Ding => include_bytes!("../assets/sound/ding.wav"),
            AudioNotification::Finish => include_bytes!("../assets/sound/finish.wav"),
            AudioNotification::RoundDone => include_bytes!("../assets/sound/round_done.wav"),
            AudioNotification::GetReady => include_bytes!("../assets/sound/get_ready.wav"),
        }
    }
}
