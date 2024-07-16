use std::fs;

pub fn read_temperature(device_id: &String) -> Result<f32, String> {
    let path = format!("/sys/bus/w1/devices/{}/temperature", device_id);
    let raw_temperature = fs::read_to_string(&path)
        .map_err(|error| format!("Failed to read temperature: {}", error))?;

    let int_temperature: i32 = raw_temperature
        .trim()
        .parse()
        .map_err(|_| format!("Failed to parse temperature: {}", raw_temperature))?;
    let temperature = int_temperature as f32 / 1000.0;

    Ok(temperature)
}
