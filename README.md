# Axo

A small NixOS package helper written in Rust.

Axo manages a simple package list and generates a NixOS module (`generated.nix`) that can be imported into your system configuration. Instead of manually editing `configuration.nix` every time you want to add or remove packages, Axo keeps track of your packages and updates the generated module for you.

## Features

* Add packages to your system configuration
* Remove packages from your system configuration
* Search available Nix packages
* Automatically generate a NixOS module
* Prevent duplicate package entries
* Simple text-based package tracking

## Installation

### Using flakes

Clone the repository:

```bash
git clone <repository-url>
cd axo
```

Install Axo:

```bash
nix profile install .
```

or run it directly:

```bash
nix run .
```

## Setup

Axo generates:

```
/etc/nixos/generated.nix
```

Your `/etc/nixos/configuration.nix` needs to import it:

```nix
imports = [
  ./hardware-configuration.nix
  ./generated.nix
];
```

After adding the import, rebuild your system:

```bash
sudo nixos-rebuild switch
```

## Usage

### Search for packages

```bash
Axo search <package>
```

Example:

```bash
Axo search firefox
```

### Add a package

```bash
Axo add <package>
```

Example:

```bash
Axo add kitty
```

Axo will update its package list and regenerate `generated.nix`.

After adding packages, rebuild your system:

```bash
sudo nixos-rebuild switch
```

### Remove a package

```bash
Axo remove <package>
```

Example:

```bash
Axo remove kitty
```

Then rebuild:

```bash
sudo nixos-rebuild switch
```

## How it works

Axo stores your package list at:

```
~/.config/axo/pkgs.txt
```

Example:

```
firefox
kitty
neovim
```

It then generates a NixOS module:

```nix
{ pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    firefox
    kitty
    neovim
  ];
}
```

This module is imported by your NixOS configuration.

## Requirements

* NixOS
* Rust toolchain (for building from source)
* Nix flakes enabled
