# Timaru

A time table management app which meets my needs. Written in Rust.

Work in progress and not well tested. Currently only CLI interface is working which throws unhelpful messages on errors, but works if you know the commands. Only works on Linux.

## Upcoming Features

Almost certainly planned:
- a TUI with all sorts of functionalities
- async execution wherever possible
- sending notifications using something like [`libnotify`]( https://gitlab.gnome.org/GNOME/libnotify ), will also need a background process for this

Might implement, but only after above things are done:
- a config file with following options:
	- colorscheme for TUI
	- specify date and time formats
	- custom keybindings
	- custom shortcuts for CLI
- Encryption of data
- Option to use less storage by storing in binary format or using a database
- Exporting to other formats like `ics`, `csv` etc.
- A logo for the app

Not planned for sure, but a programmer can dream:
- Windows support
- a GUI
- A web app/client
- Ability to host app on a server
- Multi-user support
- A mobile app when Rust takes over Kotlin as the goto language for Android Development

## Goals
- A timetable management app that I like with features that I want, but is still configurable enough to allow others to use as well.
- Vim-like keybindings wherever possible.

## The Concept

### Schedule and Tasks

Each day has a **schedule**, which is divided into **tasks**, and each task has following properties:

| Field         | Decription                                                                                     |
| ------------- | ---------------------------------------------------------------------------------------------- |
| `finished`    | a bool, telling whether the task is done or not.                                               |
| `time`        | time associated with the task. See [`TaskTime`](#TaskTime) for valid inputs.                   |
| `description` | description of the task in form of string.                                                     |
| `pomodoro`    | a 2-tuple of `u8` of the form (`total`, `done`). See [`Pomodoro`](#Pomodoro) for more details. |

### TaskTime

All instances of time used are `TaskTime` type. There are 4 valid types of `TaskTime`:

| Type           | Decription                                                                         |
| -------------- | ---------------------------------------------------------------------------------- |
| precise type   | `12:30`, `16:30:30`                                                                |
| general type   | one of `morning`, `noon`, `afternoon`, `evening`, `midnight`, or a `custom` string |
| precise period | 2 precise types, separated with a hyphen, no spaces. `12:30-16:30`.                |
| general period | 2 general types, separated with a hyphen, no spaces. `morning-evening`.            |

### Date

Date everywhere is represented as `dd-mm-yyyy`. More representations of date are coming soon.

### Pomodoro

A pomodoro has 2 fields:
- `total`: total number of pomodoro cycles given to the task.
- `done`: number of pomodoro cycles done out of total.

## Storing things

The entire config lives in one of the following, chosen in the order mentioned:
- `$XDG_CONFIG_HOME/timaru`, if defined
- `$HOME/.config/timaru` if defined
- else throws an error.

## CLI Commands

| Command                                             | Description                                                                                                                  |
| --------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `timaru list [date]`                                | Shows the schedule of the given date. If not specified then shows current day's schedule                                     |
| `timaru week`                                       | Shows the schedules of next 7 days.                                                                                          |
| `timaru month`                                      | Shows the schedules of all dates till same day next month.                                                                   |
| `timaru add [date] [time] [pomodoro] <description>` | Add a new task.                                                                                                              |
| `timaru remove <date> <idx>`                        | Remove a task. `idx` should match as listed by `timaru list`                                                                 |
| `timaru update <old_date> <idx> <subcmd>`           | Update a task. `idx` should match as listed by `timaru list`. See [`Update Command`](#Update-Command) for possible `subcmd`. |

### Update Command

`timaru update` has following subcommands:

| Command                                                               | Description                                          |
| --------------------------------------------------------------------- | ---------------------------------------------------- |
| `date <date>`                                                         | change the date of the task.                         |
| `time <time>`                                                         | change the time of the task, keeping the date same.  |
| `description <desc>`                                                  | change the description of the task.                  |
| `done`                                                                | mark task as done.                                   |
| `notdone`                                                             | mark task as not done.                               |
| <code>pomodoro [new <total> &#124; remove &#124; done <done>]</code>  | change pomodoro of the task                          |

## Goals

## License

Dual licensed under Apache License Version 2.0 and MIT License.

## Contributing

Follows [Rust Code of Conduct]( https://www.rust-lang.org/policies/code-of-conduct ) wherever applicable.
