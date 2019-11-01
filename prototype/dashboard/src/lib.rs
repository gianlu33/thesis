use std::sync::Mutex;
use std::time::SystemTime;

mod sensor_data;
use sensor_data::SensorData;

lazy_static! {
    static ref LAST_SENSOR_VALUE: Mutex<Option<SensorData>> = {
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
pub fn sensor_data_requested(_data : &[u8]) {
    debug!("sensor_data_requested");

    let mut last_value = LAST_SENSOR_VALUE.lock().unwrap();
    *last_value = Some(SensorData::new());
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
    match &mut*last_value {
        Some(val) => val.set_value(value),
        None      => error!("No previous value!")
    }
}

//@ sm_input
pub fn tap_change_requested(_data : &[u8]) {
    debug!("tap_change_data_requested");

    let mut last_value = LAST_SENSOR_VALUE.lock().unwrap();
    match &mut*last_value {
        Some(val) => val.set_request_tap_time(),
        None      => error!("No previous value!")
    }
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

    if let Err(e) = check_time_difference() {
        error!(e);
    }
}

fn check_time_difference() -> Result<(), &'static str> {
    let last_value = LAST_SENSOR_VALUE.lock().unwrap();

    let last_value = match &*last_value {
        Some(val)   => val,
        None        => {
            return Err("No sensor data to check time difference");
        }
    };

    let end_time = Some(SystemTime::now());
    let start_time = Some(last_value.get_request_sensor_time());
    let receive_sensor_time = last_value.get_receive_sensor_time();
    let request_tap_time = last_value.get_request_tap_time();


    info!(&format!("A->B: {:?} ms | B->C: {:?} ms | C->D: {:?} ms | total: {:?} ms",
        get_millis(start_time, receive_sensor_time)?, get_millis(receive_sensor_time, request_tap_time)?,
        get_millis(request_tap_time, end_time)?, get_millis(start_time, end_time)?
    ));

    Ok(())
}


fn get_millis(start : Option<SystemTime>, end : Option<SystemTime>) -> Result<Option<u128>, &'static str> {
    let start = match start {
        Some(s) => s,
        None    => return Ok(None)
    };

    let end = match end {
        Some(e) => e,
        None    => return Ok(None)
    };

    match end.duration_since(start) {
        Ok(n)       => Ok(Some(n.as_millis())),
        Err(_)      => Err("Error while checking the time difference")
    }
}
