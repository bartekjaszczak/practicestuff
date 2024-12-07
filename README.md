# WIP

This is a work in progress. Basic functionality has already been implemented, but there are a few things I'm planning to do:

- coloured output with --colour option to control it
- time rush mode (as many correct answers in given time)
- various feedback options (to replace boring "Correct" and "Incorrect" messages)

# practicestuff

**practicestuff** is a CLI app designed to help you practise skills by asking questions and checking your answers. At the end, you'll see a number of correct answers, as well as some statistics. You can stop early by pressing `Ctrl+C`.

The application should work on all platforms.

Current list of skills:

- powers of a number
- multiplication table (times table)
- doomsday algorithm

## Installation

To install the application, you need cargo. Then, run:

```bash
cargo install practicestuff
```

## Usage

If you're fine with the defaults, choose a skill to practise (e.g., powers) and simply run:

```bash
practicestuff powers
```

You'll be presented with 20 questions and at the end you'll see how it went. If you don't like the defaults, you can adjust some settings. To display configuration options use help:

```bash
# General configuration options and available commands/skills
practicestuff --help

# Powers specific options
practicestuff powers --help
```

## Configuration options

The application is somewhat configurable. Following options are available:

- Custom number of questions (with a possibility of endless mode)
- Disable statistics in-between questions
- Change what happens on incorrect answer (go to the next question, show correct answer and go to the next question or repeat until correct)

## Skills

### Powers

Allows to practise powers. Configurable parameters include:

- base (default: 2)
- exponent range (default: 1-16)

### Times table

Allows to practise multiplications. Factors' range is configurable (default: 1-10 (regular times table)).

### Doomsday algorithm

Allows to practise calculating the day of the week for a given date. Year range is configurable. By default, the application presents questions with dates ranging from ~1900 to ~2100, with a slight chance to go beyond. When either lower or upper limit is set, the date is picked randomly with equal probability for each year.

# Rationale

I created this simple app because I wanted to learn and practice the Doomsday algorithm. Later I thought that it might be cool not to limit the app to just one skill, but allow extensible architecture. I also didn't want to use any external libraries for argument parsing, so I implemented that myself as well.


# What's next & bug reports

The application in its current state suits my needs, but I'm open for proposals, either for new skills to practise or more configurability. If you feel that the app lacks something, feel free to open an issue or a PR! I'd be more than happy.

As for the bugs, there are some for sure. If you see one, report it!
