# dco
A simple tool for adding a [DCO](https://wiki.linuxfoundation.org/dco) Signed-off-by: line to a git commit message

It takes a commit message on stdin, and spits it out with the DCO line in the right place:
```
$ cat demo
commit title

message body

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# On branch main
# Changes to be committed:
#       modified:   README.md
```
```
$ dco < demo
commit title

message body

Signed-off-by: User Person <user@localhost>
# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# On branch main
# Changes to be committed:
#       modified:   README.md
```
## Installation

`cargo install dco`

## Configuration
I initially made this for the [Helix](https://helix-editor.com/) editor, but there's nothing specific to it. I'd be happy to include configuration blurbs for any editors submitted.

### Helix
In `languages.toml`:
```
[[language]]
name = "Git"
language-id = "Git"
scope = "source.git"
formatter = { command = "dco" }
file-types = ["COMMIT_EDITMSG"]
roots = [".git"]
```

## Usage
While editing a git commit message, type `:format`
