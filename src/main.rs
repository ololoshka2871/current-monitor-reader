mod report;

use std::{
    collections::{BTreeMap, HashMap},
    num::ParseIntError,
};

use hidapi::{self};

use structmap::ToMap;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Read values and output to stdout.
    Use livechart (https://github.com/sevko/livechart) [python2] to plot.
    If you have problems with Qt update \"matplotlib == 1.5.3\" in setup.py
    Break - Ctrl + C, don't close window by [X]"
)]
struct Cli {
    /// Device VID
    #[structopt(default_value="0x0483", long, parse(try_from_str = parse_hex_or_dec))]
    vid: u16,

    /// Device PID
    #[structopt(default_value="0x5789", long, parse(try_from_str = parse_hex_or_dec))]
    pid: u16,

    /// Show voltage
    #[structopt(short)]
    voltage: bool,

    /// Show current
    #[structopt(short)]
    current: bool,

    /// Show power
    #[structopt(short)]
    power: bool,

    /// Show shunt voltage
    #[structopt(short)]
    shunt_voltage: bool,
}

fn main() {
    let args = Cli::from_args();

    let api = hidapi::HidApi::new().expect("Failed to init HIDAPI");
    let device = api.open(args.vid, args.pid).expect(
        format!(
            "Failed to open device {:#06x}:{:#06x}, permisions?",
            args.vid, args.pid
        )
        .as_str(),
    );

    // Read data from device
    let mut buf = [0u8; 8];
    let mut skipinfo: HashMap<String, bool> = HashMap::new();
    {
        skipinfo.insert("voltage".into(), args.voltage);
        skipinfo.insert("shunt_voltage".into(), args.shunt_voltage);
        skipinfo.insert("current".into(), args.current);
        skipinfo.insert("power".into(), args.power);
    }
    if skipinfo.iter().filter(|(_, v)| **v).count() == 0 {
        eprintln!("Output format not selected, assuming -vc");

        *skipinfo.get_mut("voltage".into()).unwrap() = true;
        *skipinfo.get_mut("current".into()).unwrap() = true;
    }

    loop {
        let _ = device
            .read(&mut buf[..])
            .expect("Failed to read HID report");
        let report = report::Ina219Report::from(buf).to_result();
        let hm = report::Ina219Result::to_genericmap(report);

        let res = hm
            .into_iter()
            .filter_map(|(k, v)| {
                if skipinfo[&k] {
                    Some((
                        k,
                        v.f64().unwrap(), // надо чтобы это было без ""
                    ))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<_, _>>();

        println!("{:?}", res);
    }
}

fn parse_hex_or_dec(src: &str) -> Result<u16, ParseIntError> {
    if src.starts_with("0x") {
        let trimmed = src.trim_start_matches("0x");
        u16::from_str_radix(trimmed, 16)
    } else {
        u16::from_str_radix(src, 10)
    }
}
