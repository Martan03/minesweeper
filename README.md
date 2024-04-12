# minesweeper

Minesweeper implemented in terminal

![image](https://github.com/Martan03/minesweeper/assets/46300167/bb6d9047-36e4-405a-a19b-5d6490ddc815)

## Installation:
You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is `cargo`:
```
cargo build -r
```
After its done compiling, you can start it in `./target/release/minesweeper`

## Usage:
Start minesweeper with default difficulty *(medium)*:
```
./minesweeper
```

Start minesweeper with different difficulty:
```
./minesweeper -d <easy|medium|hard>
```

Or you can create your custom difficulty:
```
./minesweeper -c <board width> <board height> <number of mines>
```

## Detailed description

### Game screen:

![image](https://github.com/Martan03/minesweeper/assets/46300167/bb6d9047-36e4-405a-a19b-5d6490ddc815)

When you start the game, you will see the board in the middle and number of
flags left above the board on the left side. You can then use arrow keys to
change selected cell. By pressing `d` or `Enter` you reveal currently selected
cell. To place/remove flag, you can press `f`.

When you fill the whole board and it's correct, you will see Victory message
above the board on the right side.

### Help screen

![image](https://github.com/Martan03/minesweeper/assets/46300167/447d6824-de3b-4784-b281-9b18678f8495)

By pressing `i` you can display help with all other keybind that I didn't
mention.

## Technologies:
I used these libraries:
- [crossterm](https://crates.io/crates/crossterm)
  - Creating key listeners
- [termint](https://crates.io/crates/termint)
  - Creating TUI itself
- [rand](https://crates.io/crates/rand)
  - Generating random number

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [termint](https://github.com/Martan03/minesweeper)
- **Author website:** [martan03.github.io](https://martan03.github.io)
