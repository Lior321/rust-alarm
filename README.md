# Rust-Alarm
Alarm for linux based on xcowsay.
Written in rust as a small project learning to write actual rust

# What is the API?
Inside `.config` we will have the configuration which alarms are currently active and all the informatiion about each alarm
In addition, there will be CLI binary that will add one-time events (timers)

# Design
## General operation
There will be a Alarm manager process that will use my epoll manager library to poll on a UDS which everyone can use to send commands to it.
The manager process will have multiple threads running around to manage all the timers/alarms without needing to do complicated calculations
The following Actions are supported via the API:
1. Reload all alarms
2. Add a new alarm
3. Remove an alarm
4. Clear all alarms
5. List all current alarm
6. Add a new timer
7. Remove a timer
8. List all current timers
9. Clear all timers
10. List all current alarms & timers
11. Clear all alarms & timers

Removal of alarm/timer will be done via index
TODO - decids on detailed api for addition of timers/alarms

## Threads Design
***The Main Thread***
There will be a single management thread that will poll on the pipe that receives commands (can be sent basically by anyone), and on the pipe that the timer/alarm threads will use.
When the poll i triggered:
1. If by a command than handle the command and proceed with polling
2. If triggered by a timer/alarm send the notification to the user - each timer/alarm will have it's own unnamed pipe that will be closed when deleting the timer/alarm
***A Timer/Alarm Thread***
Sleep until the timer is triggered.
When triggered send the timer/alarm UID to the main thread. If should not repeat, than die, else set timer for next occurence

## General notes
Timers are not persistent!
Alarms are persistent.
