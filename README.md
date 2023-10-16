# Score Tracker
![Score Tracker](https://raw.githubusercontent.com/weclaw1/score-tracker/main/src/resources/io.github.weclaw1.ScoreTracker.svg)

**Score Tracker** is an application for tracking player scores in card and board games. It also contains additional functionality such as timer for measuring players turn times.

## Features
- track player scores in card and board games (or any other turn based games)
- unlimited number of players and turns
- adaptive - can be used on desktop and mobile devices
- timer for measuring players turn time

![Screenshot](https://raw.githubusercontent.com/weclaw1/score-tracker/main/src/resources/screenshot.png)

## Installation

### Requirements
If you use Flatpak you may skip this section.

#### Ubuntu/Debian
```
sudo apt install libgtk-4-dev libadwaita-1-dev
```
#### Fedora/CentOS
```
sudo dnf install gtk4-devel glib2-devel libadwaita-devel
```

#### Alpine Linux
```
sudo apk add gtk4.0-dev gcc libadwaita-dev
```

### Flatpak
Flatpak is the recommended install method.
In order to install Score Tracker using Flatpak run:
```
flatpak install flathub io.github.weclaw1.ScoreTracker
```

### Precompiled binaries
Ready-to-go executables can be found on the releases page.

### Cargo
To install Score Tracker using cargo run the following command:
```
cargo install score-tracker
```