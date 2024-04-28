# How to play?

### Where to play?
[Online](https://siiir.github.io/brick_bird/) **Note: Chrome browser higly recommended** for playing, due to perfomance.  
[Download for desktop](https://github.com/Siiir/brick_bird/releases/) **Note:** The game executable must be **put next to the asset folder** before running.
### Objective
Score as high **PS** (Passed Sectors count variable) as possible.
### Game difficulty
Marked by **MS** (Movement Speed) variable.
Increases with each passed sector as simulation accelerates.
### Tutorial
You fly automatically.  
To avoid obstackles jump.  
**Tip**: Do it interchangably using all functional buttons.  
#### Jump with
1. Keyboard – click \<space\> button.  
2. Mouse – click \<left button\>.  
3. Touch screen – touch anywhere on the game viewport.  


# Build & run the app

### Desktop version
#### Fast start
1. Go to the folder where you want to install (put) the game.  
2. Use the following bash commands:  
    ```bash
    git clone github.com/siiir/brick_bird
    cd brick_bird
    git switch stable
    cargo build --release
    ```
    These obviously requires having these bash commands installed.  
3. When you want to run the game:  
    1. go to instalation folder  
    2. enter command `cargo run -r`  
#### Better installation (optimization, click-to-run)
1. Ask AI model for help. Show it this README.md file & "Cargo.toml" file.  
2. See build profiles in "Cargo.toml".  

### Browser version
1. Go to the folder where you want to install (put) the game.  
2. Use the following bash commands:  
    ```bash
    git clone github.com/siiir/brick_bird
    cd brick_bird
    git switch web-stable
    ```
    These obviously requires having these bash commands installed.  
3. When you want to run:  
    1. go to instalation folder  
    2. run static file server (eg. Node's http-server or Python3's http.server)  
    3. open game folder (or index.html file) in the browser  


# How this educational project stands out?
### Own physic engine.
{collisions, gravity, acceleration, turbulence} have been coded from scrath using **math**.  
### Just code
Was created without any graphical editor.  
### Unique & Pro
1. Very modular and scalable.  
2. Uses unique features of Rust programing languages.  
