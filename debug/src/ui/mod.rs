use iced::Sandbox;
pub 
struct DebuggerApp;

impl Sandbox for DebuggerApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Lubalia ToolBox")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<Self::Message> {
        "This is the debugger!".into()
    }
}