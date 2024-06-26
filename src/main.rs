#![warn(clippy::all, clippy::nursery)]

pub mod args;
pub mod config;
pub mod glorious;
pub mod report;
pub mod util;

use args::{Args, Config, Kind, Report};
use clap::Parser;
use hidapi::HidApi;
use strum::IntoEnumIterator;
use util::none::None;

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let hid_api = HidApi::new()?;

    let device_info = hid_api
        .device_list()
        .filter(|d| {
            d.vendor_id() == glorious::VENDOR_ID
                && glorious::Device::iter().any(|x| x as u16 == d.product_id())
                && d.interface_number() == glorious::INTERFACE
        })
        // Get wired version of the mouse if available
        .min_by(|a, b| a.product_id().cmp(&b.product_id()))
        .none("no matching device found");

    let wired = device_info.product_id() <= 0x2013;

    let device = device_info.open_device(&hid_api)?;

    match args.kind {
        // mxw report
        Kind::Report(report) => match report {
            // mow report battery
            Report::Battery => report::battery::get(&device, wired),

            // mow report firmware
            Report::Firmware => report::firmware::get(&device, wired),
        },

        // mxw config
        Kind::Config(config) => match config {
            // mow config bind ...
            Config::Bind {
                profile,
                button,
                binding,
            } => config::bind::set(&device, profile, button, binding),

            // mxw config scroll <DIRECTION>
            Config::Scroll { direction } => config::scroll::set(&device, direction),

            // mxw config profile <ID>
            Config::Profile { id } => config::profile::set(&device, id),

            // mxw config sleep <MINUTES> [SECONDS]
            Config::Sleep { minutes, seconds } => config::sleep::set(&device, minutes, seconds),

            // mxw config led-brightness <WIRED> [WIRELESS]
            Config::LEDBrightness { wired, wireless } => {
                config::led_brightness::set(&device, wired, wireless)
            }

            // mxw config led-effect <EFFECT> ...
            Config::LEDEffect { profile, effect } => {
                config::led_effect::set(&device, profile, effect)
            }

            // mxw config polling-rate <MS>
            Config::PollingRate { ms } => config::polling_rate::set(&device, ms),

            // mxw config lift-off <MM>
            Config::LiftOff { mm } => config::lift_off::set(&device, mm),

            // mxw config debounce <MS>
            Config::Debounce { profile, ms } => config::debounce::set(&device, profile, ms),

            // mxw config dpi-stage <ID>
            Config::DPIStage { profile, id } => config::dpi_stage::set(&device, profile, id),

            // mxw config dpi-stages <STAGES>...
            Config::DPIStages { profile, stages } => {
                config::dpi_stages::set(&device, profile, stages)
            }

            // mxw config dpi-colors <COLORS>...
            Config::DPIColors { profile, colors } => {
                config::dpi_colors::set(&device, profile, colors)
            }
        },
    }
}
