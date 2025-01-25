# Rusty Dock
Rusty Dock is a customizable and lightweight desktop dock application built using **React**, **Rust**, and **Tauri**. It provides (not yet) an efficient interface for managing app shortcuts, widgets, and system information like battery and network status, all in one place.

> "As they say, if you don’t like a software, build your own. so I made my own :) "

## Features
### Core Features
- [ ] App Shortcuts: Add, remove shortcuts
- [ ] Drag-and-Drop Support
- [ ] System Info Widget (CPU, memory, disk usage)
- [ ] Battery Status Widget
- [ ] Network Status Widget

### Customization Options
- [ ] Themes and colors
- [x] Transparency and Border control
- [ ] Adjustable icon sizes
- [x] ~~Dock positioning (top/bottom)~~

### Configuration
- [x] Uses config.json for storing user preferences
   - [x] ~~Dock position~~
   - [ ] Custom shortcuts
   - [ ] Widget configurations
- [x] ~~Hot-reload configuration changes~~

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

### Configuration File

The configuration file `config.json` should be located at:

#### Absolute Path (for Windows)
`C:\ProgramData\rusty-dock\config.json`

## Dock Settings

| **Key**           | **Description**                                |
|--------------------|-----------------------------------------------|
| **`height`**       | Height of the dock in `px`.                  |
| **`bg_color`**     | Background color of the dock #RRGGBBAA.       |
| **`position`**     | Position of the dock (`"top"` or `"bottom"`). |
| **`top_offset`**   | Space between the dock and the top of the screen in `px`. |
| **`left_padding`**  | Space between the dock and the left side of the screen in `px`. |
| **`border_radius`** | Radius of dock corners in `px`.             |
| **`right_padding`** | Space between the dock and the right side of the screen in `px`. |
| **`bottom_padding`**| Space between the dock and the bottom of the screen in `px`. |


## Example Configuration

Below is an example configuration for the dock settings:

```json
{
  "dock_settings": {
    "height": 50,
    "top_padding": 3,
    "position": "top",
    "left_padding": 5,
    "border_radius": 16,
    "right_padding": 0,
    "bottom_padding": 5,
    "bg_color": "18181870"
  }
}


1. Launch Rusty Dock after Downloading or Building.
2. Configure the configuration file (config.json)
3. Enjoy 🎉
