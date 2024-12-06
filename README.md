# WIP

This is a work in progress and not all functionalities may function correctly (or even be implemented).

# Idea

CLI app designed to help you practice skills by asking questions and checking your answers.

Initial list of skills:

- powers of a digit
- multiplication table (times table)
- doomsday algorithm

## Assumptions

- User has to choose a mode (skill) and optionally some flags/settings
- User is presented with a number of questions for certain skill and has to type a correct answer
- User should be able to quit at any time
- Some statistics should be presented (accuracy, response time, overall time)
- There should be a help tooltip
- Program should return 0 on 100% accuracy
- Algorithm parsing should be done without `clap` or any other library

## General options
- Option to set number of questions (default: 20, 0 means endless mode)
- Option to show current accuracy between questions (default: true). Overall accuracy is presented at the end nevertheless
- Option to exit early by typing 'q', 'e' or similar
- Option to change behaviour on incorrect answer: continue to next question, show correct answer and continue, repeat (default: show correct & continue)

## Specific options

### Powers

- Option to set power base (default: 2)
- Option to set upper and lower power boundary (default: 1 to 16)

### Multiplication table

- Option to set upper and lower number boundary (default: 1 to 10)

### Doomsday algorithm

- Option to set start and end date (default: ±100 years 80% of the time, ±400 years 20% of the time; or normal distribution)

# TODO

## General

DONE

## Powers

DONE

## Doomsday

- help & usage prompts
- question generation w/ options
- tests

## Multiplication table

- help & usage prompts
- question generation w/ options
- tests

## Improvements

- coloured output (termcolor?) with --no-color option
- additional mode (as many answers in given time)
- multiple texts for "correct" and "incorrect" answers
