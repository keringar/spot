use gtk::prelude::*;
use gtk::ImageExt;
use gtk::RangeExt;
use std::rc::Rc;
use std::cell::{RefCell};

use crate::app::{AppAction, SongDescription};
use crate::app::components::{Component};


pub trait PlaybackModel {
    fn is_playing(&self) -> bool;
    fn current_song(&self) -> Option<SongDescription>;
    fn play_next_song(&self);
    fn play_prev_song(&self);
    fn toggle_playback(&self);
}

pub struct Playback {
    play_button: gtk::Button,
    current_song_info: gtk::Label,
    seek_bar: gtk::Range,
    seek_source_id: RefCell<Option<glib::source::SourceId>>,
    model: Rc<dyn PlaybackModel>
}

impl Playback {

    pub fn new(
        play_button: gtk::Button,
        current_song_info: gtk::Label,
        next: gtk::Button,
        prev: gtk::Button,
        seek_bar: gtk::Range,
        model: Rc<dyn PlaybackModel>) -> Self {

        let weak_model = Rc::downgrade(&model);
        play_button.connect_clicked(move |_| {
            weak_model.upgrade()
                .map(|model| model.toggle_playback());
        });

        let weak_model = Rc::downgrade(&model);
        next.connect_clicked(move |_| {
            weak_model.upgrade()
                .map(|model| model.play_next_song());
        });

        let weak_model = Rc::downgrade(&model);
        prev.connect_clicked(move |_| {
            weak_model.upgrade()
                .map(|model| model.play_prev_song());
        });

        /*worker.send_task(async move {
            let url = "https://images-na.ssl-images-amazon.com/images/I/71YJlc9Wb6L._SL1500_.jpg";
            let result = load_remote_image(url, 60, 60).await;
            image.set_from_pixbuf(result.as_ref());
        });*/

        Self { play_button, current_song_info, seek_bar, seek_source_id: RefCell::new(None), model }
    }

    fn toggle_playing(&self) {

        let is_playing = self.model.is_playing();

        self.play_button.get_children().first()
            .and_then(|child| child.downcast_ref::<gtk::Image>())
            .map(|image| {
                image.set_from_icon_name(
                    Some(playback_image(is_playing)),
                    gtk::IconSize::Button);
            })
            .expect("error updating icon");

        let seek_bar = self.seek_bar.clone();
        let new_source = if is_playing {
            Some(gtk::timeout_add_seconds(1, move || {
                let value = seek_bar.get_value();
                seek_bar.set_value(value + 1000.0);
                glib::Continue(true)
            }))
        } else {
            None
        };

        if let Some(previous) = self.seek_source_id.replace(new_source) {
            glib::source_remove(previous);
        }
    }

    fn update_current_info(&self) {

        if let Some(song) = self.model.current_song() {
            let title = glib::markup_escape_text(&song.title);
            let artist = glib::markup_escape_text(&song.artist);
            let label = format!("<b>{}</b>\n{}", title.as_str(), artist.as_str());
            self.current_song_info.set_label(&label[..]);

            let duration = song.duration as f64;
            self.seek_bar.set_range(0.0, duration);
            self.seek_bar.set_value(0.0);
        }
    }
}

impl Component for Playback {

    fn handle(&self, action: &AppAction) {
        match action {
            AppAction::Play|AppAction::Pause => {
                self.toggle_playing();
            },
            AppAction::Load(_)|AppAction::Previous|AppAction::Next => {
                self.update_current_info();
                self.toggle_playing();
            },
            _ => {}
        }
    }
}

fn playback_image(is_playing: bool) -> &'static str {
    if is_playing {
        "media-playback-pause"
    } else {
        "media-playback-start"
    }
}
