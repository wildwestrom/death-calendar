#!/bin/sh
cargo clippy -- \
	-W clippy::all \
	-W clippy::pedantic \
	-W clippy::nursery \
	-W clippy::restriction \
	-W clippy::cargo \
	-A clippy::needless-return \
	-A clippy::missing-docs-in-private-items \
	-A clippy::integer_division \
	-A clippy::implicit_return \
	-A clippy::integer_arithmetic \
	-A clippy::separated_literal_suffix
