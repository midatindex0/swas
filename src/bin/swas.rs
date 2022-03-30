use clap::{Arg, Command};
use colored::*;

fn cli() -> Command<'static> {
    Command::new("swas")
        .about(
            "CLI tool  that tells you when swas will upload new video through complex calculations. It also lets you search and play youtube videos of swas and other channels. Searching about youtube channels is also an option. Basically it's a youtube search cli tool written in rust."
        )
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("search")
                .about("Searches a video or channel by its title/name")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("unswas")
                        .long("unswas")
                        .short('u')
                        .help("View results from creators other than swas as well (Did you think this will get rid of swas? No way!)")
                        .takes_value(false),
                ).subcommand(
                    Command::new("video")
                        .about("Lets you seach videos (title optional)")
                        .arg(
                            Arg::new("TITLE")
                                .default_value("videos")
                                .help("Video title")
                        ).arg(
                            Arg::new("channel")
                                .long("channel")
                                .short('c')
                                .takes_value(true)
                                .help("Searches videos from a specific channel")
                        )
                ).subcommand(
                    Command::new("channel")
                        .about("Lets you search channels (name optional)")
                        .arg(
                            Arg::new("CHANNEL_NAME")
                                .default_value("Code With Swastik")
                                .help("Channel name")
                        )
                )
        )
}

fn unswas_video_without_channel(title: &str, page: u8) {
    for item in swas::get_data(&format!("{}", title), page) {
        match item {
            swas::Item::Channel(_) => {}
            swas::Item::Playlist(playlist, uploader) => println!(
                "{} by {} [contains {} videos]\n----> {}\n",
                playlist.title.truecolor(165, 252, 137).bold(),
                uploader.username.truecolor(252, 121, 123),
                playlist.video_count.truecolor(153, 165, 252).bold(),
                playlist.url.truecolor(252, 224, 181).bold()
            ),
            swas::Item::Video(video, uploader) => println!(
                "[{}]{} by {} ({}, uploaded {})\n----> {}\n",
                video.duration.truecolor(146, 215, 252),
                video.title.truecolor(199, 252, 146),
                uploader.username.truecolor(252, 121, 123),
                video.views.truecolor(153, 165, 252).bold(),
                video.upload_date.truecolor(153, 165, 252).bold(),
                video.url.truecolor(252, 224, 181).bold()
            ),
            swas::Item::Unknown => {}
        };
    }
}

fn unswas_video_with_channel(title: &str, channel: &str, page: u8) {
    for item in swas::get_data(&format!("{} {}", title, channel), page) {
        match item {
            swas::Item::Channel(_) => {}
            swas::Item::Playlist(playlist, uploader) => {
                if uploader.username.eq(channel) {
                    println!(
                        "{} by {} [contains {} videos]\n----> {}\n",
                        playlist.title.truecolor(165, 252, 137).bold(),
                        uploader.username.truecolor(252, 121, 123),
                        playlist.video_count.truecolor(153, 165, 252).bold(),
                        playlist.url.truecolor(252, 224, 181).bold()
                    );
                }
            }
            swas::Item::Video(video, uploader) => {
                if uploader.username.eq(channel) {
                    println!(
                        "[{}]{} by {} ({}, uploaded {})\n----> {}\n",
                        video.duration.truecolor(146, 215, 252),
                        video.title.truecolor(199, 252, 146),
                        uploader.username.truecolor(252, 121, 123),
                        video.views.truecolor(153, 165, 252).bold(),
                        video.upload_date.truecolor(153, 165, 252).bold(),
                        video.url.truecolor(252, 224, 181).bold()
                    )
                }
            }
            swas::Item::Unknown => {}
        };
    }
}

fn unswas_channel(channel: &str, page: u8) {
    for items in swas::get_data(channel, page) {
        match items {
            swas::Item::Channel(channel) => {
                println!("\nChannel ID: {}\nChannel name: {}\nChannel description: {}\nVideo count: {}\nSubscriber count: {}\nVerified: {}\nURL: {}\n",
                    channel.id.truecolor(146, 215, 252),
                    channel.title.truecolor(199, 252, 146).bold(),
                    channel.snippet.truecolor(146, 215, 252),
                    channel.video_count.truecolor(153, 165, 252).bold(),
                    channel.subscriber_count.truecolor(153, 165, 252).bold(),
                    if channel.verified { "yes".bright_green().bold() } else { "no".bright_red().bold() },
                    channel.url.truecolor(252, 224, 181).bold()
                );
            }
            _ => {}
        }
    }
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("search", sub_matches)) => match sub_matches.subcommand() {
            Some(("video", sub_sub_matches)) => {
                if sub_matches.is_present("unswas") {
                    if sub_sub_matches.is_present("channel") {
                        unswas_video_with_channel(
                            sub_sub_matches.value_of("TITLE").expect("Required"),
                            sub_sub_matches.value_of("channel").expect("Required"),
                            1,
                        );
                    } else {
                        unswas_video_without_channel(
                            sub_sub_matches.value_of("TITLE").expect("Required"),
                            1,
                        );
                    }
                } else {
                    unswas_video_with_channel(
                        sub_sub_matches.value_of("TITLE").expect("Required"),
                        "Code With Swastik",
                        1,
                    );
                }
            }
            Some(("channel", sub_sub_matches)) => {
                if sub_matches.is_present("unswas") {
                    unswas_channel(
                        sub_sub_matches.value_of("CHANNEL_NAME").expect("Required"),
                        1,
                    );
                } else {
                    unswas_channel("Code With Swastik", 1);
                }
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
