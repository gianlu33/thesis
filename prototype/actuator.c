#include <sancus/reactive.h>

#include <stdio.h>

static unsigned int SM_DATA(actuator) tap_open = 0;

SM_OUTPUT(actuator, tap_changed);

void SM_FUNC(actuator) activate_tap(unsigned int state) {
  if (tap_open != state) {
    if (state) {
      puts("[actuator] opening tap");
    }
    else {
      puts("[actuator] closing tap");
    }

    tap_open = state;
    tap_changed((unsigned char *) &state, sizeof(state));
  }
}

SM_INPUT(actuator, actuate_tap, data, len) {
  //puts("[actuator] requested change of tap");

  if (len != 2) {
    puts("[actuator] actuate_tap: error with data");
    return;
  }

  unsigned int state = *(unsigned int*) data;

  activate_tap(state);
}
