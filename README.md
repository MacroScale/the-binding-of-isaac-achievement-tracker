# The Binding of Isaac Achievement Tracker

[![Website](https://img.shields.io/website?url=https%3A%2F%2Fwww.tboisecrets.com)](https://www.tboisecrets.com)

> An online platform that uses your Steam data to display your achievement progress per character and overall in the hit game **The Binding of Isaac** by Ed McMillen.

---

## Table of Contents

- [About](#about)
- [Features](#features)
- [Architecture](#architecture)
- [Technologies Used](#technologies-used)
- [Configuration](#configuration)
- [Development](#development)
- [Roadmap](#roadmap)
- [License](#license)
- [Credits](#credits)

---

## About

The Binding of Isaac Achievement Tracker is designed for fans of **The Binding of Isaac** who want an easy way to track their progress across the game’s many characters and unlockables.

Instead of manually checking achievements in Steam or in-game menus, this platform fetches your Steam data (with your permission) and displays:

- A complete list of all available achievements
- Per-character achievement progress
- The lastest YouTube videos from popular Isaac youtubers

Check it out live at: [https://www.tboisecrets.com](https://www.tboisecrets.com)

---

## Features

✨ **Steam Integration**  
- Connect your Steam account via API to fetch your Isaac achievements
- Automatically updates your progress whenever you refresh your data

🎮 **Per-Character View**  
- See which characters still need unlocks
- Plan your runs more efficiently

📊 **Progress Overview**  
- Global progress bar for all achievements
- Easy identification of missing achievements

🔗 **Latest videos from popular Isaac Youtubers**  
- YouTube API integration to link directly to popular isaac youtubers

📰 **Latest Game updates from Steam**  
- grabs latest patches via Steam API 

🖥️ **Responsive Web UI**  
- Works on desktop and mobile

---

## Architecture

This project consists of:

- **Rust Backend**  
    - Handles:
      - Communicating with the Steam API
      - Communicating with YouTube API
      - Serving the web frontend
- **Frontend (HTML/CSS/JS Askama templated)**  
    - Displays:
      - Achievement tables
      - Character-specific pages
      - Charts or progress bars
- **Database** (PostgreSQL recommended)  
    - Stores:
      - logs of requests to debug failed requests 

All deployed as a **Docker container**, making it easy to run anywhere.

---

## Technologies Used

- Rust (main application backend)
- Docker
- PostgreSQL
- Steam Web API
- YouTube Data API
- HTML / CSS / JS (frontend)

---

## Configuration

The app is configured entirely via environment variables:

| Variable            | Description                                        |
|----------------------|----------------------------------------------------|
| `RUST_LOG`           | Logging level, e.g. `info`                         |
| `STEAM_API_KEY`      | Your Steam Web API key                             |
| `YOUTUBE_API_KEY`    | YouTube API key (optional, for video guides)       |
| `DATABASE_HOST`      | Hostname of your database                          |
| `DATABASE_PORT`      | Port of your database                              |
| `DATABASE_USER`      | Database username                                  |
| `DATABASE_PASSWORD`  | Database password                                  |

---

### How to Track Your Achievements

1. Enter steamID on site
2. Allow the app to fetch your public game data
3. Browse:
   - Per-character achievement lists
   - Overall progress bar
   
---

## Development

### Prerequisites

- Rust (stable toolchain)
- Docker (optional)
- PostgreSQL running locally (if not using Docker)
- A Steam API Key

### Running Locally

Clone the repository:

```bash
git clone https://github.com/pgxtips/tboi-achievement-tracker.git
cd tboi-achievement-tracker
````

Install dependencies and build:

```bash
cargo build
cargo run
```

.env:

```bash
RUST_LOG=info \
STEAM_API_KEY=<your_steam_api_key> \
DATABASE_HOST=localhost \
DATABASE_PORT=5432 \
DATABASE_USER=postgres \
DATABASE_PASSWORD=secret \
cargo run
```

---

## Roadmap

* ✅ Steam API integration
* ✅ Per-character progress view
* ⏳ User login and account management
* ⏳ Improved mobile layout
* ⏳ Localization / multi-language support

---

## License

This project is open source but does not currently carry an explicit license. If you wish to use or contribute, please reach out to the author.

---

## Credits

Built with love by:

* **Brandon Gill (pgxtips)**

  * [GitHub: pgxtips](https://github.com/pgxtips)

Special thanks to:

* Ed McMillen for creating The Binding of Isaac
* Steam Web API

---

## Screenshots

*Needs to be done*

---
