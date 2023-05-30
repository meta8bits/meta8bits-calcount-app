
SHELL := /bin/bash
ENV=source .env &&
DB_CONTAINER_NAME := "calcount"

# The registry is presumed to be docker.io, which is the implicit default
DOCKER_ACCOUNT=jdevries3133
CONTAINER_NAME=calcount
ifdef GITHUB_SHA
	TAG=$(GITHUB_SHA)
else
	TAG=$(shell git rev-parse HEAD)
endif
CONTAINER_QUALNAME=$(DOCKER_ACCOUNT)/$(CONTAINER_NAME)
CONTAINER_EXACT_REF=$(DOCKER_ACCOUNT)/$(CONTAINER_NAME):$(TAG)

.PHONY: build
.PHONY: check
.PHONY: setup
.PHONY: dev
.PHONY: bootstrap
.PHONY: deploy
.PHONY: _start-db
.PHONY: _stop-db
.PHONY: watch-db
.PHONY: shell-db
.PHONY: build-container
.PHONY: debug-container
.PHONY: push-container

check: setup
ifdef CI
	pnpm run build
endif
ifndef CI
	@# Locally, we want to ensure that `cargo sqlx prepare` was run, otherwise
	@# the build will fail in CI. So, we'll run an offline build as part of
	@# our checks
	SQLX_OFFLINE=true cargo build
endif
	cargo clippy -- -D warnings
	cargo fmt --check
	terraform fmt --check
	cargo test

build: setup
	pnpm run build
	cargo build --release

setup:
	[[ ! -f ./src/htmx-1.9.10.vendor.js ]] \
		&& curl -L https://unpkg.com/htmx.org@1.9.10 > src/htmx-1.9.10.vendor.js \
		|| true
ifdef CI
	npm i -g pnpm
endif
	[[ ! -d node_modules ]] \
		&& pnpm install \
		|| true
ifndef CI
	@# we only want the `.env` file locally in practice. We never run the app
	@# in CI (yet). The problem with having the `.env` file in CI is that
	@# sqlx will pickup on the `DATABASE_URL` environment variable and try
	@# to talk to a datbase that isn't there, causing compilation to fail.