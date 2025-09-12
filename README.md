# Countries app

Alpha version 0.4.5
![Game welcome](/pictures/geo_welcome.png "Game welcome")
![Game mode](/pictures/geo_mode.png "Game mode")
![Geographical game](/pictures/geo_game.png "Geographical game")

---

#### Build from source (MacOS)

***First way:***
To build from source you must have installed [cargo](https://www.rust-lang.org),
to check this you can use in your terminal command `cargo --version`.
Then you must jump to folder `GeoGame-master` and run
`cargo build --release`, when build to be successful created you can run file `build_app_sh` in folder `macos`. If you make all without errors, build also be ended witch no errors.

***Second way:***
Also how in first way you must check your `cargo`, then
in terminal go to folder `GeoGame-master/macos` and run `./build_app_sh --all`, application must be created.

#### Build from source (Windows)

***Attention!***
To build on Windows, you must have [Python3](https://www.python.org) installed.

Use the same instructions as in build points 1 and 2 for macOS. But you must run file `build_app.py` in folder `windows`. If you do everything right, you will also have an application created in the root of the folder.
