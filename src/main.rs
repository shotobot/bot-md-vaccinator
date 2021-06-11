mod shotobot;
mod conf;
mod bot;
mod logging;
mod endpoint;
mod alert;
mod notify;
mod pause;

fn main() {
    conf::verify();
    logging::init();
    bot::run();
}

