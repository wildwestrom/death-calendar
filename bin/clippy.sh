#!/bin/sh
cargo clippy -- \
	-W clippy::all \
	-W clippy::pedantic \
	-W clippy::nursery \
	-W clippy::restriction \
	-W clippy::cargo \
	-A clippy::missing-docs-in-private-items \
	-A clippy::integer_division \
	-A clippy::integer_arithmetic \
	-A clippy::implicit_return \
	-A clippy::separated_literal_suffix
