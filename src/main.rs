#![windows_subsystem = "windows"]
use iced::{Color, Element, Sandbox, Settings, Alignment, alignment::{Horizontal}};
use std::collections::HashMap;
use iced::widget::{radio, text, row, text_input::{TextInput} ,Column, Text, Container, Button};
use iced::theme::{self, Theme};

#[derive(Default)]
struct WordCounter {
    input: String,
    counts: HashMap<String, u32>,
    theme: Theme,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ThemeType {
    Dark,
    Light,
    Custom,
}

#[derive(Debug, Clone)]
enum Message {
    Calculate,
    TextInputChanged(String),
    ThemeChanged(ThemeType),
}


pub fn main() -> iced::Result {
    WordCounter::run(Settings::default())
}

impl Sandbox for WordCounter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Word Counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {    //changes the theme of the application [TODO: make it a saved state]
                self.theme = match theme {
                    ThemeType::Dark => Theme::Dark,
                    ThemeType::Light => Theme::Light,
                    ThemeType::Custom => Theme::custom(theme::Palette {
                        background: Color::from_rgb(0.12, 0.13, 0.16,),
                        text: Color::from_rgb(0.9, 0.9, 0.9),
                        primary: Color::from_rgb(1.0, 0.2, 0.2),
                        success: Color::from_rgb(0.0, 1.0, 0.0),
                        danger: Color::from_rgb(1.0, 0.0, 0.0),
                    }),
                };
            }
            Message::Calculate => {    //calculates how many times each word appears in the input (Also lowercases them and removes unncessesary punctuation)
                self.input = self.input.to_lowercase().replace(&['/', ',', '.', ';', ':', '?', '!', '"'], "");
                self.counts = self.input.split_whitespace()
                    .fold(HashMap::new(), |mut counts, word| {
                        *counts.entry(word.to_string()).or_insert(0) += 1;
                        counts
                    });
           
            }
            Message::TextInputChanged(letter) => { //tracks input within textbox
                self.input = letter;
            }
        }
    }

    fn view(&self) -> Element<Message> {

        let choose_theme = [ThemeType::Light, ThemeType::Dark, ThemeType::Custom]  //makes a row of radio buttons to select the theme
            .iter()
            .fold(
                row![text("Choose a theme:")].spacing(10),
                |column, theme| {
                    column.push(radio(
                        format!("{:?}", theme),
                        *theme,
                        Some(match self.theme {
                            Theme::Dark => ThemeType::Dark,
                            Theme::Light => ThemeType::Light,
                            Theme::Custom { .. } => ThemeType::Custom,
                        }),
                        Message::ThemeChanged,
                    )
                    .size(23)
                )
                },
            );

        let input = TextInput::new("Enter text here...", &self.input,) //textbox
            .on_input(Message::TextInputChanged)
            .on_submit(Message::Calculate)
            .size(20)
            .padding(10)
            .width(iced::Length::Fill);

        let calc = Button::new("Calculate").on_press(Message::Calculate) //button to start calculation (pressing enter while textbox is in focus works too)
            .padding(15);

        let counts = self.counts.iter() //creates column of the results
            .fold(Column::new(), |column, (word, count)| {
                column.push(Text::new(format!("{}: {}", word, count)))
                    .padding(15)
            });

        let col = Column::new()    //pushes all the elements into a centered column
            .push(choose_theme)
            .push(input)
            .push(calc)
            .push(counts)
            .align_items(Alignment::Center)
            .spacing(10);
        let col_layout = Container::new(col)    //containter to center the column within the window
            .align_x(Horizontal::Center)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(20)
            .into();
        col_layout    //result of view function
    }
    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}