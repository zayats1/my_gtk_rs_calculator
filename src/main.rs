use gtk4::{ApplicationWindow, Button, Entry, Orientation};
use gtk4::prelude::{BoxExt, ButtonExt, EditableExt, EntryBufferExtManual, GtkWindowExt};
use my_regex_calculator_lib::calculate;
use relm4::*;

enum AppMsg {
    Result,
}


struct AppModel {
    input_buffer: gtk::EntryBuffer,
    result: String,
}


impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Result => {
                let input_data = self.input_buffer.text();
                self.result = calculate(input_data);
            }
        }
        true
    }
}

struct AppWidgets {
    window: ApplicationWindow,
    vbox: gtk::Box,
    input: Entry,
    result_button: Button,
    result: Entry,
}


impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    fn init_view(model: &AppModel, _parent_widgets: &(), sender: Sender<AppMsg>) -> Self {
        let window = ApplicationWindow::builder()
            .title("Calculator")
            .default_width(300)
            .default_height(100)
            .build();

        let margin = 6;

        let vbox = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .margin_start(margin)
            .margin_end(margin)
            .margin_top(margin)
            .margin_bottom(margin)
            .build();

        let result_button = Button::builder()
            .label("=")
            .build();


        let input = Entry::builder()
            .buffer(&model.input_buffer)
            .editable(true)
            .build();


        let result = Entry::builder()
            .editable(false)
            .build();


        result_button.connect_clicked(move |_| {
            send!(sender, AppMsg::Result);
        });

        vbox.append(&input);
        vbox.append(&result);
        vbox.append(&result_button);

        window.set_child(Some(&vbox));

        Self {
            window,
            vbox,
            input,
            result_button,
            result,
        }
    }

    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(&mut self, model: &AppModel, _sender: Sender<AppMsg>) {
        self.result.set_text(&*model.result.to_owned());
    }
}


fn main() {
    let model = AppModel { input_buffer: Default::default(), result: "0".to_string() };
    let app = RelmApp::new(model);
    app.run();
}
