# Cargo WS Release

This crate needs a new name! And should maybe even be merged in with `cargo-release`

## Why

I'm working on [lockchain](https://github.com/spacekookie/lockchain) which is a bunch of loosely connected crates that depend on each other. When doing a release, I need to bump all the versions manually because they're in a workspace and all crates in a workspace need to build to publish. Not to mention that cargo release has some weird undefined behaviour when using it in a workspace (sometimes failing to write files, etc).


## Why tho?

I want to be able to type `cargo release-thing minor` and bump the version of the crate I'm working on, all dependencies of crates on that crate in the workspace, make a commit which bumps all the versions, then doing the same thing as `cargo-release` to make releases easier and automatic.


## Is it ready?

*snorts* Yea...no.


## Why put it on github then?

IT'S CALLED OFFSITE BACKUP, KAY? I'll probably [tweet](twitter.com/spacekookie) about it when the time comes. And ultimately, having these features in `cargo-release` might be a nice touch too.