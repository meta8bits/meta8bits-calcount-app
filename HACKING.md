# Getting Started

To run the project locally, you need the following CLI tools:

- [docker CLI](https://docs.docker.com/engine/reference/commandline/cli/)
- [cargo](https://rustup.rs/)
- [pnpm](https://pnpm.io/)
- [concurrently](https://www.npmjs.com/package/concurrently)
- [cURL](https://curl.se/)
- [Make](https://formulae.brew.sh/formula/make)

The following ports also must be free on your machine:

- `5432` for PostgreSQL
- `8000` for this application

You will need to bootstrap the app and database by performing on offline
compilation using `./sqlx-data.json` -- there's a handy make rule to get you
started;

```
make bootstrap
```

After running the bootstrap rule, the app will be running, but it won't
live-reload. To run the typical dev scripts, stop the app and run the dev rule:

```
make dev
```

There are very few unit tests, but you can run them with:

```
cargo test
```

There are some utilities in the Makefile for working with the database. In
particular:

```
make shell-db  # attach to an interactive PostgreSQL shell inside the DB
make watch-db  # live-tail the database logs
```

Additionally, there is a rule for running CI just like it runs in CI!

```
make check
```

You will notice that there is a pre-push hook in `./githooks` which calls this
Make rule. I recommend running `git config --local core.hooksPath githooks` to
setup githooks for your local repo, which will run the checks locally before you
push, giving some faster feedback. This is only trul