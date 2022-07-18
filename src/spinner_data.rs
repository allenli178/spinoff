use crate::{spinner_enum::Spinners, StringLiteral};
use maplit::{self, hashmap};
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// A Struct that contains the data for a spinner.
/// Frames is a Vec of &str, each &str is a frame of the spinner.
/// Interval is the number of milliseconds to wait before moving to the next frame.
pub struct SpinnerFrames {
    pub frames: Vec<StringLiteral>,
    pub interval: u16,
}

/// A HashMap of all the spinners from the Spinners enum and their data.
/// The reason for using a HashMap instead of something like a JSON file is
/// because there's no need for extra files and/or crates for parsing data.
pub static SPINNER_FRAMES: Lazy<HashMap<Spinners, SpinnerFrames>> = Lazy::new(|| {
    hashmap! {
        Spinners::Dots => SpinnerFrames {frames: vec![
            "⠋",
            "⠙",
            "⠹",
            "⠸",
            "⠼",
            "⠴",
            "⠦",
            "⠧",
            "⠇",
            "⠏"
        ], interval: 80},
        Spinners::Dots2 => SpinnerFrames {frames: vec![
            "⠋",
            "⠙",
            "⠚",
            "⠞",
            "⠖",
            "⠦",
            "⠴",
            "⠲",
            "⠳",
            "⠓"
        ], interval: 80},
        Spinners::Dots3 => SpinnerFrames {frames: vec![
            "⠋",
            "⠙",
            "⠚",
            "⠞",
            "⠖",
            "⠦",
            "⠴",
            "⠲",
            "⠳",
            "⠓"
          ], interval: 80},
          Spinners::Dots4 => SpinnerFrames {frames: vec![
            "⠄",
            "⠆",
            "⠇",
            "⠋",
            "⠙",
            "⠸",
            "⠰",
            "⠠",
            "⠰",
            "⠸",
            "⠙",
            "⠋",
            "⠇",
            "⠆"
          ], interval: 80},
          Spinners::Dots5 => SpinnerFrames {frames: vec![
            "⠋",
            "⠙",
            "⠚",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠲",
            "⠴",
            "⠦",
            "⠖",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠓",
            "⠋"
          ], interval: 80},
          Spinners::Dots6 => SpinnerFrames {frames: vec![
            "⠁",
            "⠉",
            "⠙",
            "⠚",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠲",
            "⠴",
            "⠤",
            "⠄",
            "⠄",
            "⠤",
            "⠴",
            "⠲",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠚",
            "⠙",
            "⠉",
            "⠁"
          ], interval: 80},
          Spinners::Dots7 => SpinnerFrames {frames: vec![
            "⠈",
            "⠉",
            "⠋",
            "⠓",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠖",
            "⠦",
            "⠤",
            "⠠",
            "⠠",
            "⠤",
            "⠦",
            "⠖",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠓",
            "⠋",
            "⠉",
            "⠈"
          ], interval: 80},
          Spinners::Dots8 => SpinnerFrames {frames: vec![
            "⠁",
            "⠁",
            "⠉",
            "⠙",
            "⠚",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠲",
            "⠴",
            "⠤",
            "⠄",
            "⠄",
            "⠤",
            "⠠",
            "⠠",
            "⠤",
            "⠦",
            "⠖",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠓",
            "⠋",
            "⠉",
            "⠈",
            "⠈"
          ], interval: 80},
          Spinners::Dots9 => SpinnerFrames {frames: vec![
            "⢹",
            "⢺",
            "⢼",
            "⣸",
            "⣇",
            "⡧",
            "⡗",
            "⡏"
          ], interval: 80},
          Spinners::Dots10 => SpinnerFrames {frames: vec![
            "⢄",
            "⢂",
            "⢁",
            "⡁",
            "⡈",
            "⡐",
            "⡠"
          ], interval: 80},
          Spinners::Dots11 => SpinnerFrames {frames: vec![
            "⠁",
            "⠂",
            "⠄",
            "⡀",
            "⢀",
            "⠠",
            "⠐",
            "⠈"
          ], interval: 100},
          Spinners::Dots12 => SpinnerFrames {frames: vec![
            "⢀⠀",
            "⡀⠀",
            "⠄⠀",
            "⢂⠀",
            "⡂⠀",
            "⠅⠀",
            "⢃⠀",
            "⡃⠀",
            "⠍⠀",
            "⢋⠀",
            "⡋⠀",
            "⠍⠁",
            "⢋⠁",
            "⡋⠁",
            "⠍⠉",
            "⠋⠉",
            "⠋⠉",
            "⠉⠙",
            "⠉⠙",
            "⠉⠩",
            "⠈⢙",
            "⠈⡙",
            "⢈⠩",
            "⡀⢙",
            "⠄⡙",
            "⢂⠩",
            "⡂⢘",
            "⠅⡘",
            "⢃⠨",
            "⡃⢐",
            "⠍⡐",
            "⢋⠠",
            "⡋⢀",
            "⠍⡁",
            "⢋⠁",
            "⡋⠁",
            "⠍⠉",
            "⠋⠉",
            "⠋⠉",
            "⠉⠙",
            "⠉⠙",
            "⠉⠩",
            "⠈⢙",
            "⠈⡙",
            "⠈⠩",
            "⠀⢙",
            "⠀⡙",
            "⠀⠩",
            "⠀⢘",
            "⠀⡘",
            "⠀⠨",
            "⠀⢐",
            "⠀⡐",
            "⠀⠠",
            "⠀⢀",
            "⠀⡀"
          ], interval: 80},
          Spinners::Dots8Bit => SpinnerFrames {frames: vec![
            "⠀",
            "⠁",
            "⠂",
            "⠃",
            "⠄",
            "⠅",
            "⠆",
            "⠇",
            "⡀",
            "⡁",
            "⡂",
            "⡃",
            "⡄",
            "⡅",
            "⡆",
            "⡇",
            "⠈",
            "⠉",
            "⠊",
            "⠋",
            "⠌",
            "⠍",
            "⠎",
            "⠏",
            "⡈",
            "⡉",
            "⡊",
            "⡋",
            "⡌",
            "⡍",
            "⡎",
            "⡏",
            "⠐",
            "⠑",
            "⠒",
            "⠓",
            "⠔",
            "⠕",
            "⠖",
            "⠗",
            "⡐",
            "⡑",
            "⡒",
            "⡓",
            "⡔",
            "⡕",
            "⡖",
            "⡗",
            "⠘",
            "⠙",
            "⠚",
            "⠛",
            "⠜",
            "⠝",
            "⠞",
            "⠟",
            "⡘",
            "⡙",
            "⡚",
            "⡛",
            "⡜",
            "⡝",
            "⡞",
            "⡟",
            "⠠",
            "⠡",
            "⠢",
            "⠣",
            "⠤",
            "⠥",
            "⠦",
            "⠧",
            "⡠",
            "⡡",
            "⡢",
            "⡣",
            "⡤",
            "⡥",
            "⡦",
            "⡧",
            "⠨",
            "⠩",
            "⠪",
            "⠫",
            "⠬",
            "⠭",
            "⠮",
            "⠯",
            "⡨",
            "⡩",
            "⡪",
            "⡫",
            "⡬",
            "⡭",
            "⡮",
            "⡯",
            "⠰",
            "⠱",
            "⠲",
            "⠳",
            "⠴",
            "⠵",
            "⠶",
            "⠷",
            "⡰",
            "⡱",
            "⡲",
            "⡳",
            "⡴",
            "⡵",
            "⡶",
            "⡷",
            "⠸",
            "⠹",
            "⠺",
            "⠻",
            "⠼",
            "⠽",
            "⠾",
            "⠿",
            "⡸",
            "⡹",
            "⡺",
            "⡻",
            "⡼",
            "⡽",
            "⡾",
            "⡿",
            "⢀",
            "⢁",
            "⢂",
            "⢃",
            "⢄",
            "⢅",
            "⢆",
            "⢇",
            "⣀",
            "⣁",
            "⣂",
            "⣃",
            "⣄",
            "⣅",
            "⣆",
            "⣇",
            "⢈",
            "⢉",
            "⢊",
            "⢋",
            "⢌",
            "⢍",
            "⢎",
            "⢏",
            "⣈",
            "⣉",
            "⣊",
            "⣋",
            "⣌",
            "⣍",
            "⣎",
            "⣏",
            "⢐",
            "⢑",
            "⢒",
            "⢓",
            "⢔",
            "⢕",
            "⢖",
            "⢗",
            "⣐",
            "⣑",
            "⣒",
            "⣓",
            "⣔",
            "⣕",
            "⣖",
            "⣗",
            "⢘",
            "⢙",
            "⢚",
            "⢛",
            "⢜",
            "⢝",
            "⢞",
            "⢟",
            "⣘",
            "⣙",
            "⣚",
            "⣛",
            "⣜",
            "⣝",
            "⣞",
            "⣟",
            "⢠",
            "⢡",
            "⢢",
            "⢣",
            "⢤",
            "⢥",
            "⢦",
            "⢧",
            "⣠",
            "⣡",
            "⣢",
            "⣣",
            "⣤",
            "⣥",
            "⣦",
            "⣧",
            "⢨",
            "⢩",
            "⢪",
            "⢫",
            "⢬",
            "⢭",
            "⢮",
            "⢯",
            "⣨",
            "⣩",
            "⣪",
            "⣫",
            "⣬",
            "⣭",
            "⣮",
            "⣯",
            "⢰",
            "⢱",
            "⢲",
            "⢳",
            "⢴",
            "⢵",
            "⢶",
            "⢷",
            "⣰",
            "⣱",
            "⣲",
            "⣳",
            "⣴",
            "⣵",
            "⣶",
            "⣷",
            "⢸",
            "⢹",
            "⢺",
            "⢻",
            "⢼",
            "⢽",
            "⢾",
            "⢿",
            "⣸",
            "⣹",
            "⣺",
            "⣻",
            "⣼",
            "⣽",
            "⣾",
            "⣿"
          ], interval: 80},
          Spinners::Line => SpinnerFrames {frames: vec![
            "-",
            "\\",
            "|",
            "/"
          ], interval: 130},
          Spinners::Line2 => SpinnerFrames {frames: vec![
            "⠂",
            "-",
            "–",
            "—",
            "–",
            "-"
          ], interval: 100},
          Spinners::Pipe => SpinnerFrames {frames: vec![
            "┤",
            "┘",
            "┴",
            "└",
            "├",
            "┌",
            "┬",
            "┐"
          ], interval: 100},
          Spinners::SimpleDots => SpinnerFrames {frames: vec![
            ".  ",
            ".. ",
            "...",
            "   "
          ], interval: 400},
          Spinners::SimpleDotsScrolling => SpinnerFrames {frames: vec![
            ".  ",
            ".. ",
            "...",
            " ..",
            "  .",
            "   "
          ], interval: 200},
          Spinners::Star => SpinnerFrames {frames: vec![
            "✶",
            "✸",
            "✹",
            "✺",
            "✹",
            "✷"
          ], interval: 70},
          Spinners::Star2 => SpinnerFrames {frames: vec![
            "+",
            "x",
            "*"
          ], interval: 80},
          Spinners::Flip => SpinnerFrames {frames: vec![
            "_",
            "_",
            "_",
            "-",
            "`",
            "`",
            "'",
            "´",
            "-",
            "_",
            "_",
            "_"
          ], interval: 70},
          Spinners::Hamburger => SpinnerFrames {frames: vec![
            "☱",
            "☲",
            "☴"
          ], interval: 100},
          Spinners::GrowVertical => SpinnerFrames {frames: vec![
            "▁",
            "▃",
            "▄",
            "▅",
            "▆",
            "▇",
            "▆",
            "▅",
            "▄",
            "▃"
          ], interval: 120},
          Spinners::GrowHorizontal => SpinnerFrames {frames: vec![
            "▏",
            "▎",
            "▍",
            "▌",
            "▋",
            "▊",
            "▉",
            "▊",
            "▋",
            "▌",
            "▍",
            "▎"
          ], interval: 120},
          Spinners::Balloon => SpinnerFrames {frames: vec![
            " ",
            ".",
            "o",
            "O",
            "@",
            "*",
            " "
          ], interval: 140},
          Spinners::Balloon2 => SpinnerFrames {frames: vec![
            ".",
            "o",
            "O",
            "°",
            "O",
            "o",
            "."
          ], interval: 120},
          Spinners::Noise => SpinnerFrames {frames: vec![
            "▓",
            "▒",
            "░"
          ], interval: 100},
          Spinners::Bounce => SpinnerFrames {frames: vec![
            "⠁",
            "⠂",
            "⠄",
            "⠂"
          ], interval: 120},
          Spinners::BoxBounce => SpinnerFrames {frames: vec![
            "▖",
            "▘",
            "▝",
            "▗"
          ], interval: 120},
          Spinners::BoxBounce2 => SpinnerFrames {frames: vec![
            "▌",
            "▀",
            "▐",
            "▄"
          ], interval: 100},
          Spinners::Triangle => SpinnerFrames {frames: vec![
            "◢",
            "◣",
            "◤",
            "◥"
          ], interval: 50},
          Spinners::Arc => SpinnerFrames {frames: vec![
            "◜",
            "◠",
            "◝",
            "◞",
            "◡",
            "◟"
          ], interval: 100},
          Spinners::Circle => SpinnerFrames {frames: vec![
            "◡",
            "⊙",
            "◠"
          ], interval: 120},
          Spinners::SquareCorners => SpinnerFrames {frames: vec![
            "◰",
            "◳",
            "◲",
            "◱"
          ], interval: 180},
          Spinners::CircleQuarters => SpinnerFrames {frames: vec![
            "◴",
            "◷",
            "◶",
            "◵"
          ], interval: 120},
          Spinners::CircleHalves => SpinnerFrames {frames: vec![
            "◐",
            "◓",
            "◑",
            "◒"
          ], interval: 50},
          Spinners::Squish => SpinnerFrames {frames: vec![
            "╫",
            "╪"
          ], interval: 100},
          Spinners::Toggle => SpinnerFrames {frames: vec![
            "⊶",
            "⊷"
          ], interval: 250},
          Spinners::Toggle2 => SpinnerFrames {frames: vec![
            "▫",
            "▪"
          ], interval: 80},
          Spinners::Toggle3 => SpinnerFrames {frames: vec![
            "□",
            "■"
          ], interval: 120},
          Spinners::Toggle4 => SpinnerFrames {frames: vec![
            "■",
            "□",
            "▪",
            "▫"
          ], interval: 100},
          Spinners::Toggle5 => SpinnerFrames {frames: vec![
            "▮",
            "▯"
          ], interval: 100},
          Spinners::Toggle6 => SpinnerFrames {frames: vec![
            "ဝ",
            "၀"
          ], interval: 300},
          Spinners::Toggle7 => SpinnerFrames {frames: vec![
            "⦾",
            "⦿"
          ], interval: 80},
          Spinners::Toggle8 => SpinnerFrames {frames: vec![
            "◍",
            "◌"
          ], interval: 100},
          Spinners::Toggle9 => SpinnerFrames {frames: vec![
            "◉",
            "◎"
          ], interval: 100},
          Spinners::Toggle10 => SpinnerFrames {frames: vec![
            "㊂",
            "㊀",
            "㊁"
          ], interval: 100},
          Spinners::Toggle11 => SpinnerFrames {frames: vec![
            "⧇",
            "⧆"
          ], interval: 50},
          Spinners::Toggle12 => SpinnerFrames {frames: vec![
            "☗",
            "☖"
          ], interval: 120},
          Spinners::Toggle13 => SpinnerFrames {frames: vec![
            "=",
            "*",
            "-"
          ], interval: 80},
          Spinners::Arrow => SpinnerFrames {frames: vec![
            "←",
            "↖",
            "↑",
            "↗",
            "→",
            "↘",
            "↓",
            "↙"
          ], interval: 100},
          Spinners::Arrow2 => SpinnerFrames {frames: vec![
            "⬆️ ",
            "↗️ ",
            "➡️ ",
            "↘️ ",
            "⬇️ ",
            "↙️ ",
            "⬅️ ",
            "↖️ "
          ], interval: 80},
          Spinners::Arrow3 => SpinnerFrames {frames: vec![
            "▹▹▹▹▹",
            "▸▹▹▹▹",
            "▹▸▹▹▹",
            "▹▹▸▹▹",
            "▹▹▹▸▹",
            "▹▹▹▹▸"
          ], interval: 120},
          Spinners::BouncingBar => SpinnerFrames {frames: vec![
            "[    ]",
            "[=   ]",
            "[==  ]",
            "[=== ]",
            "[ ===]",
            "[  ==]",
            "[   =]",
            "[    ]",
            "[   =]",
            "[  ==]",
            "[ ===]",
            "[====]",
            "[=== ]",
            "[==  ]",
            "[=   ]"
          ], interval: 80},
          Spinners::BouncingBall => SpinnerFrames {frames: vec![
            "( ●    )",
            "(  ●   )",
            "(   ●  )",
            "(    ● )",
            "(     ●)",
            "(    ● )",
            "(   ●  )",
            "(  ●   )",
            "( ●    )",
            "(●     )"
          ], interval: 80},
          Spinners::Smiley => SpinnerFrames {frames: vec![
            "😄 ",
            "😝 "
          ], interval: 200},
          Spinners::Monkey => SpinnerFrames {frames: vec![
            "🙈 ",
            "🙈 ",
            "🙉 ",
            "🙊 "
          ], interval: 300},
          Spinners::Hearts => SpinnerFrames {frames: vec![
            "💛 ",
            "💙 ",
            "💜 ",
            "💚 ",
            "❤️ "
          ], interval: 100},
          Spinners::Clock => SpinnerFrames {frames: vec![
            "🕛 ",
            "🕐 ",
            "🕑 ",
            "🕒 ",
            "🕓 ",
            "🕔 ",
            "🕕 ",
            "🕖 ",
            "🕗 ",
            "🕘 ",
            "🕙 ",
            "🕚 "
          ], interval: 100},
          Spinners::Earth => SpinnerFrames {frames: vec![
            "🌍 ",
            "🌎 ",
            "🌏 "
          ], interval: 180},
          Spinners::Material => SpinnerFrames {frames: vec![
            "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "███████▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "████████▁▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "██████████▁▁▁▁▁▁▁▁▁▁",
            "███████████▁▁▁▁▁▁▁▁▁",
            "█████████████▁▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁▁██████████████▁▁▁▁",
            "▁▁▁██████████████▁▁▁",
            "▁▁▁▁█████████████▁▁▁",
            "▁▁▁▁██████████████▁▁",
            "▁▁▁▁██████████████▁▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁▁██████████████",
            "▁▁▁▁▁▁██████████████",
            "▁▁▁▁▁▁▁█████████████",
            "▁▁▁▁▁▁▁█████████████",
            "▁▁▁▁▁▁▁▁████████████",
            "▁▁▁▁▁▁▁▁████████████",
            "▁▁▁▁▁▁▁▁▁███████████",
            "▁▁▁▁▁▁▁▁▁███████████",
            "▁▁▁▁▁▁▁▁▁▁██████████",
            "▁▁▁▁▁▁▁▁▁▁██████████",
            "▁▁▁▁▁▁▁▁▁▁▁▁████████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁██████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "████████▁▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "███████████▁▁▁▁▁▁▁▁▁",
            "████████████▁▁▁▁▁▁▁▁",
            "████████████▁▁▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁▁▁█████████████▁▁▁▁",
            "▁▁▁▁▁████████████▁▁▁",
            "▁▁▁▁▁████████████▁▁▁",
            "▁▁▁▁▁▁███████████▁▁▁",
            "▁▁▁▁▁▁▁▁█████████▁▁▁",
            "▁▁▁▁▁▁▁▁█████████▁▁▁",
            "▁▁▁▁▁▁▁▁▁█████████▁▁",
            "▁▁▁▁▁▁▁▁▁█████████▁▁",
            "▁▁▁▁▁▁▁▁▁▁█████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁"
          ], interval: 17},
          Spinners::Moon => SpinnerFrames {frames: vec![
            "🌑 ",
            "🌒 ",
            "🌓 ",
            "🌔 ",
            "🌕 ",
            "🌖 ",
            "🌗 ",
            "🌘 "
          ], interval: 80},
          Spinners::Runner => SpinnerFrames {frames: vec![
            "🚶 ",
            "🏃 "
          ], interval: 140},
          Spinners::Pong => SpinnerFrames {frames: vec![
            "▐⠂       ▌",
            "▐⠈       ▌",
            "▐ ⠂      ▌",
            "▐ ⠠      ▌",
            "▐  ⡀     ▌",
            "▐  ⠠     ▌",
            "▐   ⠂    ▌",
            "▐   ⠈    ▌",
            "▐    ⠂   ▌",
            "▐    ⠠   ▌",
            "▐     ⡀  ▌",
            "▐     ⠠  ▌",
            "▐      ⠂ ▌",
            "▐      ⠈ ▌",
            "▐       ⠂▌",
            "▐       ⠠▌",
            "▐       ⡀▌",
            "▐      ⠠ ▌",
            "▐      ⠂ ▌",
            "▐     ⠈  ▌",
            "▐     ⠂  ▌",
            "▐    ⠠   ▌",
            "▐    ⡀   ▌",
            "▐   ⠠    ▌",
            "▐   ⠂    ▌",
            "▐  ⠈     ▌",
            "▐  ⠂     ▌",
            "▐ ⠠      ▌",
            "▐ ⡀      ▌",
            "▐⠠       ▌"
          ], interval: 80},
          Spinners::Shark => SpinnerFrames {frames: vec![
            "▐|\\____________▌",
            "▐_|\\___________▌",
            "▐__|\\__________▌",
            "▐___|\\_________▌",
            "▐____|\\________▌",
            "▐_____|\\_______▌",
            "▐______|\\______▌",
            "▐_______|\\_____▌",
            "▐________|\\____▌",
            "▐_________|\\___▌",
            "▐__________|\\__▌",
            "▐___________|\\_▌",
            "▐____________|\\▌",
            "▐____________/|▌",
            "▐___________/|_▌",
            "▐__________/|__▌",
            "▐_________/|___▌",
            "▐________/|____▌",
            "▐_______/|_____▌",
            "▐______/|______▌",
            "▐_____/|_______▌",
            "▐____/|________▌",
            "▐___/|_________▌",
            "▐__/|__________▌",
            "▐_/|___________▌",
            "▐/|____________▌"
          ], interval: 120},
          Spinners::Dqpb => SpinnerFrames {frames: vec![
            "d",
            "q",
            "p",
            "b"
          ], interval: 100},
          Spinners::Weather => SpinnerFrames {frames: vec![
            "☀️ ",
            "☀️ ",
            "☀️ ",
            "🌤 ",
            "⛅️ ",
            "🌥 ",
            "☁️ ",
            "🌧 ",
            "🌨 ",
            "🌧 ",
            "🌨 ",
            "🌧 ",
            "🌨 ",
            "⛈ ",
            "🌨 ",
            "🌧 ",
            "🌨 ",
            "☁️ ",
            "🌥 ",
            "⛅️ ",
            "🌤 ",
            "☀️ ",
            "☀️ "
          ], interval: 100},
          Spinners::Christmas => SpinnerFrames {frames: vec![
            "🌲",
            "🎄"
          ], interval: 400},
          Spinners::Grenade => SpinnerFrames {frames: vec![
            "،  ",
            "′  ",
            " ´ ",
            " ‾ ",
            "  ⸌",
            "  ⸊",
            "  |",
            "  ⁎",
            "  ⁕",
            " ෴ ",
            "  ⁓",
            "   ",
            "   ",
            "   "
          ], interval: 80},
          Spinners::Point => SpinnerFrames {frames: vec![
            "∙∙∙",
            "●∙∙",
            "∙●∙",
            "∙∙●",
            "∙∙∙"
          ], interval: 125},
          Spinners::Layer => SpinnerFrames {frames: vec![
            "-",
            "=",
            "≡"
          ], interval: 150},
          Spinners::BetaWave => SpinnerFrames {frames: vec![
            "ρββββββ",
            "βρβββββ",
            "ββρββββ",
            "βββρβββ",
            "ββββρββ",
            "βββββρβ",
            "ββββββρ"
          ], interval: 80},
          Spinners::FingerDance => SpinnerFrames {frames: vec![
            "🤘 ",
            "🤟 ",
            "🖖 ",
            "✋ ",
            "🤚 ",
            "👆 "
          ], interval: 160},
          Spinners::FistBump => SpinnerFrames {frames: vec![
            "🤜　　　　🤛 ",
            "🤜　　　　🤛 ",
            "🤜　　　　🤛 ",
            "　🤜　　🤛　 ",
            "　　🤜🤛　　 ",
            "　🤜✨🤛　　 ",
            "🤜　✨　🤛　 "
          ], interval: 80},
          Spinners::SoccerHeader => SpinnerFrames {frames: vec![
            " 🧑⚽️       🧑 ",
            "🧑  ⚽️      🧑 ",
            "🧑   ⚽️     🧑 ",
            "🧑    ⚽️    🧑 ",
            "🧑     ⚽️   🧑 ",
            "🧑      ⚽️  🧑 ",
            "🧑       ⚽️🧑  ",
            "🧑      ⚽️  🧑 ",
            "🧑     ⚽️   🧑 ",
            "🧑    ⚽️    🧑 ",
            "🧑   ⚽️     🧑 ",
            "🧑  ⚽️      🧑 "
          ], interval: 80},
          Spinners::Mindblown => SpinnerFrames {frames: vec![
            "😐 ",
            "😐 ",
            "😮 ",
            "😮 ",
            "😦 ",
            "😦 ",
            "😧 ",
            "😧 ",
            "🤯 ",
            "💥 ",
            "✨ ",
            "　 ",
            "　 ",
            "　 "
          ], interval: 160},
          Spinners::Speaker => SpinnerFrames {frames: vec![
            "🔈 ",
            "🔉 ",
            "🔊 ",
            "🔉 "
          ], interval: 160},
          Spinners::OrangePulse => SpinnerFrames {frames: vec![
            "🔸 ",
            "🔶 ",
            "🟠 ",
            "🟠 ",
            "🔶 "
          ], interval: 100},
          Spinners::BluePulse => SpinnerFrames {frames: vec![
            "🔹 ",
            "🔷 ",
            "🔵 ",
            "🔵 ",
            "🔷 "
          ], interval: 100},
          Spinners::OrangeBluePulse => SpinnerFrames {frames: vec![
            "🔸 ",
            "🔶 ",
            "🟠 ",
            "🟠 ",
            "🔶 ",
            "🔹 ",
            "🔷 ",
            "🔵 ",
            "🔵 ",
            "🔷 "
          ], interval: 100},
          Spinners::TimeTravel => SpinnerFrames {frames: vec![
            "🕛 ",
            "🕚 ",
            "🕙 ",
            "🕘 ",
            "🕗 ",
            "🕖 ",
            "🕕 ",
            "🕔 ",
            "🕓 ",
            "🕒 ",
            "🕑 ",
            "🕐 "
          ], interval: 100},
          Spinners::Aesthetic => SpinnerFrames {frames: vec![
            "▰▱▱▱▱▱▱",
            "▰▰▱▱▱▱▱",
            "▰▰▰▱▱▱▱",
            "▰▰▰▰▱▱▱",
            "▰▰▰▰▰▱▱",
            "▰▰▰▰▰▰▱",
            "▰▰▰▰▰▰▰",
            "▰▱▱▱▱▱▱"
          ], interval: 80}
    }
});