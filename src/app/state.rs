use std::rc::Rc;
use crate::app::{AppAction, AbstractDispatcher};
use crate::app::credentials;


#[derive(Clone, Debug)]
pub struct AlbumDescription {
    pub title: String,
    pub artist: String,
    pub uri: String,
    pub art: String,
    pub songs: Vec<SongDescription>,
    pub id: String
}

#[derive(Clone, Debug)]
pub struct SongDescription {
    pub title: String,
    pub artist: String,
    pub uri: String,
    pub duration: i64
}

impl SongDescription {
    pub fn new(title: &str, artist: &str, uri: &str, duration: i64) -> Self {
        Self { title: title.to_string(), artist: artist.to_string(), uri: uri.to_string(), duration }
    }
}

pub struct AppState {
    pub is_playing: bool,
    pub current_song_uri: Option<String>,
    pub playlist: Vec<SongDescription>,
    pub api_token: Option<String>
}

impl AppState {
    pub fn new(songs: Vec<SongDescription>) -> Self {
        Self {
            is_playing: false,
            current_song_uri: None,
            playlist: songs,
            api_token: Option::None
        }
    }
}

pub struct AppModel {
    pub state: AppState,
    pub dispatcher: Rc<dyn AbstractDispatcher<AppAction>>
}

impl AppModel {
    pub fn new(state: AppState, dispatcher: Rc<dyn AbstractDispatcher<AppAction>>) -> Self {
        Self { state, dispatcher }
    }

    pub fn dispatch(&self, action: AppAction) -> Option<()> {
        self.dispatcher.dispatch(action)
    }

    pub fn update_state(&mut self, message: &AppAction) {
        match message {
            AppAction::Play => {
                self.state.is_playing = true;
            },
            AppAction::Pause => {
                self.state.is_playing = false;
            },
            AppAction::Next => {
                let next = self.next_song().map(|s| s.uri.clone());
                if next.is_some() {
                    self.state.is_playing = true;
                    self.state.current_song_uri = next;
                }
            },
            AppAction::Previous => {
                let prev = self.prev_song().map(|s| s.uri.clone());
                if prev.is_some() {
                    self.state.is_playing = true;
                    self.state.current_song_uri = prev;
                }
            },
            AppAction::Load(uri) => {
                self.state.is_playing = true;
                self.state.current_song_uri = Some(uri.to_string());
            },
            AppAction::LoadPlaylist(tracks) => {
                self.state.playlist = tracks.to_vec();
            },
            AppAction::LoginSuccess(creds) => {
                let _ = credentials::save_credentials(creds.clone());
                self.state.api_token = Some(creds.token.clone());
            }
            _ => {}
        };
    }

    fn prev_song(&self) -> Option<&SongDescription> {
        let state = &self.state;
        state.current_song_uri.as_ref().and_then(|uri| {
            state.playlist.iter()
                .take_while(|&song| song.uri != *uri)
                .last()
        })
    }

    fn next_song(&self) -> Option<&SongDescription> {
        let state = &self.state;
        state.current_song_uri.as_ref().and_then(|uri| {
            state.playlist.iter()
                .skip_while(|&song| song.uri != *uri)
                .skip(1)
                .next()
        })
    }
}

