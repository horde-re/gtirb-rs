# Contributing

Welcome stranger!

If you have come here to learn how to contribute to gtirb-rs, we have some tips for you!

First of all, don't hesitate to ask questions!
Use the [issue tracker](https://github.com/horde-re/gtirb-rs/issues), no question is too simple.

### Issues to work on

todo

### Building gtirb-rs

gtirb-rs builds on stable Rust, if you want to build gtirb-rs from source, here are the steps to follow:

1. Navigate to the directory of your choice
2. Clone this repository with git

    ```
    git clone https://github.com/horde-re/gtirb-rs.git
    ```

3. Navigate into the newly created `gtirb-rs` directory
4. Run `cargo build`

The resulting library can be found in `gtirb-rs/target/debug/`.

### Code Quality

We love code quality and Rust has some excellent tools to assist you with contributions.

#### Formatting Code with rustfmt

Before you make your Pull Request to the project, please run it through the `rustfmt` utility.
This will ensure we have good quality source code that is better for us all to maintain, following the [Rust Style Guide](https://doc.rust-lang.org/beta/style-guide/index.html).

[rustfmt](https://github.com/rust-lang/rustfmt) has a lot more information on the project.
The quick guide is

1. Install it (`rustfmt` is usually installed by default via [rustup](https://rustup.rs/)):

    ```
    rustup component add rustfmt
    ```

2. You can now run `rustfmt` on a single file simply by...

    ```
    rustfmt src/path/to/your/file.rs
    ```

    ... or you can format the entire project with

    ```
    cargo fmt
    ```

    When run through `cargo` it will format all bin and lib files in the current package.

For more information, such as running it from your favourite editor, please see the `rustfmt` project. [rustfmt](https://github.com/rust-lang/rustfmt)

#### Finding Issues with Clippy

[Clippy](https://doc.rust-lang.org/clippy/) is a code analyser/linter detecting mistakes, and therefore helps to improve your code.
Like formatting your code with `rustfmt`, running clippy regularly and before your Pull Request will help us maintain awesome code.

1. To install

    ```
    rustup component add clippy
    ```

2. Running clippy

    ```
    cargo clippy
    ```

### Change requirements

Please consider the following when making a change:

* Almost all changes that modify the Rust code must be accompanied with a test.

* Almost all features and changes must update the documentation.
  See [gtirb-rs Documentation](https://horde-re.github.io/gtirb-rs/) whose source is at <https://github.com/horde-re/gtirb-rs/tree/main/docs>.

* Almost all Rust items should be documented with doc comments.
  See the [Rustdoc Book](https://doc.rust-lang.org/rustdoc/) for more information on writing doc comments.

* Breaking the API can only be done in major SemVer releases.
  These are done very infrequently, so it is preferred to avoid these when possible.
  See [SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html) for more information on what a SemVer breaking change is.

* Check out the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) for guidelines on designing the API.

### Making a pull-request

When you feel comfortable that your changes could be integrated into gtirb-rs, you can create a pull-request on GitHub.
One of the core maintainers will then approve the changes or request some changes before it gets merged.

### Git usage

We use the [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/) specification to format our commits, pull requests and branches. This allows us to keep a good project history and simplify the making of changelogs.

That means that for our workflow, we use the following branch names :

* hotfix : `hotfix/<hotfix name>`
* feature : `feat/<feature name>`
* documentation : `doc/<documentation changed>`
* develop : `dev`
* ...

![Git workflow](https://miro.medium.com/v2/resize:fit:1100/format:webp/1*tnvRls6Dg7vFt0zGdtfu_w.png "Git workflow")

## Publishing new releases

Instructions for gtirb-rs maintainers to publish a new release:

1. Create a PR to update the version :
    1. Update the version in `Cargo.toml`
    2. Run `cargo test` to verify that everything is passing, and to update `Cargo.lock`.
    3. Double-check for any SemVer breaking changes.
       Try [`cargo-semver-checks`](https://crates.io/crates/cargo-semver-checks).
    4. Commit the changes, and open a PR.
2. After the PR has been merged, create a release in GitHub describing the changelog.
