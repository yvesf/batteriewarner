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

fn file_read_integer<P: AsRef<Path>>(path: P) -> Result<u64> {
    let mut buf = [0; 20];
    let mut file = OpenOptions::new().read(true).open(path.as_ref())?;
    let len = file.read(&mut buf)?;
    if len < 1 {
        return Err(Error::new(ErrorKind::Other, "Empty file"));
    };

    let s = str::from_utf8(&buf[0..len])
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    s.trim().parse()
        .map_err(|err| Error::new(ErrorKind::Other, format!("String parse failed: {}", err)))
}

fn read_battery_percentage() -> Result<u32> {
    let now = file_read_integer(PATH_ENERGY_NOW)?;
    let full = file_read_integer(PATH_ENERGY_FULL)?;
    Ok(((now * 100) / full) as u32)
}

fn write_brightness(level: u8) -> Result<()> {
    let string = format!("{}\n", level);
    let mut file = OpenOptions::new().write(true).open(PATH_LIGHT)?;
    if file.write(string.as_bytes())? == string.len() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Failed to write brightness"))
    }
}

fn read_brightness() -> Result<u8> {
    let level = file_read_integer(PATH_LIGHT)?;
    Ok(level as u8)
}

fn indicate_battery_level(level: u32) -> Result<()> {
    let led_state = read_brightness()?;
    let flashing_time: u32 = 10000;
    let offtime: u32 = 40;
    let ontime: u32 = 20;
    let n = flashing_time / (level * offtime + level * ontime);
    for _ in 0..n {
        write_brightness(0)?;
        sleep(Duration::from_millis(u64::from(level * offtime)));
        write_brightness(1)?;
        sleep(Duration::from_millis(u64::from(level * ontime)));
    }
    write_brightness(led_state)?;
    Ok(())
}

fn main() {
    loop {
        let p = read_battery_percentage();
        match p {
            Ok(level) => {
                if level < 20 {
                    if let Err(e) = indicate_battery_level(level) {
                        panic!("Error: {}", e);
                    }
                }
            }
            Err(e) => panic!("Error: {}", e),
        }
        sleep(Duration::from_secs(10));
    }
}
