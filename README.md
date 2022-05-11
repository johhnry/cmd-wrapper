# Command wrapper

## Install

To run the script in dev mode:

```bash
cargo run -- --pre="whoami" --cmd="ls"
```

To build an executable in release mode:

```bash
cargo build --release
```

## Usage

```bash
cmd-wrapper --pre="whoami" --pre="uname" --cmd="pwd" --post="df -h"
```

```
----------------------------- PRE COMMAND 0 BEGIN -----------------------------
josephhenry
----------------- PRE COMMAND 0 END in 1.21ms (exit status: 0) -----------------

----------------------------- PRE COMMAND 1 BEGIN -----------------------------
Linux
--------------- PRE COMMAND 1 END in 809.20µs (exit status: 0) ---------------

------------------------------ MAIN COMMAND BEGIN ------------------------------
/home/josephhenry/silex/cmd-wrapper
----------------- MAIN COMMAND END in 24.21ms (exit status: 0) -----------------

----------------------------- POST COMMAND 0 BEGIN -----------------------------
Filesystem      Size  Used Avail Use% Mounted on
/dev/sdb        251G   11G  228G   5% /
tmpfs            13G     0   13G   0% /mnt/wsl
tools           239G  227G   13G  95% /init
none             13G     0   13G   0% /dev
none             13G  4.0K   13G   1% /run
none             13G     0   13G   0% /run/lock
none             13G     0   13G   0% /run/shm
none             13G     0   13G   0% /run/user
tmpfs            13G     0   13G   0% /sys/fs/cgroup
drivers         239G  227G   13G  95% /usr/lib/wsl/drivers
lib             239G  227G   13G  95% /usr/lib/wsl/lib
C:\             239G  227G   13G  95% /mnt/c
D:\             1.9T  477G  1.4T  26% /mnt/d
--------------------- POST COMMAND 0 END (exit status: 0) ---------------------
```

## Notes

- You can provide multiple `--pre` arguments to run multiple pre-commands in order. Same with `--post`.

- post-commands will be executed even if the main command fails
