all: test

#
# Rust
#

test: test/spec test/int

test/spec:
	cargo test

test/int:
	cargo build
	rm -rf trial/cases/*/dist
	./trial/run.sh

#
# CI
#

sync/master:
	git checkout master
	git rebase origin/release
	git push origin master

sync/release:
	git checkout release
	git rebase origin/master
	git push origin release

ci/skip:
	./ci/scripts/skip.sh

#
# NPM world
#

ci/release:
	npx standard-version --no-verify --commit-all

ci/build:
	npx ncc build ci/src/cargo-toml.ts -o ci/dist/cargo-toml/

ci/clean:
	rm -rf ci/dist

ci/install:
	npm ci

#
# Git Hooks
#

clean-git-hooks:
	rm .git/hooks/*

setup-hooks: clean-git-hooks
	ln -f git-hooks/* .git/hooks
