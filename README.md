# Togetherer

Put all files within a C# project into on singular file.

This tool aims to simplify the development of more complicated MCGalaxy plugins by encouraging the habit of structuring code in different files.

## Installation

This tool is written in Rust, so you need to have that installed. Then run

```sh
cargo install togetherer
```

## Usage

Go to the directory of of your C# project, and run

```sh
togetherer
```

You should see a single output file at `./out/YourProject.cs`.

### Rules

Your C# project should follow the following rules.
- Files and directories you want to include must start with an uppercase character.

---

That's it, open an issue on GitHub if anything is not working.
