use std::{thread, time::Duration};
use getopts::{Matches, Options};
use notify_rust::Notification;
use thread::JoinHandle;
use std::env;

#[derive(Default)]
struct RNotification {
    delay: u64,
    title: String,
    message: String,
    icon: String,
}

impl From<&Matches> for RNotification {
    fn from(matches: &Matches) -> Self {
        let mut delay = 0;
        if matches.opt_present("delay") {
            delay = matches.opt_get("delay").unwrap().unwrap();
        }

        Self {
            delay,
            title: matches.opt_str("title").unwrap_or_default(),
            message: matches.opt_str("message").unwrap_or_default(),
            icon: matches.opt_str("icon").unwrap_or_default(),
        }
    }
}

fn spawn_countdown_thread(notif: RNotification) -> JoinHandle<()> {
    thread::spawn(move || {
        thread::sleep(Duration::new(notif.delay, 0));
        Notification::new()
            .summary(&notif.title)
            .body(&notif.message)
            .icon(&notif.icon)
            .appname("rnotify")
            .show().unwrap();
    })
}

fn print_usage(prog: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", prog);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    opts.optopt("d", "delay", "How long the notification should be delayed", "SECONDS");
    opts.optopt("t", "title", "The notification title", "TITLE");
    opts.optopt("m", "message", "The notification message", "MESSAGE");
    opts.optopt("i", "icon", "The notification icon", "ICON");

    let args: Vec<String> = env::args().collect();
    let prog = args[0].clone();

    let matches = match opts.parse(&args) {
        Ok(m) => m,
        Err(f) => panic!(f.to_owned()),
    };

    if matches.opt_present("help") || !matches.opt_present("message") {
        print_usage(&prog, &opts);
        return;
    }

    let notif = RNotification::from(&matches);
    let thread = spawn_countdown_thread(notif);

    thread.join().unwrap();
}
