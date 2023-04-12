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

unsigned char* MOVE_PROCESSED = "A";
unsigned char STILL_MOVING = "B";
unsigned char HAHA = "D";
const byte a = 2;
uint8_t NOOO = 99;

int max_pos = MOTOR_STEPS * MICROSTEPS * 1;  // motor steps to make the platform rotate one time

String inString = "";
String command = "22";

int pos_top = 0;

bool waiting_for_top_pos = false;

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
  // Serial.print("A");
  // Serial.print("B");
  // testSerial();

  // if (command == "22") {
  //   Serial.println("geilo");
  // }
  // Serial.println(command);
  // Serial.println(command.toInt());


  // Serial.println((char)104);

  processCommands();
  // testMoveSteppers();
  // delay(1500);

  // aaa();
  // moveSteppers();
  // delay(1500);
}

void processCommands() {
  while (Serial.available() > 0) {
    Serial.println("A");
    int inInt = Serial.read();
    // char inChar = (char) inInt;

    if (waiting_for_top_pos) {
      stepper_bottom.enable();
      stepper_top.enable();

      moveSteppers(inInt);
      stepper_bottom.disable();
      stepper_top.disable();
      Serial.print("C");  // signal that moving is done
      waiting_for_top_pos = false;
    }

    if (inInt == 104) {  // 104 => h
      Serial.print("B");
      waiting_for_top_pos = true;
    }

    // if (inInt == 104) {
    //   Serial.println("B");
    // }
  }
}

void testSerial() {
  if (Serial.available()) {
    Serial.println(String("received") + Serial.readString());
  }


  Serial.println("lol");
  Serial.println(22.1249);
}

void moveSteppers(int top_desired) {
  // Serial.println("starting");
  // Serial.println("C");
  // stepper_bottom.setRPM(20);
  // stepper_top.setRPM(200);
  float multiplier = ((float)top_desired) / 256.0;
  int remapped = max_pos * multiplier;
  int to_move_top = remapped - pos_top;

  pos_top = remapped;
  // Serial.println(remapped);


  stepper_bottom.startMove(0);
  stepper_top.startMove(-to_move_top);


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


  // delay(100);
}

void testMoveSteppers() {
  for (int i = 0; i < 256; i++) {
    moveSteppers(i);
    delay(10);
  }
}

void aaa() {

  stepper_bottom.enable();
  stepper_bottom.startMove(max_pos);

  unsigned wait_time_micros = 1;
  while (wait_time_micros > 0) {
    wait_time_micros = stepper_bottom.nextAction();
    int remaining = stepper_bottom.getStepsRemaining();
    int new_pos = 0;
  }
  // delay(0);
  stepper_bottom.disable();
}
