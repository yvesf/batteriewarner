use std::str;
use std::fs::OpenOptions;
use std::io::{Result, Read, Write, Error, ErrorKind};
use std::convert::AsRef;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

const PATH_ENERGY_NOW: &str = "/sys/class/power_supply/BAT0/energy_now";
const PATH_ENERGY_FULL: &str = "/sys/class/power_supply/BAT0/energy_full_design";
const PATH_LIGHT: &str = "/sys/devices/platform/thinkpad_acpi/leds/tpacpi::power/brightness";

fn error(msg: &str) -> Error {
    Error::new(ErrorKind::Other, msg)
}

macro_rules! mytry {
    ($expr : expr, $msg : expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            return Err(Error::new(ErrorKind::Other, format!("{}: {:?}", $msg, err)))
        }
    });
}

fn file_read_integer<P: AsRef<Path>>(path: P) -> Result<u64> {
    let mut buf = [0; 20];
    let mut file = mytry!(OpenOptions::new().read(true).open(path.as_ref()),
        "Failed to open file");
    let len = mytry!(file.read(&mut buf), "Failed to read from file");
    if len < 1 {
        return Err(error("Failed to read value, file is empty"));
    }

    let s = mytry!(str::from_utf8(&buf[0..len]), "UTF-8 error when reading file");
    Ok(mytry!(s.trim().parse(), "Failed to parse String"))
}

fn read_battery_percentage() -> Result<u32> {
    let now = mytry!(file_read_integer(PATH_ENERGY_NOW), "Failed to read energy_now");
    let full = mytry!(file_read_integer(PATH_ENERGY_FULL), "Failed to read energy_full");
    Ok(((now * 100) / full) as u32)
}

fn write_brightness(level: u8) -> Result<()> {
    let string = format!("{}\n", level);
    let mut file = mytry!(OpenOptions::new().write(true).open(PATH_LIGHT), "Failed to open PATH_LIGHT");
    let bytes_written = mytry!(file.write(string.as_bytes()), "Failed to write brightness file");
    if bytes_written == string.len() {
        Ok(())
    } else {
        Err(error("Failed to write all bytes to brightness file"))
    }
}

fn read_brightness() -> Result<u8> {
    let level = mytry!(file_read_integer(PATH_LIGHT), "Failed to read PATH_LIGHT");
    Ok(level as u8)
}

fn indicate_battery_level(level: u32) -> Result<()> {
    let led_state = mytry!(read_brightness(), "Reading brightness failed");
    let flashing_time: u32 = 10000;
    let offtime: u32 = 40;
    let ontime: u32 = 20;
    let n = flashing_time / (level * offtime + level * ontime);
    for _ in 0..n {
        mytry!(write_brightness(0), "Failed to switch off brightness");
        sleep(Duration::from_millis(u64::from(level * offtime)));
        mytry!(write_brightness(1), "Failed to switch on brightness");
        sleep(Duration::from_millis(u64::from(level * ontime)));
    }
    mytry!(write_brightness(led_state), "Failed to re-set brightness");
    Ok(())
}

fn main() {
    loop {
        match read_battery_percentage() {
            Ok(level) => {
                if level < 20 {
                    if let Err(e) = indicate_battery_level(level) {
                        panic!("Error: {}", e);
                    }
                }
            }
            Err(e) => panic!("Error: {}", e),
        }
        sleep(Duration::from_secs(60));
    }
}
