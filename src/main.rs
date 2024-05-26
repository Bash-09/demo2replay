use std::{io::Cursor, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use bitbuffer::BitRead;
use chrono::{Datelike, Timelike};
use filenamify::filenamify;
use gui::main_window;
use iced::{widget, Application};
use image::{io::Reader, DynamicImage, GenericImage, GenericImageView, ImageFormat};
use tf_demo_parser::{
    demo::{self, header::Header},
    Demo,
};

pub mod gui;

const DEFAULT_THUMBNAIL: &[u8] = include_bytes!("default.png");

const TEMPLATE_DMX: &str = include_str!("template_dmx.txt");
const TEMPLATE_VMT: &str = include_str!("template_vmt.txt");
const DIR_THUMBNAIL: &str = "tf/materials/vgui/replay/thumbnails";
const DIR_REPLAY: &str = "tf/replay/client/replays";
const DEMO_PATH: &str = "tf/demos";

const SUB_NAME: &str = "%replay_name%";
const SUB_MAP: &str = "%map%";
const SUB_LENGTH: &str = "%length%";
const SUB_TITLE: &str = "%title%";
const SUB_DEMO: &str = "%demo%";
const SUB_SCREENSHOT: &str = "%screenshot%";
const SUB_DATE: &str = "%date%";
const SUB_TIME: &str = "%time%";
const SUB_HANDLE: &str = "%handle%";

const TF2_APP_ID: u32 = 440;

#[derive(Debug, Clone)]
pub enum Message {
    BrowseTF2Dir,
    BrowseDemoPath,
    BrowseThumbnailPath,
    ClearThumbnail,
    CreateReplay,
    SetReplayName(String),
}

pub struct App {
    tf2_dir: Option<PathBuf>,
    demo_path: Option<PathBuf>,
    thumbnail_path: Option<PathBuf>,
    demo: Result<demo::header::Header, String>,
    status: String,

    replay_name: String,
    thumbnail: DynamicImage,
    thumbnail_handle: widget::image::Handle,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = Option<PathBuf>;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let thumbnail = DynamicImage::new(0, 0, image::ColorType::Rgb8);
        let mut image_bytes = Vec::new();
        thumbnail
            .write_to(&mut Cursor::new(&mut image_bytes), ImageFormat::Bmp)
            .expect("Couldn't write to vector???");
        let thumbnail_handle = widget::image::Handle::from_memory(image_bytes);

        let mut app = Self {
            tf2_dir: flags,
            demo_path: None,
            thumbnail_path: None,
            demo: Err(String::from("None chosen")),
            replay_name: String::new(),
            thumbnail,
            thumbnail_handle,
            status: String::new(),
        };

        app.load_thumbnail(None)
            .expect("Coudln't load default thumbnail");
        (app, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("demo2replay")
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    #[allow(clippy::too_many_lines)]
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::BrowseTF2Dir => {
                let Some(new_tf2_dir) = rfd::FileDialog::new().pick_folder() else {
                    return iced::Command::none();
                };
                self.tf2_dir = Some(new_tf2_dir);
            }
            Message::BrowseThumbnailPath => {
                if let Some(new_thumbnail_path) = rfd::FileDialog::new().pick_file() {
                    if let Err(e) = self.load_thumbnail(Some(new_thumbnail_path)) {
                        self.status = format!("Failed to set thumbnail: {e:?}");
                    }
                };
            }
            Message::BrowseDemoPath => {
                let mut picker = rfd::FileDialog::new();
                if let Some(tf2_dir) = &self.tf2_dir {
                    picker = picker.set_directory(tf2_dir.join(DEMO_PATH));
                }

                let Some(new_demo_path) = picker.pick_file() else {
                    return iced::Command::none();
                };
                self.demo_path = Some(new_demo_path);

                let Some(demo_path) = &self.demo_path else {
                    return iced::Command::none();
                };

                let bytes = match std::fs::read(demo_path) {
                    Ok(b) => b,
                    Err(e) => {
                        self.demo = Err(format!("{e}"));
                        return iced::Command::none();
                    }
                };

                let demo = Demo::new(&bytes);
                let mut stream = demo.get_stream();

                let header: Header = match Header::read(&mut stream) {
                    Ok(header) => header,
                    Err(e) => {
                        self.demo = Err(format!("Couldn't parse demo header ({e})"));
                        return iced::Command::none();
                    }
                };

                let datetime = chrono::offset::Local::now();
                self.replay_name = format!(
                    "{}-{}-{} {}:{} - {} on {}",
                    datetime.year(),
                    datetime.month(),
                    datetime.day(),
                    datetime.hour(),
                    datetime.minute(),
                    &header.nick,
                    &header.map,
                );

                println!("Loaded demo: {header:?}");
                self.demo = Ok(header);
            }
            Message::ClearThumbnail => {
                if let Err(e) = self.load_thumbnail(None) {
                    self.status = format!("Failed to set thumbnail: {e:?}");
                }
            }
            Message::CreateReplay => {
                if let Err(e) = self.create_replay() {
                    println!("Error creating replay: {e:?}");
                }
            }
            Message::SetReplayName(name) => self.replay_name = name,
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        main_window(self).into()
    }
}

impl App {
    #[allow(clippy::missing_errors_doc)]
    pub fn load_thumbnail(&mut self, new_thumbnail_path: Option<PathBuf>) -> Result<()> {
        let thumbnail_bytes = new_thumbnail_path.as_ref().map_or_else(
            || Ok(Vec::from(DEFAULT_THUMBNAIL)),
            |p| std::fs::read(p).context("Reading thumbnail file"),
        )?;

        let thumbnail_original = Reader::new(Cursor::new(&thumbnail_bytes))
            .with_guessed_format()
            .context("Determining file format")?
            .decode()
            .context("Decoding image")?
            .resize(512, 512, image::imageops::FilterType::Triangle);

        let mut thumbnail = DynamicImage::new(512, 512, image::ColorType::Rgb8);
        for (x, y, p) in thumbnail_original.pixels() {
            thumbnail.put_pixel(x, y, p);
        }

        let mut image_bytes = Vec::new();
        thumbnail
            .write_to(&mut Cursor::new(&mut image_bytes), ImageFormat::Bmp)
            .context("Writing file to buffer")?;

        let thumbnail_handle = widget::image::Handle::from_memory(image_bytes);

        self.thumbnail_path = new_thumbnail_path;
        self.thumbnail = thumbnail;
        self.thumbnail_handle = thumbnail_handle;

        Ok(())
    }

    /// Returns the create replay of this [`App`].
    ///
    /// # Errors
    /// If not all the required fields are present, or some IO error prevented file writeback.
    ///
    /// This function will return an error if .
    pub fn create_replay(&self) -> Result<()> {
        let Ok(header) = &self.demo else {
            return Err(anyhow!("No valid demo"));
        };
        let Some(tf2_dir) = &self.tf2_dir else {
            return Err(anyhow!("No TF2 directory set"));
        };
        let Some(demo_path) = &self.demo_path else {
            return Err(anyhow!("No demo provided"));
        };

        let file_name = filenamify(&self.replay_name);

        let handle = &mut std::fs::read_dir(tf2_dir.join(DIR_REPLAY))
            .context("Reading replay folder")?
            .filter_map(std::result::Result::ok)
            .filter(|d| d.path().extension().is_some_and(|e| e == "dmx"))
            .count();

        let datetime = chrono::offset::Local::now();

        #[allow(clippy::cast_sign_loss)]
        let date: u32 = (datetime.year() as u32 - 2009) << 9
            | (datetime.month() - 1) << 5
            | (datetime.day() - 1);
        let time: u32 = datetime.minute() << 5 | datetime.hour();

        let vtf = vtf::vtf::VTF::create(self.thumbnail.clone(), vtf::ImageFormat::Rgb888)
            .context("Creating thumbnail VTF")?;

        // Write replay DMX
        let mut dmx_contents = String::from(TEMPLATE_DMX);
        dmx_contents = dmx_contents.replace(SUB_NAME, &file_name);
        dmx_contents = dmx_contents.replace(SUB_MAP, &header.map);
        dmx_contents = dmx_contents.replace(SUB_LENGTH, &format!("{}", header.duration));
        dmx_contents = dmx_contents.replace(SUB_TITLE, &self.replay_name);
        dmx_contents = dmx_contents.replace(SUB_DEMO, &format!("{file_name}.dem"));
        dmx_contents = dmx_contents.replace(SUB_SCREENSHOT, &file_name);
        dmx_contents = dmx_contents.replace(SUB_DATE, &format!("{date}"));
        dmx_contents = dmx_contents.replace(SUB_TIME, &format!("{time}"));
        dmx_contents = dmx_contents.replace(SUB_HANDLE, &format!("{handle}"));

        std::fs::write(
            tf2_dir.join(DIR_REPLAY).join(format!("{file_name}.dmx")),
            dmx_contents,
        )
        .context("Writing demo DMX")?;

        std::fs::copy(
            demo_path,
            tf2_dir.join(DIR_REPLAY).join(format!("{file_name}.dem")),
        )
        .context("Copying demo file")?;

        // Write thumbnail stuff
        let mut thumbnail_vmt = String::from(TEMPLATE_VMT);
        thumbnail_vmt = thumbnail_vmt.replace(SUB_SCREENSHOT, &file_name);

        std::fs::write(
            tf2_dir.join(DIR_THUMBNAIL).join(format!("{file_name}.vmt")),
            thumbnail_vmt,
        )
        .context("Writing thumbnail VMT")?;

        std::fs::write(
            tf2_dir.join(DIR_THUMBNAIL).join(format!("{file_name}.vtf")),
            vtf,
        )
        .context("Writing thumbnail VTF")?;

        Ok(())
    }
}

fn main() {
    App::run(iced::Settings::with_flags(
        steamlocate::SteamDir::locate()
            .and_then(|mut s| s.app(&TF2_APP_ID).map(|a| a.path.clone())),
    ))
    .expect("Failed to run app.");
}
