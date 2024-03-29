# Death Calendar

![docs](https://github.com/wildwestrom/death-calendar/actions/workflows/docs.yml/badge.svg)
![tests](https://github.com/wildwestrom/death-calendar/actions/workflows/tests.yml/badge.svg)
![format](https://github.com/wildwestrom/death-calendar/actions/workflows/format.yml/badge.svg)
![lint](https://github.com/wildwestrom/death-calendar/actions/workflows/lint.yml/badge.svg)
![build](https://github.com/wildwestrom/death-calendar/actions/workflows/build.yml/badge.svg)
![package](https://github.com/wildwestrom/death-calendar/actions/workflows/package.yml/badge.svg)

![Example Logarithmic Calendar](./images/demo-img-log.svg)

> Generated with:
>
> ```console
> death-calendar img 2012-5-11 --lifespan-years=99 \
> --color-primary=64727D --color-secondary=2D3436 \
> --scale-factor=15 -o=images/demo-img-log.svg \
> log --width-height-ratio=8
> ```

![Example Grid Calendar](./images/demo-img-grid.svg)

> Generated with:
>
> ```console
> death-calendar img 2012-5-11 \
> --color-primary=64727D --color-secondary=2D3436 \
> --scale-factor=3 -o=images/demo-img-grid.svg \
> grid --week-shape=circle --length=8 --border=1 --border-unit=shape
> ```

Ever wonder how many days you have left to live?  
Death Calendar can show you at a glance.

The main feature is its SVG calendar generator. It's meant to be used as a
reminder of your mortality, so that you can make the most of your limited time
on Earth.

This was 100% inspired by the
[waitbutwhy.com](https://waitbutwhy.com/2014/05/life-weeks.html) article,
visualizing human life using various charts.

## Installation

Make sure you have a working rust toolchain.

```shell
# Shallow clone the repository
git clone --depth 1 https://github.com/wildwestrom/death-calendar
# Install it via cargo install
cargo install --path death-calendar
```

Make sure `$CARGO_HOME/bin/` on your `$PATH` so you can run it.

## Goals

- Make an easy installer
- Make it so the calendar updates every day automatically
- Make a graphical interface for customizing the look of the calendar
- Should work on multiple platforms
  - Linux
  - MacOS
  - Windows
  - Android
  - iOS

### New Calendar Render Ideas

- Make a spiral calendar
- Represent your time as an hourglass

### Non Goals (For Now)

- Calculate estimated lifespan based on lifestyle, income, genetics, etc.

## License

Death Calendar: Calculate how much time you have until your ultimate demise.  
Copyright © 2021 Christian Westrom

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
