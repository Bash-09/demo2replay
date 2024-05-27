# demo2replay

Create a Replay from a demo file for TF2

# Usage

Windows: Just run the .exe!

Linux: Mark the file as an executable (or `chmod +x demo2replay`) and then run it with `./demo2replay`

1. When it starts it will attempt to automatically locate your TF2 directory, but if that fails you can browse and set the directory manually.
2. Click the "Select demo file" button to choose your demo
3. (optional) Click "Select thumbnail" to set a custom thumbnail, otherwise the default "replay 2 demo" image will be used
4. Click "Create Replay"
5. If you see "Successfully created replay!", then you are good to launch TF2 and view your replay!

# Other info

- Replays are stored in your TF2 folder at `Team Fortress 2/tf/replay/client/replay/`, including both the demo file and a `.dmx` file
- Replay thumbnails are stored as a `VTF` file (and accompanying `VMT`) at `Team fortress 2/tf/materials/vgui/replay/thumbnails/`
- Replays are not easily portable between users, so if you want to share a replay with someone else I recommend sharing just the demo file and getting the other person to run this tool on it to generate their own
- Deleting some replay files can cause the in-game replay UI to mess up, so I recommend not touching the files yourself, or if you are going to delete any replay files you should just delete them all at once
- If there is an issue about any files or folders not being found, try creating the folders `Team Fortress 2/tf/replay/client/replay/` and `Team fortress 2/tf/materials/vgui/replay/thumbnails/` yourself before trying again

# Building

Using Rust, download `rustup` and run `cargo run` from inside the repository.

# Example Images

![image](https://github.com/Bash-09/demo2replay/assets/47521168/d0bc76dc-448c-4d41-a125-7383d13deb5c)
![image](https://github.com/Bash-09/demo2replay/assets/47521168/d229939c-699f-46f2-974c-033f3be54478)
