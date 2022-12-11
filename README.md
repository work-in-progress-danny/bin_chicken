# Bin Chicken 🗑️🐔

Bin Chicken is a command line application written in the Rust programming language. It is designed to be a safer alternative to the `rm` command in Unix-like operating systems. Instead of permanently deleting files, Bin Chicken moves them to the system `bin`. This allows you to recover accidentally deleted files, while still providing a way to clean up your file system.

## Installing Bin Chicken

To install Bin Chicken, you will need to have the Rust programming language installed on your system. You can check if you have Rust installed by running the following command:

```bash
rustc --version
```

If you don't have Rust installed, you can follow the instructions on the [Rust website](https://www.rust-lang.org/) to install it.

Once you have Rust installed, you can install Bin Chicken by running the following command:

```bash
cargo install bin_chicken
```

## Using Bin Chicken

To use Bin Chicken, simply pass it the path of the file or directory you want to move to the bin. For example, if you want to move the file my_file.txt to the bin, you would run the following command:

```bash
bin_chicken my_file.txt
```

If you want to use Bin Chicken as a drop-in replacement for `rm`, you can create an alias by adding the following line to your `~/.bashrc` or `~/.zshrc` file:

```bash
alias rm="bin_chicken"
```

This will allow you to use `rm` as you normally would, but it will actually run Bin Chicken instead, moving the files to the `bin` instead of permanently deleting them.

## Limitations

Currently, Bin Chicken is MacOS only and does not support wildcard characters such as \* for matching multiple files or directories. You will need to specify each path individually.

## TODO

- [ ] Add support for undoing the last command
- [ ] Add support for \* wildcards
- [ ] Add support for other platforms

## Contributing to Bin Chicken

If you are interested in contributing to Bin Chicken, we would love to have your help! You can start by checking out the [ open issues ](https://github.com/Danny-Duck/bin_chicken/issues) on our GitHub repository to see if there is anything you can help with. You can also suggest new features or improvements by opening a new issue.

To contribute code to Bin Chicken, you will need to fork the repository and create a new branch for your changes. Once you have made your changes, you can submit a pull request for them to be reviewed and merged into the main codebase.

## License

Bin Chicken is released under the [MIT License](https://opensource.org/licenses/MIT).
