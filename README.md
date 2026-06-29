# AXO: A simple package manager for NixOS

AXO is a lightweight tool aimed at making package management on NixOS easier to handle.

⚠️ Note: AXO currently does not support flakes (work in progress). It requires sudo privileges.

## Installation
### 1. Clone the repository
git clone https://github.com/nugget071215/Axo
cd Axo
### 2. Move the main script
mv axo ~
### 3. Move AXO into NixOS config directory
mv ~/Axo /etc/nixos
### 4. Add AXO to your system
#### Option A: Home Manager alias
programs.bash = {
  enable = true;
  shellAliases = {
    axo = "~/path/to/axo/file";
  };
};
#### Option B: Add to PATH

Temporary:

export PATH="$PATH:/absolute/path/to/folder"

Permanent (~/.bashrc):

export PATH="$PATH:/absolute/path/to/folder"
### 5. Enable AXO in NixOS

Add this to configuration.nix:

imports = [
  ./axo/axo.nix
];

Then rebuild:

sudo nixos-rebuild switch
## Usage

AXO provides simple package management commands:

install – Install packages from Nixpkgs
uninstall – Remove installed packages
enable – Enable installed packages (mainly window managers)
disable – Disable installed packages
list – Show installed packages
update-db – Update local database

## Notes
Do not delete files inside the repository unless you understand their purpose.
.git folder can be safely removed if you're not contributing to development.
Run AXO commands with sudo when required.
