# rust-ci-setup-bors

You can fetch this repository to setup rust CI with bors in a project.

In your project:

```
git checkout -b add-ci
git fetch github.com/matthiasbeyer/rust-ci-setup-bors master
git merge --no-ff --allow-unrelated-histories FETCH_HEAD
```

As this repository contains a default rust package (generated via `cargo init`),
you might need to resolve some conflicts.

We need to cargo package to be able to run automated CI jobs on this repository,
for example for updating actions.

You might also need to remove this README.

## License

(C) 2023 Matthias Beyer

WTFPL

