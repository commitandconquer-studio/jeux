+++
title = "Install the games"
template = "page.html"
+++

All our games are **free and open source**. Pick the way to play that suits you.

## 1. Play in your browser, nothing to install

The easiest way: open a game's page and click **▶ Play now**. The game takes a few seconds to load the first time (about ten megabytes), then starts instantly.

- You need a recent browser (Firefox, Chrome, Edge, Safari, Brave…) and a graphics card that supports WebGL2, which nearly all recent computers and phones do.
- Your progress and settings are saved in your browser.

## 2. Install the games on Linux

The games install into your applications menu (under **Games**), with no administrator rights. For now, they are built from the source code: allow about ten minutes the first time.

**Once only**: install Rust and the required libraries (Ubuntu, Debian, Linux Mint…).

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install git build-essential pkg-config libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
```

**Then** get the games and install them:

```
git clone https://github.com/commitandconquer-studio/jeux.git commit-and-conquer
cd commit-and-conquer
tools/install-linux.sh
```

That's it: the games show up in your applications menu. To update them later:

```
cd commit-and-conquer
git pull
tools/install-linux.sh
```

And to remove everything: `tools/install-linux.sh --desinstaller`.

## 3. Windows, macOS and Android

The **Windows**, **macOS** and **Android** versions are on their way. Meanwhile, you can play in your browser on these systems.

## A question, a problem?

Write to us on the [studio's GitHub repository](https://github.com/commitandconquer-studio/jeux/issues) (“Issues” tab): all feedback is welcome.
