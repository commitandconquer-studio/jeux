+++
title = "Install the games"
template = "page.html"
+++

All our games are **free and open source**. Pick the way to play that suits you.

## 1. Play in your browser, nothing to install

The easiest way: open a game's page and click **▶ Play now**. The game takes a few seconds to load the first time (about ten megabytes), then starts instantly.

- You need a recent browser (Firefox, Chrome, Edge, Safari, Brave…) and a graphics card that supports WebGL2, which nearly all recent computers and phones do.
- Your progress and settings are saved in your browser.

## 2. Install the launcher on Linux

The **Commit & Conquer launcher** installs all our games, keeps them up to date and starts them. It adds itself to your applications menu (under **Games**), with no administrator rights and nothing to compile.

The quickest way: paste this line into a terminal.

```
mkdir -p /tmp/lanceur-cc && curl -sL https://github.com/commitandconquer-studio/jeux/releases/latest/download/lanceur-linux-x86_64.tar.gz | tar xz -C /tmp/lanceur-cc && /tmp/lanceur-cc/lanceur
```

Or by hand: [download the launcher](https://github.com/commitandconquer-studio/jeux/releases/latest/download/lanceur-linux-x86_64.tar.gz), extract the archive, then open the `lanceur` file (right-click, “Run as a program”).

On its first run, it installs itself in `~/.local/share/commit-and-conquer` and shows up in the applications menu as **Commit & Conquer**. Then **Install** downloads a game, **Play** opens it, and **Update** appears when a new version comes out.

To remove everything, delete the `~/.local/share/commit-and-conquer` folder and the `commit-and-conquer*.desktop` files in `~/.local/share/applications`.

**Rather build it yourself?** The source code and the `tools/install-linux.sh` script are on the [studio's GitHub repository](https://github.com/commitandconquer-studio/jeux).

## 3. Windows, macOS and Android

The **Windows**, **macOS** and **Android** versions are on their way. Meanwhile, you can play in your browser on these systems.

## A question, a problem?

Write to us at [contact@commitandconquer.fr](mailto:contact@commitandconquer.fr), or on the [studio's GitHub repository](https://github.com/commitandconquer-studio/jeux/issues) (“Issues” tab): all feedback is welcome.
