# Rusty Dock
Rusty Dock is a customizable and lightweight desktop dock application built using **React**, **Rust**, and **Tauri**. It provides an efficient interface for managing app shortcuts, widgets, and system information like battery and network status, all in one place.

> "As they say, if you don’t like a software, build your own. so I made my own :) "

## Features
### Core Features
- [ ] App Shortcuts: Add, remove, and rearrange shortcuts
- [ ] Drag-and-Drop Support
- [ ] System Info Widget (CPU, memory, disk usage)
- [ ] Battery Status Widget
- [ ] Network Status Widget

### Customization Options
- [ ] Hot-reload configuration changes
- [ ] Themes and colors
- [ ] Transparency control
- [ ] Adjustable icon sizes
- [x] ~~Dock positioning (top/bottom)~~
- [ ] Smooth animations

### Configuration
- [x] Uses config.json for storing user preferences
   - [x] ~~Dock position~~
   - [ ] Theme settings
   - [ ] Custom shortcuts
   - [ ] Widget configurations
- [ ] Hot-reload configuration changes

## Technologies Used
[![Tech Stack](https://skillicons.dev/icons?i=rust,react,tailwind,tauri)](https://v2.tauri.app/)

## Build from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/akshat0012/rusty-dock.git
   cd rusty-dock
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Build the Tauri application:
   ```bash
   npm run tauri build
   ```

4. Run the application:
   ```bash
   npm run tauri dev
   ```

## Usage

## config.json Reference

| Key             | Possible Values                                                                |
| ----------------- | ----------------------- |
| "height" | int |
| "top-offset" | int |
| "left-offset" | int |
| "right-offset" | int |
| "bottom-offset" | int |
| "border-radius" | int |
| "theme" | "dark" or "light" |
| "position" | "top" or "bottom" |


1. Launch Rusty Dock after Downloading or Building.
2. Configure the configuration file (config.json)
3. Enjoy 🎉


