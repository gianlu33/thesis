use std::time::SystemTime;

pub struct SensorData {
    value                  : Option<u16>,
    request_sensor_time    : SystemTime,
    receive_sensor_time    : Option<SystemTime>,
    request_tap_time       : Option<SystemTime>
}

impl std::fmt::Debug for SensorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
        -> Result<(), std::fmt::Error> {
            write!(f, "{:?}", self.value)
        }
}

impl SensorData {
    pub fn new() -> SensorData {
        SensorData {
            value               : None,
            request_sensor_time : SystemTime::now(),
            receive_sensor_time : None,
            request_tap_time    : None
        }
    }

    pub fn get_value(&self) -> Option<u16> {
        self.value.clone()
    }

    pub fn set_value(&mut self, value : u16) {
        self.value = Some(value);
        self.receive_sensor_time = Some(SystemTime::now());
    }

    pub fn get_request_sensor_time(&self) -> SystemTime {
        self.request_sensor_time.clone()
    }

    pub fn get_receive_sensor_time(&self) -> Option<SystemTime> {
        self.receive_sensor_time.clone()
    }

    pub fn get_request_tap_time(&self) -> Option<SystemTime> {
        self.request_tap_time.clone()
    }

    pub fn set_request_tap_time(&mut self) {
        self.request_tap_time = Some(SystemTime::now());
    }
}
