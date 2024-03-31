# minesweeper

Minesweeper implemented in terminal

![image](https://github.com/Martan03/minesweeper/assets/46300167/7ee06ad9-15d5-4d6a-a6f1-55fc36f9793b)

## Installation:
You have to compile it yourself, but that shouldn't be a problem. Only thing you need is `cargo`:
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

![image](https://github.com/Martan03/minesweeper/assets/46300167/7ee06ad9-15d5-4d6a-a6f1-55fc36f9793b)

When you start the game, you will see the board in the middle and number of flags left above
the board on the left side. You can then use arrow keys to change selected cell. By pressing
`d` or `Enter` you reveal currently selected cell. To place/remove flag, you can press `f`.

When you fill the whole board and it's correct, you will see Victory message above the board
on the right side.

![image](https://github.com/Martan03/minesweeper/assets/46300167/2c0e4cdd-797a-4f23-88cf-eedaa2ba9cd9)

By pressing `i` you can display help with all other keybind that I didn't mention.

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
