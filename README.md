# WIP

This is a work in progress and not all functionalities may function correctly (or even be implemented).

# practicestuff

**practicestuff** is a CLI app designed to help you practice skills by asking questions and checking your answers. At the end, you'll see a number of correct answers, as well as some statistics. You can stop early by pressing `Ctrl+C`.

The application should work on all platforms.

List of skills:

- powers of a number
- multiplication table (times table) (not yet implemented)
- doomsday algorithm (not yet implemented)

## Configuration options

The application is somewhat configurable. Following options are available:

- Custom number of questions (with a possibility of endless mode)
- Disable statistics in-between questions
- Change what happens on incorrect answer (go to the next question, show correct answer and go to the next question or repeat until correct)

## Skills

### Powers

Allows to practice powers. Configurable parameters include:

- base (default: 2)
- exponent range (default: 1-16)

# Development

## Initial assumptions

- User has to choose a mode (skill) and optionally some flags/settings
- User is presented with a number of questions for certain skill and has to type a correct answer
- User should be able to quit at any time
- Some statistics should be presented (accuracy, response time, overall time)
- There should be a help flag available
- Argument parsing should be done without `clap` or any other library
- There should be some options to be set:
    - Option to set number of questions (including endless mode)
    - Option to show current accuracy between questions (default: true). Overall accuracy is presented at the end nevertheless
    - Option to exit early
    - Option to change behaviour on incorrect answer: continue to next question, show correct answer and continue, repeat
- It should be relatively easy to add new skills
- All skills should be configurable

## Current TODO

### Doomsday

- help & usage prompts
- question generation w/ options
- tests

### Multiplication table

- help & usage prompts
- question generation w/ options
- tests

### Improvements

- coloured output (termcolor?) with --no-color option
- additional mode (as many answers in given time)
- multiple texts for "correct" and "incorrect" answers
- gitlab & github CI

## What's next & bug reports

The application in its current state suits my needs, but I'm open for proposals, either for new skills to practice or more configurability. If you feel that the app lacks something, feel free to open an issue or a PR! I'd be more than happy.

As for the bugs, there are some for sure. If you see one, report it!
