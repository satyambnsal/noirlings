# NOIRLINGS

### An interactive tutorial to get you up and running with Noir

---

A hands-on, interactive way to learn Noir programming language through practical exercises. Perfect for both beginners and developers looking to sharpen their Noir skills.

<table>
  <tr>
    <td>
      <a href="https://twitter.com/intent/follow?screen_name=NoirLang">
         <img src="https://img.shields.io/twitter/follow/NoirLang?label=Follow Noir&style=social" />
      </a >
      </td>
    <td>
        <a href="https://twitter.com/intent/follow?screen_name=satyambnsal">
            <img src="https://img.shields.io/twitter/follow/shrameetweets?label=Follow%20Satyam&style=social" />
        </a>
    </td>
  <tr>
<table>

## Setup and run

Make sure you have both Rust and Noir installed:

1. Install Rust and Cargo:

```sh
curl https://sh.rustup.rs -sSf | sh -s
```

2. Install Noir:

```sh
curl -L https://raw.githubusercontent.com/noir-lang/noirup/refs/heads/main/install | bash
noirup
```

3. Clone and enter the repository:

```sh
git clone https://github.com/satyambnsal/noirlings.git
cd noirlings
```

4. Run noirlings (first run may take some time):

```sh
cargo run -r --bin noirlings
```

5. Start watch mode to begin the exercises:

```sh
cargo run -r --bin noirlings watch
```

## How Noirlings Works

Noirlings is designed to help you learn Noir through hands-on exercises:

1. Each exercise is a Noir file containing a problem to solve
2. Run `noirlings watch` to start - you'll see an error message for the first exercise
3. Open the exercise file in your editor and start solving
4. Type `hint` in watch mode for help, or run `cargo run -r --bin noirlings hint exercise_name`
5. Remove the `// I AM NOT DONE` comment when you've solved an exercise
6. Watch mode automatically verifies your solution and moves to the next exercise

## VSCode Support

For syntax highlighting and language support:

1. Install the [Noir VSCode Extension](https://marketplace.visualstudio.com/items?itemName=noir-lang.noir-programming-language)
2. Open the project folder in VSCode

## Contributing

### Adding New Exercises

1. Create exercise file in `./exercises/<module_name>/<exercise_name>.nr`
2. Add exercise metadata to `info.toml`:

```toml
[[exercises]]
name = "exercise_name"
path = "exercises/module/exercise_name.nr"
mode = "test"
hint = "Your helpful hint here"
```

3. Test your exercise:

```sh
cargo run -r --bin noirlings run exercise_name
```

4. Run test suite
5. Submit a PR!

### Testing

Run specific tests:

```sh
cargo test noir
```

Run all tests:

```sh
cargo test
```

## Noir Resources

- [Official Noir Documentation](https://noir-lang.org/docs)
- [Noir GitHub Repository](https://github.com/noir-lang/noir)
- [Awesome Noir](https://github.com/noir-lang/awesome-noir)
- [Noir Examples](https://github.com/noir-lang/noir/tree/master/examples)

## Credits

- Inspired by [Rustlings](https://github.com/rust-lang/rustlings)
- Based on [Starklings](https://github.com/shramee/starklings-cairo1)
