#include <sancus/reactive.h>

#include <stdio.h>

#define LOCAL 0

static unsigned int SM_DATA(sensor) sensor_data = 50;
static unsigned int SM_DATA(sensor) tap_open = 0;
SM_OUTPUT(sensor, value_read);

#if LOCAL
  static const unsigned int SM_DATA(sensor) UPPER_TRESHOLD = 55;
  static const unsigned int SM_DATA(sensor) LOWER_TRESHOLD = 45;
  SM_OUTPUT(sensor, actuate_tap);
#endif

unsigned int SM_FUNC(sensor) get_sensor_data(void) {
  //puts("[sensor] retrieving data from sensor");

  if (tap_open) {
    return ++sensor_data;
  }
  else {
    return --sensor_data;
  }
}

#if LOCAL
  void SM_FUNC(sensor) check_sensor_data(unsigned int data) {
    //puts("[sensor] check sensor data..");

    if (data > UPPER_TRESHOLD) {
        puts("[sensor] sending command to close tap");
        tap_open = 0;
        actuate_tap((unsigned char *) &tap_open, sizeof(tap_open));
    }
    else if (data < LOWER_TRESHOLD) {
        puts("[sensor] sending command to open tap");
        tap_open = 1;
        actuate_tap((unsigned char *) &tap_open, sizeof(tap_open));
    }
  }
#endif

SM_INPUT(sensor, read_value, data, len) {
  puts("[sensor] reading sensor data");

  unsigned int value = get_sensor_data();

  value_read((unsigned char *) &value, sizeof(value));

  #if LOCAL
    check_sensor_data(value);
  #endif
}

SM_INPUT(sensor, tap_changed, data, len) {
  //puts("[sensor] tap has changed");

  if (len != 2) {
    puts("[sensor] tap_changed: error with data");
    return;
  }

  tap_open = *(unsigned int*) data;
}
