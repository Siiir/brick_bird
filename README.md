# How to play?
### Where to play?
[Online](https://siiir.github.io/brick_bird/) **Note: Chrome browser higly recommended** for playing, due to perfomance.  
// [Desktop]()
### Playing tutorial
You fly automatically.  
To avoid obstackles jump.  
Tip: Do it interchangably using all buttons.  
##### Jump with
1. Keyboard – click <space> button.  
2. Mouse – click <left button>.  
3. Touch screen – touch anywhere on the game viewport.  

# Build & run the app

### Desktop version
##### Fast start
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
  a) go to instalation folder  
  b) enter command `cargo run -r`  
##### Better installation
1. Ask AI model. Show it this README.md file.  
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
  a) go to instalation folder  
  b) run static file server (eg. Node's http-server or Python3's http.server)  
  b) open game folder (or index.html file) in the browser  
