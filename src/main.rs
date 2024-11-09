use std::process::{Command, Output};
use chrono::prelude::*;
use battery::{Manager, State};
use anyhow::{anyhow, Ok, Result};

fn main() {
    let date = get_date();
    let battery = get_battery().unwrap();
    let volume = get_volume().unwrap();
    println!("\u{f017}  {} | {}  | {} ", date, volume, battery);
}

fn get_date() -> String {
    let date = Local::now();
    date.format("%a %b %d - %I:%M %p").to_string()
}

fn get_battery() -> Result<String> {
    let manager = Manager::new();

    if let Some(battery) = (manager?.batteries()?).next() {
        let battery = battery?;
        let state: State = battery.state();
        let charge: usize = (battery.state_of_charge().value * 100.0) as usize;
        let state_icon = get_battery_state_icon(state);
        let charge_icon = get_battery_charge_icon(charge); 

        return Ok(format!("{}% {}{}", charge, charge_icon, state_icon.unwrap_or("".to_string())))
    }

    Ok("ok".to_string())
}

fn get_volume() -> Result<String> {
    let volume = Command::new("pamixer")
        .arg("--get-volume")
        .output()?;

    let mute = Command::new("pamixer")
        .arg("--get-mute")
        .output()?;

    let volume = get_output_as_string(volume)?.trim().to_string();
    let vol_val: u8 = volume.parse::<u8>()?;
    let mute = get_output_as_string(mute)?.trim().to_string();

    let icon = if mute == "true" {
        "\u{eee8}"
    } else if vol_val > 50 {
        "\u{f028}"
    } else if vol_val > 0 {
        "\u{f027}"
    } else {
        "\u{f026}"
    };

    Ok(format!("{}% {}", volume, icon))
}

fn get_battery_charge_icon(charge: usize) -> String {
    if charge > 90 {
        "\u{f0079}".to_string()
    } else if charge > 80 {
        "\u{f0082}".to_string()
    } else if charge > 70 {
        "\u{f0081}".to_string()
    } else if charge > 60 {
        "\u{f0080}".to_string()
    } else if charge > 50 {
        "\u{f007f}".to_string()
    } else if charge > 40 {
        "\u{f007e}".to_string()
    } else if charge > 30 {
        "\u{f007d}".to_string()
    } else if charge > 20 {
        "\u{f007c}".to_string()
    } else if charge > 10 {
        "\u{f007b}".to_string()
    } else {
        "\u{f007a}".to_string()
    }
}

fn get_battery_state_icon(state: State) -> Option<String> {
    match state {
        State::Charging => Some("\u{f140b}".to_string()),
        _ => None
    }
}

fn get_output_as_string(output: Output) -> Result<String> {
    if output.status.success() {
        return Ok(String::from_utf8(output.stdout)?);
    }
    Err(anyhow!("Failed to get command output"))
}

#[test]
fn test_get_battery_charge_icon() {
    assert_eq!(get_battery_charge_icon(95), "󰁹");
    assert_eq!(get_battery_charge_icon(85), "󰂂");
    assert_eq!(get_battery_charge_icon(75), "󰂁");
    assert_eq!(get_battery_charge_icon(65), "󰂀");
    assert_eq!(get_battery_charge_icon(55), "󰁿");
    assert_eq!(get_battery_charge_icon(45), "󰁾");
    assert_eq!(get_battery_charge_icon(35), "󰁽");
    assert_eq!(get_battery_charge_icon(25), "󰁼");
    assert_eq!(get_battery_charge_icon(15), "󰁻");
    assert_eq!(get_battery_charge_icon(5), "󰁺");
}

#[test]
fn test_get_battery_state_icon() {
    assert_eq!(get_battery_state_icon(State::Charging).unwrap(), "󱐋");
    assert!(get_battery_state_icon(State::Discharging).is_none());
}
