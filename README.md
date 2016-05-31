Cargo git hooks
===

This is a very simple cargo build script that will symlink a folder called
`hooks` in the root of your cargo project into `.git/hooks` so that your
developers are all using the same git hooks. Requires zero configuration.

Just do

``` bash
$ cargo add git_hooks
$ cargo build
```

And you're all set, your hooks should be linked in!

```
$ ls -l .git
total 48
-rw-r--r--   1 nm46057  staff   15 May 29 11:59 COMMIT_EDITMSG
-rw-r--r--   1 nm46057  staff   23 May 29 11:41 HEAD
-rw-r--r--   1 nm46057  staff  324 May 29 12:00 config
-rw-r--r--   1 nm46057  staff   73 May 29 11:41 description
lrwxr-xr-x   1 nm46057  staff   39 May 29 11:58 hooks -> $YOUR_PROJECT_ROOT/hooks
-rw-r--r--   1 nm46057  staff  405 May 29 11:59 index
drwxr-xr-x   3 nm46057  staff  102 May 29 11:41 info
drwxr-xr-x   4 nm46057  staff  136 May 29 11:59 logs
drwxr-xr-x  11 nm46057  staff  374 May 29 11:59 objects
drwxr-xr-x   5 nm46057  staff  170 May 29 12:00 refs
```

Security Warning
===

Commit hooks are arbitrary commands which are run around certain git interactions.
Blindly accepting git hooks from strangers is a bad idea because it would allow them to do anything within the hook, for example, wiping your entire filesystem or installing malicious software.
Sharing git hooks is a pattern which should really only be done within a trusted group.
Forcing git hook use on a public project may receieve ire from the community.
