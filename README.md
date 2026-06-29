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

