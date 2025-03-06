# Countryfetch

Countryfetch is a [neofetch](https://github.com/dylanaraps/neofetch)-like tool for fetching information about your country.

![united kingdom](https://github.com/user-attachments/assets/4fea8218-37e6-41cb-9dee-f88d1e9407af)

![japan](https://github.com/user-attachments/assets/49ed2991-c290-4859-b93a-07a068e23afa)

![united states](https://github.com/user-attachments/assets/b142255e-9d0d-4326-8a6a-9ab549bea861)

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

## Installation

### Linux / MacOS

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/nik-rev/countryfetch/releases/download/v0.1.5/countryfetch-installer.sh | sh
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

```sh
cargo install countryfetch
```
