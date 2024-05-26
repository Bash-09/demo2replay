use iced::{
    widget::{self, Container, Image},
    Length,
};

use crate::{App, Message};

type IcedContainer<'a> = Container<'a, Message, iced::Theme, iced::Renderer>;

#[must_use]
pub fn main_window(app: &App) -> IcedContainer<'_> {
    let content = widget::column![
        path_selection(app),
        widget::horizontal_rule(1),
        details(app)
    ]
    .padding(15)
    .spacing(15);

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
}

#[must_use]
pub fn path_selection(app: &App) -> IcedContainer<'_> {
    const BUTTON_WIDTH: u16 = 150;
    let content = widget::column![
        widget::row![
            widget::button("Set TF2 Directory")
                .on_press(Message::BrowseTF2Dir)
                .width(BUTTON_WIDTH),
            widget::text(
                app.tf2_dir
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default()
            ),
        ]
        .spacing(15)
        .align_items(iced::Alignment::Center),
        widget::row![
            widget::button("Select demo file")
                .on_press(Message::BrowseDemoPath)
                .width(BUTTON_WIDTH),
            widget::text(
                app.demo_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default()
            ),
        ]
        .spacing(15)
        .align_items(iced::Alignment::Center),
        widget::row![
            widget::button("Select thumbnail")
                .on_press(Message::BrowseThumbnailPath)
                .width(BUTTON_WIDTH),
            widget::button("Clear").on_press(Message::ClearThumbnail),
            widget::text(
                app.thumbnail_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default()
            ),
        ]
        .spacing(15)
        .align_items(iced::Alignment::Center),
    ]
    .align_items(iced::Alignment::Start)
    .spacing(5)
    .width(Length::Fill);

    Container::new(content)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Left)
        .center_y()
}

#[must_use]
pub fn details(app: &App) -> IcedContainer<'_> {
    const DETAIL_WIDTH: u16 = 120;
    match &app.demo {
        Ok(header) => {
            let content = widget::column![
                widget::row![
                    // thumbnail
                    Image::new(app.thumbnail_handle.clone())
                        .width(512)
                        .height(288)
                        .content_fit(iced::ContentFit::None),
                    // details
                    widget::column![
                        widget::row![
                            widget::text("Replay name: ").width(DETAIL_WIDTH),
                            widget::text_input("Replay Name", &app.replay_name)
                                .on_input(Message::SetReplayName),
                        ]
                        .align_items(iced::Alignment::Center),
                        widget::row![
                            widget::text("Map: ").width(DETAIL_WIDTH),
                            widget::text(&header.map),
                        ]
                        .align_items(iced::Alignment::Center),
                        widget::row![
                            widget::text("Player: ").width(DETAIL_WIDTH),
                            widget::text(&header.nick)
                        ]
                        .align_items(iced::Alignment::Center),
                        widget::row![
                            widget::text("Server: ").width(DETAIL_WIDTH),
                            widget::text(&header.server)
                        ]
                        .align_items(iced::Alignment::Center),
                        widget::row![
                            widget::text("Length: ").width(DETAIL_WIDTH),
                            widget::text(format!("{:.2}s", header.duration)),
                        ]
                        .align_items(iced::Alignment::Center),
                        widget::row![
                            widget::text("Ticks: ").width(DETAIL_WIDTH),
                            widget::text(format!("{}", header.ticks))
                        ]
                        .align_items(iced::Alignment::Center),
                    ]
                    .spacing(5),
                ]
                .spacing(15),
                // convert
                widget::button("Create Replay").on_press(Message::CreateReplay)
            ]
            .spacing(15);

            Container::new(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .align_y(iced::alignment::Vertical::Top)
        }
        Err(e) => Container::new(widget::text(format!("Invalid demo: {e}")))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y(),
    }
}
