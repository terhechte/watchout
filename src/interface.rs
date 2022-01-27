use crate::{config::Config, delegate, watcher, WatchoutState};

use druid::{
    widget::{Align, Either, Image, Label, SizedBox, Spinner},
    AppLauncher, ImageBuf, LocalizedString, Widget, WidgetExt, WindowDesc,
};

const WINDOW_TITLE: LocalizedString<WatchoutState> = LocalizedString::new("Watchout");

pub fn launch_app(config: Config) {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = WatchoutState {
        image: ImageBuf::empty(),
        loading: false,
        failure: false,
    };

    let launcher = AppLauncher::with_window(main_window);
    let event_sink = launcher.get_external_handle();

    for path in config.watch_paths() {
        watcher::watch(path.into(), event_sink.clone()).expect("Expect a watchable path");
    }
    let delegate = delegate::Delegate::new(event_sink, config);

    let launcher = launcher.delegate(delegate);
    launcher.launch(initial_state).expect("launch failed");
}

fn build_root_widget() -> impl Widget<WatchoutState> {
    // The image (if we're not loading)
    let image = Image::new(ImageBuf::empty());
    let image = druid::widget::SizedBox::new(image.controller(ImageController)).expand();

    // The waiting if we're loading
    let spinner = SizedBox::new(Spinner::new()).fix_size(64.0, 64.0);
    let spinner = Align::centered(spinner);

    let main_content_load = Either::new(|s, _e| s.loading, spinner, image);

    let error_box = Label::new("Error");
    let main_content_error = Either::new(|s, _e| s.failure, error_box, main_content_load);

    main_content_error
}

struct ImageController;

impl druid::widget::Controller<WatchoutState, Image> for ImageController {
    fn update(
        &mut self,
        child: &mut Image,
        ctx: &mut druid::UpdateCtx,
        _old_data: &WatchoutState,
        data: &WatchoutState,
        _env: &druid::Env,
    ) {
        // Have to perform expensive clone operation here
        child.set_image_data(data.image.clone());
        ctx.children_changed();
    }
}
