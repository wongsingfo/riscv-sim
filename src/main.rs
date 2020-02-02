use iced::{Button, Color, Column, Container, Element, Font, Length, Row, Sandbox, Scrollable, Settings, Space, Text, TextInput};
use iced::{button, scrollable, text_input};

use objdump::Elf;

const MONOSPACED_FONT: Font = Font::External {
    name: "Courier",
    bytes: include_bytes!("fonts/SFNSMono.ttf"),
};

fn main() {
    RiscvSim::run(Settings::default());
}

#[derive(Default)]
struct TextInputState {
    value: String,
    state: text_input::State,
}

#[derive(Default)]
struct RiscvSim {
    processor: Processor,
    load_button: button::State,
    next_button: button::State,
    scroll: scrollable::State,
    search_symbol: TextInputState,
    elf: Elf,
    symbols: Vec<(String, u64)>,
}

#[derive(Debug, Default)]
struct Processor {}

#[derive(Debug, Default)]
struct SymbolTable {}

#[derive(Debug, Clone)]
enum Message {
    LoadFile,
    SearchInputChanged(String),
}

impl RiscvSim {
    fn update_search(&mut self) {
        let key = &self.search_symbol.value;

        self.symbols =
            self.elf.symbol_entries.iter().filter(|x| {
                x.0.contains(key)
            }).cloned().collect();
    }
}

impl Sandbox for RiscvSim {
    type Message = Message;

    fn new() -> Self {
        Self {
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("RISC-V RV64I simulator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::LoadFile => {
                if let Ok(rv) = Elf::open("objdump/test_obj/a.out") {
                    self.elf = rv;
                    self.update_search();
                }
            }
            Message::SearchInputChanged(new_value) => {
                self.search_symbol.value = new_value;
                self.update_search();
            }
            _ => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        let controls = Row::new()
            .spacing(20)
            .push(Button::new(
                &mut self.load_button,
                Text::new("Load"))
                .on_press(Message::LoadFile))
            .push(Button::new(
                &mut self.next_button,
                Text::new("Next")))
            .push(Space::with_width(Length::Fill));

        let search_symbol = TextInput::new(
             &mut self.search_symbol.state,
            "Search Symbol",
            &self.search_symbol.value,
            Message::SearchInputChanged,
        );

        let mut symbols = Column::new()
            .spacing(2);

        for (name, value) in &self.symbols {
            let mut t = Row::new()
                .spacing(10)
                .push(Text::new(name).width(Length::FillPortion(1)))
                .push(Text::new(
                    format!("0x {:08x} {:08x}",
                            value >> 32, (*value) as u32)).font(MONOSPACED_FONT));
            symbols = symbols.push(t);
        }

        let content: Element<_> = Column::new()
            .max_width(820)
            .spacing(20)
            .padding(20)
            .push(controls)
            .push(search_symbol)
            .push(symbols)
            .into();
        
        let content =
            content.explain(Color::BLACK);

        let scrollable = Scrollable::new(&mut self.scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
//            .center_y()
            .into()
    }
}
