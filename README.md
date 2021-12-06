# Rust Statistics Calculator

A TI-80s series calculator emulation made using the Rust programming language.

Also my first project with Rust!

### How To Run

[Install Rust here.](https://doc.rust-lang.org/book/ch01-01-installation.html)

Clone the project using 

> `git clone https://github.com/kevinMEH/rust-statistics-calculator`

Navigate to the main directory with 

> `cd rust-statistics-calculator`

Run the program by using 

> `cargo run`

### How to use

Follow the instructions on screen to use the program.

Answer simple questions with `yes` or `no`.

Available responses for more complex questions will be displayed in all caps, such as `EDIT | CALC | EXIT`.

#### Memory

Before the start of the calculator, you can read saved lists from the `memory.json` file.

Once you quit the calculator, you will have the option to save your lists to `memory.json`.

#### Adding Lists

To add a list, use the `EDIT` command, and then select the list you want to edit.

Put in all the numbers of the list you want to edit. Make sure that the delimiters for the numbers are consistant. For example, `1, 4, 5.5, 3` works, and so does `1 5 7 9 2.42` and even `6y27y9y25.5y2`, but not `1, 5 9 67, 52.2  2`.

The program supports custom delimiters. If you have a list that looks like this: `4:56:25.2:8` or this: `6y27y9y25.5y2`, select `CUSTOM` as your delimiter and enter in `:` or `y`.

## Credits

[Learn more about the Rust programming language.](https://www.rust-lang.org/)
