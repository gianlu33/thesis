const UPPER_TRESHOLD : u16 = 55;
const LOWER_TRESHOLD : u16 = 45;

//@ sm_output(get_sensor_value)
//@ sm_output(actuate_tap)
//@ sm_output(send_sensor_data)

//@ sm_entry
pub fn request_data(_data : &[u8]) -> ResultMessage {
    debug!("request_data");

    get_sensor_value(&[]);

    success(None)
}

//@ sm_input
pub fn sensor_data_received(data : &[u8]) {
    debug!("sensor_data_received");

    if data.len() != 2 {
        error!("sensor_data_received: Wrong data");
        return;
    }

    let value = u16::from_le_bytes([data[0], data[1]]);

    send_sensor_data(data);
    check_sensor_data(value);
}

fn check_sensor_data(value : u16) {
    if value > UPPER_TRESHOLD {
        info!("Sending command to close tap");
        actuate_tap(&0u16.to_le_bytes());
    }
    else if value < LOWER_TRESHOLD {
        info!("Sending command to open tap");
        actuate_tap(&1u16.to_le_bytes());
    }
}
