# Scout Workshop

![https://img.shields.io/badge/license-MIT-green](https://img.shields.io/badge/license-MIT-green)

<p align="center">
  <img src="scout_workshop_banner.png" alt="Scout in a Dark Forest" center  />
</p>

Welcome to the Scout Workshop repository! This repository contains all the necessary materials for our workshop on using Scout, CoinFabrik's bug detection tool for Soroban smart contracts. 

Below you'll find a summary of the workshop agenda, installation guides, and examples we will work with during the sessions.

## Workshop Agenda

The agenda for the workshop is as follows:

1. **Intro and Tool Installation**
   - Overview of Scout
   - Key features and benefits
   - Installing the CLI
   - Installing the VSCode Extension
3. **Scout execution**
4. **Output interpretation**
5. **Practical example: using Scout to solve an issue**
6. **CI/CD Integration: adding Scout to your GitHub pipeline**
7. **VS Code Extension: adding Scout to your development environment**

For more details, please visit the [Workshop Landing Page](https://www.coinfabrik.com/scout-workshops/).

## Workshop Notes

### Installation

#### CLI

Make sure that [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is installed on your computer. Then, follow these 5 simple steps:

**1. Install Rust Nightly Toolchain:**

```bash
rustup toolchain install nightly-2023-12-16
```

**2. Set Default Nightly Toolchain:**

```bash
rustup default nightly-2023-12-16
```

**3. Add rust-src Component:**

```bash
rustup component add rust-src --toolchain nightly-2023-12-16
```

**4. Install additional tools required by Scout:**

```bash
cargo install cargo-dylint dylint-link mdbook
```

**5. Install Scout Audit:**

```bash
cargo install cargo-scout-audit
```

For more information on installation and usage, please refer to the [Getting Started](https://coinfabrik.github.io/scout-soroban/docs/intro) section in our documentation section below.

#### VSCode Extension

Add Scout to your development workspace with Scout's VS Code extension to run Scout automatically upon saving your file.

:bulb: Before installing Scout VS Code Extension, make sure to install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and Scout CLI.

:point_right: Install Scout VS Code from [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=CoinFabrik.scout-audit).

:bulb: Tip: To see the errors highlighted in your code, we recommend installing the [Error Lens Extension](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens).

### Excecution

#### CLI

To run Scout on your project, navigate to the directory of your smart contract and execute the following command:

```bash
cargo scout-audit
```

:warning: Currently Scout doesn't offer full support for workspaces. If you have a workspace, run Scout in each member instead of running it in the workspace `Cargo.toml`.

```
├── your-soroban-project
│   ├── your-smart-contract
│   │   ├── // Run Scout here.
│   │   ├── src
│   │   |   ├── contract.rs 
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── README.md

```
:warning: Make sure that your smart contracts compile properly. Scout won't run if any compilation errors exist.

#### VSCode Extension

:warning: To ensure the extension runs properly, make sure that you open the directory containing your smart contract, rather than the entire project. For example, if your smart contracts are located in `myproject/contracts`, and you want to work on the `token` contract while using the Scout VS Code Extension, open `myproject/contracts/token`.


### Output Interpretation

Scout's output follows the following format.

```console
warning: [Short description of the issue]
  --> src/lib.rs:[issue_line_start]:[issue_char_start]
   |
   |         some code;
   |         ^^^^^^^^^^^^^^^^^^
   |
   = help: [Recommendation on how to remediate the issue]
   = note: `#[warn([detector_name])]` on by default
```

Here:
- `warning` provides a short description of the detected issue.
- `help` provides a recommendation on how to fix the issue.
- `note` indicates the detector triggered, which is turned on by default on the user profile. Check on [profile configuration](https://coinfabrik.github.io/scout-soroban/docs/intro#profile-configuration) to include or exclude detectors from your analysis.

### Practical Example

Take a look at `example-1` and `example-2`. Let's run scout on these examples and see how to use the CLI and VSCode Extension.

### Scout Action: CI/CD Example





