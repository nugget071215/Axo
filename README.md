# AXO: A simple package manager for NixOS

## DIRECTIONS

### 1
Copy the github repo.
<pre> ```bash git clone https://github.com/nugget071215/Axo cd Axo``` </pre>

### 2
Move the file just named "Axo" out of the folder.
<pre> ```bash mv axo ~``` </pre>

### 3
Move the folder into /etc/nixos.
<pre> ```bash mv ~/Axo /etc/nixos``` </pre>

### 4
Either use home.nix and make an alias that executes the axo file, or move it to a folder that's in your path.
home.nix

### home.nix

<pre>
  ```nix
  programs.bash = {
    enable = true
    shellAliases = {
      axo = "~/path/to/axo/file"
    }
  }
  ```
</pre>

### ~/.bashrc
<pre> ```bash export PATH="$PATH:/absolute/path/to/the/desired/folder" ``` </pre>

### Temporary (Only for current terminal instance)
<pre> ```bash export PATH="$PATH:/absolute/path/to/the/desired/folder" ``` </pre>

