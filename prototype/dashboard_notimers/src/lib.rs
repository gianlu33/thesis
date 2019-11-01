use std::sync::Mutex;

lazy_static! {
    static ref LAST_SENSOR_VALUE: Mutex<Option<u16>> = {
        Mutex::new(None)
    };
    static ref TAP_STATE: Mutex<Option<bool>> = {
        Mutex::new(None)
    };
}

//@ sm_entry
pub fn print_last_sensor_value(_data : &[u8]) -> ResultMessage {
    debug!("print_last_sensor_value");

    let sensor_value = LAST_SENSOR_VALUE.lock().unwrap();

    info!(&format!("Last sensor value: {:?}", *sensor_value));

    drop(sensor_value);

    success(None)
}

//@ sm_entry
pub fn print_tap_state(_data : &[u8]) -> ResultMessage {
    debug!("print_tap_state");

    let tap_state = TAP_STATE.lock().unwrap();

    info!(&format!("Tap state: {:?}", *tap_state));

    drop(tap_state);

    success(None)
}

//@ sm_input
pub fn sensor_data_received(data : &[u8]) {
    debug!("sensor_data_received");

    if data.len() != 2 {
        error!("sensor_data_received: Wrong data received");
        return;
    }

    let value = u16::from_le_bytes([data[0], data[1]]);

    let mut last_value = LAST_SENSOR_VALUE.lock().unwrap();
    *last_value = Some(value);
}

//@ sm_input
pub fn tap_state_received(data : &[u8]) {
    debug!("tap_state_received");

    if data.len() != 2 {
        error!("tap_state_received: Wrong data");
        return;
    }

    let state = u16::from_le_bytes([data[0], data[1]]);

    let mut tap_state = TAP_STATE.lock().unwrap();

    match state {
        0 => *tap_state = Some(false),
        1 => *tap_state = Some(true),
        _ => error!("Wrong tap state")
    }
}
