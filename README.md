# Swas

[![version](https://img.shields.io/crates/v/swas.svg)](https://crates.io/crates/swas)

Introducing the greatest rust library + CLI tool in the entire history of rust.

Swas is a rust library + CLI tool  that tells you when swas will upload new video through complex calculations. It also lets you search and play youtube videos of swas and other channels. Searching about youtube channels is also an option. Basically it's a youtube search cli tool written in rust.

Docs:  https://docs.rs/swas/latest/swas/ 

## Current features
- search youtube videos
- search youtube channels
- search swas specific info

## In progress
- `swas upload when` commnd
- `swas play` command
- Support for swas.py channel
- Better and interactive UI

## Furure plans
- A dedicated scraper to scrap youtube search results

## Installation

Install the CLI tool using cargo:

`cargo install swas`

Install library as dependency:

`swas = "0.3.0"`

Install and build from github repo:

```sh
git clone https://github.com/lonely-code-cube/swas
cd swas
cargo build --release
cargo run -- --help
```

## CLI usage
- Help: `swas --help`
- Search swas's videos: `swas search video`
  
  !["video search img"](/assets/img1.png)
- Search swas's videos by title: `swas search video "Select Menus in Discord.py & Pycord"`
  
  !["video search img"](/assets/img2.png)
- Search any video: `swas search --unswas video` or `swas search -u video`
  !["video search img"](/assets/img3.png)
- Search any video by title: `swas search --unswas video "Yoku"` or `swas search -u video "Yoku"`
  
  !["video search img"](/assets/img4.png)
- Search videos of a specific channel: `swas search --unswas video --channel "E ve"` or `swas search -u video -c "E ve"`
  
  !["video search img"](/assets/img5.png)
- Search videos of a specific channel by title: `swas search --unswas video "Yoku" --channel "E ve"` or `swas search -u video "Yoku" -c "E ve"`
  
  !["video search img"](/assets/img6.png)
- Search swas's channel: `swas search channel`
- Search any channel: `swas search --unswas channel "E ve"` or `swas search -u channel "E ve"`
  
  !["video search img"](/assets/img7.png)

## Library usage
To be documented