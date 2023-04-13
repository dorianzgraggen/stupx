#include "BasicStepperDriver.h"

// Motor steps per revolution. Most steppers are 200 steps or 1.8 degrees/step
#define MOTOR_STEPS 200
#define RPM 25

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

int previous_slide_pos = 0;

bool waiting_for_top_pos = false;
int temp_slide_pos = 0;
bool waiting_for_rot = false;

byte previous_rot = 0;

#define SMALL_GEAR_TEETH 18
#define LARGE_GEAR_TEETH 126

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);

  stepper_bottom.begin(RPM, MICROSTEPS);
  stepper_bottom.setEnableActiveState(LOW);
  stepper_bottom.enable();

  stepper_top.begin(RPM, MICROSTEPS);
  stepper_top.setEnableActiveState(LOW);
  stepper_top.enable();

  Serial.println("=========begin");
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
  // testStepperBug();
  // testStepperBug2();
  // testRotation();
  // delay(1500);

  // aaa();
  // moveSteppers();
  // delay(1500);
}

void testRotation() {
  stepper_bottom.move(MOTOR_STEPS * MICROSTEPS * (LARGE_GEAR_TEETH / SMALL_GEAR_TEETH));
  delay(2000);
}

void processCommands() {
  while (Serial.available() > 0) {
    Serial.print("A");
    int inInt = Serial.read();
    // char inChar = (char) inInt;

    if (inInt == 104) {  // 104 => h
      Serial.print("B");
      waiting_for_top_pos = true;
      continue;
    }

    if (waiting_for_top_pos) {
      temp_slide_pos = inInt;
      waiting_for_top_pos = false;
      waiting_for_rot = true;
      continue;
    }

    if (waiting_for_rot) {
      stepper_bottom.enable();
      stepper_top.enable();

      moveSteppers(temp_slide_pos, byte(inInt));
      stepper_bottom.disable();
      stepper_top.disable();
      Serial.print("C");  // signal that moving is done

      waiting_for_rot = false;
      continue;
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

void moveSteppers(int slide_pos_desired, byte rotation_desired) {
  moveSteppers(slide_pos_desired, rotation_desired, false);
}

void moveSteppers(int slide_pos_desired, byte rotation_desired, bool debug) {
  // Serial.println("starting");
  // Serial.println("C");
  // stepper_bottom.setRPM(20);
  // stepper_top.setRPM(200);

  // remaps from 0..255 to 0..{steps_for_max_slide_position}
  float slide_multiplier = ((float)slide_pos_desired) / 256.0;
  int remapped_slide_pos = max_pos * slide_multiplier;

  int to_slide = remapped_slide_pos - previous_slide_pos;
  previous_slide_pos = remapped_slide_pos;
  stepper_top.startMove(to_slide);


  // ROTATION =======================================================

  if (debug) {
    Serial.print("from ");
    Serial.print(previous_rot);
    Serial.print(" to ");
    Serial.println(rotation_desired);
  }

  byte t = previous_rot + rotation_desired;

  if (debug) {

    Serial.print("t: ");
    Serial.println(t);
    Serial.println((int)t);
  }

  int previous_smart = (int)t - (int)rotation_desired;

  if (debug) {

    Serial.print("previous_smart: ");
    Serial.println(previous_smart);
  }

  int to_rotate = (rotation_desired - previous_smart) % 256;
  int to_rotate2 = (rotation_desired - (int)previous_rot);
  int to_rotate3 = (rotation_desired - (int)previous_rot) + 255;
  int to_rotate4 = (rotation_desired - (int)previous_rot) - 255;

  if (debug) {
    Serial.print("to_rotate: ");
    Serial.println(to_rotate);
    Serial.print("to_rotate2: ");
    Serial.println(to_rotate2);
    Serial.print("to_rotate3: ");
    Serial.println(to_rotate3);
    Serial.print("to_rotate4: ");
    Serial.println(to_rotate4);
  }

  if (abs(to_rotate2) < abs(to_rotate)) {
    if (debug) {
      Serial.println("using to_rotate2");
    }
    to_rotate = to_rotate2;
  }

  if (abs(to_rotate3) < abs(to_rotate)) {
    if (debug) {
      Serial.println("using to_rotate3");
    }
    to_rotate = to_rotate3;
  }

  if (abs(to_rotate4) < abs(to_rotate)) {
    if (debug) {
      Serial.println("using to_rotate4");
    }
    to_rotate = to_rotate4;
  }


  int one_full_rotation = MOTOR_STEPS * MICROSTEPS * (LARGE_GEAR_TEETH / SMALL_GEAR_TEETH);
  int remapped_to_rotate = ((float)to_rotate / 256.0) * one_full_rotation;
  previous_rot = rotation_desired;

  if (debug) {

    Serial.print("startMove: ");
    Serial.println((int)remapped_to_rotate);
  }

  stepper_bottom.startMove((int)remapped_to_rotate);


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
  if (debug) {

    Serial.println("");
  }

  // delay(100);
}

void testMoveSteppers() {
  delay(2000);
  Serial.println("gonna move");
  for (int i = 0; i < 256; i++) {
    byte c = byte(i);
    Serial.println(c);
    moveSteppers(0, c);
    // delay(10);
  }
  Serial.println("in between");
  delay(1000);
  Serial.println("section 2");
  for (int i = 0; i < 256; i++) {
    byte c = byte(255 - i);
    Serial.println(c);
    moveSteppers(0, c);
  }
  Serial.println("before wait");
  delay(4000);
  Serial.println("after wait");
}

void testStepperBug() {
  delay(2000);
  Serial.println("step1");
  moveSteppers(0, 255, true);
  delay(1000);
  Serial.println("step2");
  moveSteppers(20, 245, true);

  delay(2000);
}



void testStepperBug2() {
  delay(2000);
  Serial.println("step1");
  moveSteppers(0, 254, true);
  delay(1000);
  Serial.println("step2");
  moveSteppers(0, 0, true);

  delay(2000);
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
