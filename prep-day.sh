#!/bin/sh

set -e

# Downloads the input and sets up module boilerplate for
# the given day. Expects that a `.session` file exists with the
# user's session key from the Advent of Code website. See the
# README for instructions on how to set it up.
#
# This script should be idempotent, so don't worry about things
# breaking if you run it for the same day multiple times.
#
# Usage:
# ./prep-day.sh 10

YEAR=2024

mkdir -p .input
mkdir -p "$YEAR"

if test -z "$1"; then
  echo "Must provide day of month (not zero-padded) as first argument"
  exit 1
fi

if [[ 1 -gt "$1" || 25 -lt "$1" ]]; then
  echo "Day must be between 1 and 25, inclusive"
  exit 1
fi

SESSION=$(cat .session)
if test -z "$SESSION"; then
  echo "Must set the session from the Advent of Code site"
  exit 1
fi

if test -e ".input/$1.txt"; then
  echo "Data already exists for day $1, skipping download..."
else
  echo "Downloading data for day $1 to .input/$1.txt..."
  curl "https://adventofcode.com/$YEAR/day/$1/input" \
    --silent --max-time 10 --cookie "session=$SESSION" > ".input/$1.txt"
fi

if test -e "src/day$1.rs"; then
  echo "src/day$1.rs already exists, skipping..."
else
  echo "Creating boilerplate module for day $1 at src/day$1.rs..."

  cat <<-EOF > "src/day$1.rs"
use crate::{DaySolution, FromInput};

pub struct Day$1;

impl FromInput for Day$1 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        todo!("Parse your input from the input file");
        for l in _lines {
        }
    }
}

impl DaySolution for Day$1 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;
        todo!("Solve part one of day $1 using your parsed input");
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        todo!("Solve part two of day $1 using your parsed input");
        sum.to_string()
    }
}
EOF

SED="sed"
if [ "$(uname -s)" = "Darwin" ]
then
  SED=gsed
fi

"$SED" -i "s|// MOD_MARKER|mod day$1;\nuse day$1::Day$1;\n// MOD_MARKER|" src/main.rs
"$SED" -i "s|        // DAY_MARKER|        $1 => Box::new(Day$1::from_lines(lines)),\n        // DAY_MARKER|" src/main.rs

echo "Updated main.rs:"
  git diff src/main.rs
  #echo "  mod day$1;"
  #echo "  use day$1::Day$1;"
  #echo "  $1 => Box::new(Day$1::from_lines(lines)),"
fi

echo "Happy coding!"
