# Death Calendar

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
# Move into it
cd death-calendar
# Compile it
cargo build --release
# Now copy the newly compiled file to a location on your $PATH
cp target/release/death-calendar [DESTINATION]
```

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
- Make a logarithmic calendar
- Represent your time as an hourglass

## License

Death Calendar: Calculate how much time you have until your ultimate demise.  
Copyright © 2021 Christian Westrom

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with this program. If not, see <https://www.gnu.org/licenses/>.
