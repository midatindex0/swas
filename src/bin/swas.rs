use clap::{arg, Arg, Command};

fn cli() -> Command<'static> {
    Command::new("swas")
        .about(
            "A CLI tool that tells you when swas will upload new video and also lets you search or play his videos\n(also videos from other channels if you prefer that)",
        )
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("search")
                .about("Searches a video by its title")
                .arg(arg! (<TITLE> "Video title"))
                .arg_required_else_help(true)
                .arg(
                    Arg::new("unswas")
                        .long("unswas")
                        .help("View results from creators other than swas as well (Did you think this will get rid of swas's videos? No way!")
                        .takes_value(false),
                ),
        ).subcommand(
            Command::new("channel")
                .about("Shows info about swas's youtube channel (also other channels in you prefer that)")
                .args(channel_args()),
        )
}

fn channel_args() -> Vec<clap::Arg<'static>> {
    vec![arg!(--unswas <CHANNEL_NAME>).required(false)]
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            if sub_matches.is_present("unswas") {
                for item in swas::get_data(sub_matches.value_of("TITLE").expect("Required"), 1) {
                    match item {
                        swas::Item::Channel(_) => {}
                        swas::Item::Playlist(playlist, uploader) => println!(
                            "{} by {} [contains {} videos]\nLnk -> {}\n",
                            playlist.title, uploader.username, playlist.video_count, playlist.url
                        ),
                        swas::Item::Video(video, uploader) => println!(
                            "[{}]{} by {} ({}, uploaded {})\nLink -> {}\n",
                            video.duration,
                            video.title,
                            uploader.username,
                            video.views,
                            video.upload_date,
                            video.url
                        ),
                        swas::Item::Unknown => {}
                    };
                }
            } else {
                for item in swas::get_data(
                    &format!(
                        "Code With Swastik {}",
                        sub_matches.value_of("TITLE").expect("Required")
                    ),
                    1,
                ) {
                    match item {
                        swas::Item::Channel(_) => {}
                        swas::Item::Playlist(playlist, uploader) => {
                            if uploader.username.eq("Code With Swastik") {
                                println!(
                                    "{} by {} [contains {} videos]\nLink -> {}\n",
                                    playlist.title,
                                    uploader.username,
                                    playlist.video_count,
                                    playlist.url
                                );
                            }
                        }
                        swas::Item::Video(video, uploader) => {
                            if uploader.username.eq("Code With Swastik") {
                                println!(
                                    "[{}]{} by {} ({}, uploaded {})\nLink -> {}\n",
                                    video.duration,
                                    video.title,
                                    uploader.username,
                                    video.views,
                                    video.upload_date,
                                    video.url
                                )
                            }
                        }
                        swas::Item::Unknown => {}
                    };
                }
            }
        }
        Some(("channel", sub_matches)) => {
            if !sub_matches.is_present("unswas") {
                match &swas::get_data("Code with swastik", 1)[0] {
                    swas::Item::Channel(channel) => {
                        println!("Channel ID: {}\nChannel name: {}\nChannel description: {}\nVideo count: {}\nSubscriber count: {}\nVerified: {}", channel.id, channel.title, channel.snippet, channel.video_count, channel.subscriber_count, channel.verified);
                    }
                    _ => {}
                }
            } else {
                let channel_name = sub_matches.value_of("unswas");
                for items in swas::get_data(channel_name.unwrap(), 1) {
                    match items {
                        swas::Item::Channel(channel) => {
                            println!("Channel ID: {}\nChannel name: {}\nChannel description: {}\nVideo count: {}\nSubscriber count: {}\nVerified: {}", channel.id, channel.title, channel.snippet, channel.video_count, channel.subscriber_count, channel.verified);
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => unreachable!(),
    }
}
