use std::time::{Duration, Instant};
use iced::{futures, executor, Application, clipboard, Command, Element, Settings, Font, Text, button, Button, Column, alignment, Length, Row, Alignment};
use iced_native::{Clipboard, Subscription};

struct GUI {
    start_stop_button_state: button::State,
    reset_button_state: button::State,
    tick_state: TickState,
    last_update: Instant,
    total_duration: Duration
}

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf")
};

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Reset,
    Update
}

pub enum TickState {
    Stopped,
    Ticking
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
                tick_state: TickState::Stopped,
                last_update: Instant::now(),
                total_duration: Duration::default()
            }, Command::none())
    }

    fn title(&self) -> String {
        String::from("STOPWATCH DEMO")
    }

    fn update(&mut self, _message: Self::Message)
        -> Command<Self::Message> {
        match _message {
            Message::Start => {
                self.tick_state = TickState::Ticking;
                self.last_update = Instant::now();
            }
            Message::Stop => {
                self.tick_state = TickState::Stopped;
                self.total_duration += Instant::now() - self.last_update;
            }
            Message::Reset => {
                self.last_update = Instant::now();
                self.total_duration = Duration::default();
            }
            Message::Update => match self.tick_state {
                TickState::Ticking => {
                    let current = Instant::now();
                    self.total_duration += current - self.last_update;
                    self.last_update = current;
                }
                _ => {}
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let timer = Timer::new(Duration::from_millis(MILLISECOND / FPS));
        iced::Subscription::from_recipe(timer).map(|_| Message::Update)
    }

    fn view(&mut self) -> Element<Self::Message> {
        // setup duration text.
        // let tick_text = Text::new("00:00:00.00").font(FONT).size(60);
        let seconds = self.total_duration.as_secs();
        let duration_text = format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.total_duration.subsec_millis() / 10
        );

        // setup start/stop text.
        let start_stop_text = match self.tick_state {
            TickState::Stopped => Text::new("Start")
                .horizontal_alignment(alignment::Horizontal::Center)
                .font(FONT),
            TickState::Ticking => Text::new("Stop")
                .horizontal_alignment(alignment::Horizontal::Center)
                .font(FONT)
        };

        // setup start/stop message on button press.
        let start_stop_message = match self.tick_state {
            TickState::Stopped => Message::Start,
            TickState::Ticking => Message::Stop
        };

        // init widgets.
        let tick_text = Text::new(duration_text).font(FONT).size(60);
        let start_stop_button = Button::new(
            &mut self.start_stop_button_state,
            start_stop_text,
            // Text::new("Start")
            //     .horizontal_alignment(alignment::Horizontal::Center)
            //     .font(FONT)
        ).width(Length::Units(80))
            .on_press(start_stop_message); // add start/stop message function
        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                .horizontal_alignment(alignment::Horizontal::Center)
                .font(FONT)
        ).width(Length::Units(80))
            .on_press(Message::Reset); // add reset message function

        // init Column
        Column::new()
            .push(tick_text)
            .push(Row::new()
                .push(start_stop_button)
                .push(reset_button)
                .spacing(10)
            )
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .into()
        // Text::new("Hello, World!").into()
    }
}

pub struct Timer {
    duration: Duration
}

impl Timer {
    fn new(duration: Duration) -> Timer {
        Timer { duration }
    }
}

const FPS: u64 = 30;
const MILLISECOND: u64 = 1000;
const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;

impl<H, E> iced_native::subscription::Recipe<H, E> for Timer
where
    H: std::hash::Hasher,
{
    type Output = Instant;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
        self.duration.hash(state);
    }

    fn stream(self: Box<Self>, _input: futures::stream::BoxStream<'static, E>)
        -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;;
        async_std::stream::interval(self.duration)
            .map(|_| Instant::now())
            .boxed()
    }
}

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings)
}
