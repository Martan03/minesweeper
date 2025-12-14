# minesweeper

Minesweeper TUI implementation in Rust 🦀

![image](https://github.com/Martan03/minesweeper/assets/46300167/bb6d9047-36e4-405a-a19b-5d6490ddc815)

## Installation:

You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is `cargo`:

```bash
cargo build -r
```

After its done compiling, you can start it in `./target/release/minesweeper`

## Usage:

Running minesweeper without any arguments behaves based on the configuration.
By default it opens difficulty picker, where you can select difficulty and
start playing:

```bash
./minesweeper
```

Start the minesweeper game with given difficulty:

```bash
./minesweeper -d <easy|medium|hard>
```

You can also create your own difficulty by running:

```bash
./minesweeper -c <board width> <board height> <number of mines>
```

Every usage is shown in the program help:

```bash
./minesweeper -h
```

## What is Minesweeper?

**Minesweeper** is a puzzle game where your objective is to clear a rectanguler
board containing hidden mines without detonating any of them.

In the beginning all the cells are hidden. You can reveal any cell to see, what
is underneath. Each cell contains a **number of mines** around it
_(8 neighboring cells)_. If the number is zero, the cell is empty
_(doesn't have any number)_.

Based on the numbered cells you have to **deduce** where the mines are located
and mark them with a **flag**. If you flag all the mines, you win the game. But
if you denote even a single mine, the game is over.

## Detailed description

### Difficulty picker:

When you start the game without previously setting the default difficulty or
running the program with CLI arguments setting the difficulty, you will be
greeted with this difficulty picker page. It contains a list of three
predefined difficulties you can choose from. You can use arrow keys or vim
motion keys (`jk`) to change the selected difficulty. To confirm you choice,
press `Enter`.

You can also open it from the game screen by pressing `Tab`.

### Game screen:

![image](https://github.com/Martan03/minesweeper/assets/46300167/bb6d9047-36e4-405a-a19b-5d6490ddc815)

After you choose a difficulty you want to play, you will be greeted with the
board in the middle. You can also see number of flags left above the board on
the left side. You can then use arrow keys or vim motion keys (`hjkl`) to
change the selected cell. By pressing `d` or `Enter` you reveal the currently
selected cell. There's also special reveal feature, where when you reveal
already revealed cell and the number of flags around it matches its number, it
reveals all the neighboring cells. To place/remove flag, you can press `f` key.

When you fill the whole board and it's correct, you will see Victory message
above the board on the right side.

If you finish a game, or you fail one, you can reset the board by pressing `r`.
You can also press `c` to select the board's center cell.

### Help screen

![image](https://github.com/Martan03/minesweeper/assets/46300167/447d6824-de3b-4784-b281-9b18678f8495)

By pressing `i` you can toggle between help screen and game screen. It contains
list of all the described keybinds above.

### Configuration

Some players prefer playing certain difficulty and for that reason there's
configuration. You can open the config file by running:

```bash
./minesweeper config
```

Which opens the config file in editor based on the `EDITOR` environment
variable. You can change the `default_difficulty` to any of the difficulties
and when running without arguments, the game of that difficulty will be
started without showing you the difficulty picker. By default the value is
`null`, which opens the difficulty picker on start.

You can set it to predefined difficulty:

- `"Easy"`
- `"Medium"`
- `"Hard"`

Or you can set it to a custom difficulty:

```json
{
    "Custom": {
        "width": 20,
        "height": 10,
        "mines": 30
    }
}
```

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [termint](https://github.com/Martan03/minesweeper)
- **Author website:** [martan03.github.io](https://martan03.github.io)
