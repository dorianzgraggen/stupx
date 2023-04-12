#include "BasicStepperDriver.h"

// Motor steps per revolution. Most steppers are 200 steps or 1.8 degrees/step
#define MOTOR_STEPS 200
#define RPM 60

// Since microstepping is set externally, make sure this matches the selected mode
// Set the jumper to middle position when using MICROSTEPS 4, no jumper = MICROSTEPS 1
// 1=full step, 2=half step etc.
#define MICROSTEPS 4

// Driver in CNC shield X
#define DIR_X 5
#define STEP_X 2

// Driver in CNC shield Y
#define DIR_Y 6
#define STEP_Y 3

// Define the pin for enable/disable functionality
#define SLEEP 8

BasicStepperDriver stepper_bottom(MOTOR_STEPS, DIR_X, STEP_X, SLEEP);
BasicStepperDriver stepper_top(MOTOR_STEPS, DIR_Y, STEP_Y, SLEEP);

int final_pos = MOTOR_STEPS * MICROSTEPS * 7;  // motor steps to make the platform rotate one time

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);

  stepper_bottom.begin(RPM, MICROSTEPS);
  stepper_bottom.setEnableActiveState(LOW);
  stepper_bottom.enable();

  stepper_top.begin(RPM, MICROSTEPS);
  stepper_top.setEnableActiveState(LOW);
  stepper_top.enable();
}

void loop() {
  // testSerial();
  Serial.println("lol");
  // moveSteppers();
}

void testSerial() {
  if (Serial.available()) {
    Serial.println(String("received") + Serial.readString());
  }


  Serial.println("lol");
  Serial.println(22.1249);
}

void moveSteppers() {
  stepper_bottom.setRPM(20);
  Serial.println(Serial.available());
  stepper_bottom.startMove(final_pos);
  stepper_top.startMove(final_pos);



  unsigned wait_time_bottom = 1;
  unsigned wait_time_top = 1;
  while (wait_time_bottom > 0 || wait_time_top > 0) {

    if (wait_time_bottom > 0) {
      wait_time_bottom = stepper_bottom.nextAction();
    }

    if (wait_time_top > 0) {
      wait_time_top = stepper_top.nextAction();
    }

    // int remaining = stepper_bottom.getStepsRemaining();
  }
}
