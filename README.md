# Rust Alarm

An alarm manager for Linux based on `xcowsay`. Written in Rust as a project for learning Rust development.

## API Overview

Inside `.config`, the configuration stores which alarms are currently active and all the information about each alarm.
In addition, there is a CLI binary that can add one-time events (timers).

## Design

### General Operation

There is an Alarm Manager process that uses the epoll manager library to poll on a Unix Domain Socket (UDS) through which commands can be sent.
The manager process manages multiple threads to handle timers and alarms efficiently.

The following actions are supported via the API:
1. Reload all alarms
2. Add a new alarm
3. Remove an alarm
4. Clear all alarms
5. List all current alarms
6. Add a new timer
7. Remove a timer
8. List all current timers
9. Clear all timers
10. List all current alarms & timers
11. Clear all alarms & timers

Removal of an alarm or timer is performed via index.
*TODO: Decide on detailed API for adding timers/alarms.*

### Threads Design

#### The Main Thread
There is a single management thread that polls on the socket/pipe that receives external commands, as well as the pipe that timer/alarm threads use.
When poll is triggered:
1. If triggered by a command, handle the command and proceed with polling.
2. If triggered by a timer/alarm, send the notification to the user (each timer/alarm has its own pipe, which is closed when deleting the timer/alarm).

#### A Timer/Alarm Thread
Sleeps until the timer is triggered.
When triggered, sends the timer/alarm UID to the main thread. If it should not repeat, the thread terminates; otherwise, it schedules the next occurrence.

## General Notes

* **Timers**: Not persistent.
* **Alarms**: Persistent.
