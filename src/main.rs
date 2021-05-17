mod app;
mod cgmath;
mod raytracer;
mod utils;

fn init_logger() {
    use env_logger::*;

    let env = Env::default().default_filter_or("info");
    Builder::from_env(env).init();
}

fn main() {
    init_logger();

    eframe::run_native(Box::new(app::App::default()), Default::default());
}
