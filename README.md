# Countryfetch

Countryfetch is a [neofetch](https://github.com/dylanaraps/neofetch)-like tool for fetching information about your country.

![united kingdom](https://github.com/user-attachments/assets/4da5565b-76a2-408a-9342-4af2cb012f78)

![japan](https://github.com/user-attachments/assets/f36f1a77-4241-49e8-9179-0728e74217e2)

![united states](https://github.com/user-attachments/assets/63e15b08-7560-47d4-8d15-3f4f9962375d)

## Usage

Get information about your country:

```sh
countryfetch
```

Get information about specific countries:

```sh
countryfetch UnitedStates UnitedKingdom
```

You can also use 2-letter country codes:

```sh
countryfetch us gb
```

List all countries:

```sh
countryfetch --list-countries
```

Show information about all countries:

```sh
countryfetch --all-countries
```

## Installation

### Linux / MacOS

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/nik-rev/countryfetch/releases/latest/download/countryfetch-installer.sh | sh
```

### Linux / MacOS (Homebrew)

```sh
brew install nik-rev/tap/countryfetch
```

### Windows / MacOS / Linux (Powershell)

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/nik-rev/countryfetch/releases/latest/download/countryfetch-installer.ps1 | iex"
```

### Windows / MacOS / Linux (Cargo)

Note: You will need at least **Cargo 1.85** to build from source, using this method.

```sh
cargo install countryfetch
```
