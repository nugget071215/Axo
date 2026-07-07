# AXO: A simple package manager for NixOS

AXO is a lightweight tool aimed at making package management on NixOS easier to handle.

⚠️ Note: Axo requires sudo privileges. Flake management does not include uninstalling.

## Installation
### 1. Clone the repository
<pre> 
git clone https://github.com/nugget071215/Axo
cd Axo
</pre>

## Optional:
use the install.sh script in the repo, however it will assume you'll have the folders and put them in predetermined places. If you want to customize them then do the manual install

### 2. Move the main script
<pre> mv axo ~ </pre>
### 3. Move AXO into NixOS config directory
<pre> mv ~/Axo /etc/nixos </pre>
### 4. Add AXO to your system
#### Option A: Home Manager alias
<pre> 
programs.bash = {
  enable = true;
  shellAliases = {
    axo = "~/path/to/axo/file";
  };
};
</pre>
#### Option B: Add to PATH

Temporary:

<pre> export PATH="$PATH:/absolute/path/to/folder" </pre>

Permanent (~/.bashrc):

<pre> export PATH="$PATH:/absolute/path/to/folder" </pre>
### 5. Enable AXO in NixOS

Add this to configuration.nix:

<pre>
imports = [
  ./axo/axo.nix
];
</pre>

Then rebuild:

<pre> sudo nixos-rebuild switch </pre>
## Usage

AXO provides simple package management commands:

install – Install packages from Nixpkgs
uninstall – Remove installed packages
enable – Enable installed packages (mainly window managers)
disable – Disable installed packages
list – Show installed packages
update-db – Update local database
flakes - Installs a package that's a valid flake

## Notes
Do not delete files inside the repository unless you understand their purpose.
.git folder can be safely removed if you're not contributing to development.
Run AXO commands with sudo when required.
