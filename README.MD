# Digital Wellbeing Tracker

Activity tracker made for Windows using Rust-lang and EGUI UI library

![gui](https://i.imgur.com/d6OkIfK.png)
## Installation

Download the .exe installer from the latest releases and follow the instruction. A restart would be needed to enable the tracker.
If you are having any problem, make sure to make an Issue and report it.

[Download here](https://github.com/MOMIN5/digital-wellbeing/releases)

## For Developers
The installer is made using [Inno Setup](https://jrsoftware.org/isinfo.php), to run at startup it adds a registry in ```SOFTWARE\Microsoft\Windows\CurrentVersion\Run```. All the log files are stored in ```/%APPDATA%/digital-wellbeing/Data/```

Special thanks to egui discord!
