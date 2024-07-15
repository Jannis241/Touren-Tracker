mod jeskoFile;



use druid::{AppLauncher, Color, Widget, WindowDesc};
use druid::widget::{Label, Flex, Button};

fn build_ui() -> impl Widget<()> {

    let button = Button::new("Click me").on_click(|_ctx, _data, _env| {
        println!("Button clicked!");
    });
    Flex::column()
        .with_child(button)
}


fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Druid Example")
        .window_size((400.0, 400.0));
    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Failed to launch application");
}