# AXO: A simple package manager for NixOS

## DIRECTIONS

DO NOT DELETE ANY OF THE FILES WITHIN THE REPO, THEY'RE ALL IN THERE FOR A REASON (except for the .git folder you can get rid of that)

### 1
Copy the github repo.
<pre> git clone https://github.com/nugget071215/Axo cd Axo </pre>

### 2
Move the file just named "Axo" out of the folder.
<pre> mv axo ~ </pre>

### 3
Move the folder into /etc/nixos.
<pre> mv ~/Axo /etc/nixos </pre>

### 4
Either use home.nix and make an alias that executes the axo file, or move it to a folder that's in your path.
home.nix

### home.nix

<pre>
  programs.bash = {
    enable = true
    shellAliases = {
      axo = "~/path/to/axo/file"
    }
  }
</pre>

### ~/.bashrc
<pre> export PATH="$PATH:/absolute/path/to/the/desired/folder" </pre>

### Temporary (Only for current terminal instance)
<pre> export PATH="$PATH:/absolute/path/to/the/desired/folder" </pre>

### 5
Go to configuration.nix and put ./axo/axo.nix into the imports section.
<pre>
  imports = [
    (other imports)
    ./axo/axo.nix
  ];
</pre>

## USAGE
Axo is a package manager aimed at making NixOS much easier to deal with. As of June 29th 2026, it does not support flakes, though that is being worked on.
It must be ran with sudo privileges

Theres a few options for Axo

install: Installs any packages that are available in NixPkgs. (not supporting flakes)
uninstall: Removes any packages that have been installed.

enable: Will enable any package that has been installed. (very prone to bugs so only use for window managers)
disable: Will disable any package that has been installed.

list: Lists all packages that have been installed using axo.

update-db: Updates the database that is stored locally.

You are able to view this again by inputting only "axo"
