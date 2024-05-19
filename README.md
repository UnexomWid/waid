# About <a href="https://github.com/UnexomWid/eryn"><img align="right" src="https://img.shields.io/badge/ERYN-0.3-4527A0?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyBpZD0iaWNvIiB3aWR0aD0iMTAyNHB4IiBoZWlnaHQ9IjEwMjRweCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0gImh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiPg0KICAgIDxnIHRyYW5zZm9ybT0icm90YXRlKC0xMCwgNTEyLCA1MTIpIiBzdHlsZT0iZmlsbDogI0ZGRkZGRjsiPg0KICAgICAgICA8cGF0aCBkPSJNIDE5MSA5NCBDIDE4OS4zMzggOTQgMTg4IDk1LjMzOCAxODggOTcgTCAxODggMzI5IEMgMTg4IDMzMC42NjIgMTg5LjMzOCAzMzIgMTkxIDMzMiBMIDkzMiAzMzIgQyA5MzMuNjYyIDMzMiA5MzUgMzMwLjY2MiA5MzUgMzI5IEwgOTM1IDk3IEMgOTM1IDk1LjMzOCA5MzMuNjYyIDk0IDkzMiA5NCBMIDE5MSA5NCB6IE0gMzkgMzk0IEMgMzcuMzM4IDM5NCAzNiAzOTUuMzM4IDM2IDM5NyBMIDM2IDYyOSBDIDM2IDYzMC42NjIgMzcuMzM4IDYzMiAzOSA2MzIgTCA3ODAgNjMyIEMgNzgxLjY2MiA2MzIgNzgzIDYzMC42NjIgNzgzIDYyOSBMIDc4MyAzOTcgQyA3ODMgMzk1LjMzOCA3ODEuNjYyIDM5NCA3ODAgMzk0IEwgMzkgMzk0IHogTSAxOTEgNjk0IEMgMTg5LjMzOCA2OTQgMTg4IDY5NS4zMzggMTg4IDY5NyBMIDE4OCA5MjkgQyAxODggOTMwLjY2MiAxODkuMzM4IDkzMiAxOTEgOTMyIEwgOTMyIDkzMiBDIDkzMy42NjIgOTMyIDkzNSA5MzAuNjYyIDkzNSA5MjkgTCA5MzUgNjk3IEMgOTM1IDY5NS4zMzggOTMzLjY2MiA2OTQgOTMyIDY5NCBMIDE5MSA2OTQgeiAiIC8%2BDQogICAgPC9nPg0KPC9zdmc%2B" alt="eryn" /></a><a href="https://nodejs.org/en"><img align="right" src="https://img.shields.io/badge/NodeJS-16+-339933?logo=node.js" alt="NodeJS" /></a><a href="https://www.rust-lang.org/"><img align="right" src="https://img.shields.io/badge/Rust-1%2E73-f74c00?logo=Rust" alt="Rust 1.73" /></a>


**WAID** or *What the fuck am I doing?* is a Windows tool that tracks how you use your time.

It has 2 components

- the Rust client (Windows-only), which detects what you're doing in real time and reports to the server
- the Node.js server which stores and shows you reports about how you spend your day

<p align="center">
  <img src="demo.png" alt="demo">
</p>

# Installing

You first need to install [Rust](https://www.rust-lang.org/) and [Node](https://nodejs.org/en).

Then clone this repo and run the setup:

```sh
git clone https://github.com/UnexomWid/waid

cd waid

setup.bat
```

...and that's it ツ

To run WAID, all you have to do is this:

```sh
start.bat
```

This will open 2 terminal tabs. You have to keep both tabs open.

In the client tab you'll see info like the current window title, detected activity, etc.

Every ~5 minutes, the client will send the data to the server. You can see statistics via the browser:

```
http://localhost:3010
```

Because updates happen every 5 minutes, you will only see a blank page if you just started the tool. Just wait a bit.

Why only every 5 minutes? Because there's no point in doing it more often at the moment.

You can use the left/right arrow keys to move between days more quickly.

To change which activities you want the tool to detect, see below.

# Configuration

The default configuration is ready to go, but you can customize your activity categories as follows.

## Client

Open the file `client/waid.json`.

### Categories

You can edit the existing categories, which are used to detect what kind of activity you are doing based on the window that your mouse cursor is hovering on.

Use the default config as an example. If it's still not clear how to add your custom categories, continue reading below.

`window_title` is an array of regexes; if the current window title matches any of these, the activity is set to that specific category.

`process_path` is the same, but it looks at the process path of the window. **Use `\\\\` for path separators, otherwise the regexes won't match!**.

You can include both of those fields inside a category if you need to.

The priority is based on the order of the categories. The first category is checked first; if it doesn't match, the second is checked, and so on.

Therefore, you should place categories with **more specific regexes** first, and categories with **more general regexes** last.

If you **add new categories**, make sure you **assign a color** for them in the [server config](#Server).

### Other

`user_inactive_threshold` (in seconds) means that if you are away from the computer longer than this, the program
will stop tracking what you do since you are away. Being away means not using your mouse or keyboard. When you come back,
it will resume.

`server.endpoint` is the server URL that the client will send requests to in order to send the data.
When the client sends the data successfully, it is erased locally. Make sure this matches what's in the server config.

`server.send_frequency` (also in seconds) is how often the client should report the data to the server. 5 mins aka 300s is
a good compromise.

## Server

Open the file `server/config.json`. You can change the hostname/port. **Make sure they match what's in the client config**.

The server config also stores a color for each category, which is used for rendering charts.
The list of categories should be the same as in the client config.
If you **add a new category** in the client config, make sure to **assign a color** for it here.

# How it works

Instead of looking at the current focused window, the tool looks at the window over which you are hovering your cursor.

Why? Because you could have focus on the Visual Studio window, while scrolling on Reddit.

It detects the window title and process path, and picks the current activity based on the config file.

If you don't use your keyboard or mouse for a period of time (see the client config), the tool will stop tracking until you are active again.

The client periodically sends the tracked time for each activity to the server. If this is successful, it will erase the tracking data locally,
because it has been saved on the server.

The request to the server includes a secret which is configurable in the client and server configs, and they should obviously match.
This secret defaults to `placeholder` since the server is configured to run on localhost by default. If you want to host the server somewhere,
make sure to use a random secret so that no one else except your client can make updates on the server.

# Todo

- detect games made with Unity
- add more ways to detect activities (e.g. based on the process name)
- use a cache to store pairs of PID -> detected activity

# License <a href="https://github.com/UnexomWid/waid/blob/master/LICENSE"><img align="right" src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT" /></a>

**WAID** was created by [UnexomWid](https://uw.exom.dev). It is licensed under [MIT](https://github.com/UnexomWid/waid/blob/master/LICENSE-MIT) OR [Apache 2](https://github.com/UnexomWid/waid/blob/master/LICENSE-APACHE).