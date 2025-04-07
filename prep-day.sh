#!/bin/sh

# Downloads the input and sets up module boilerplate for
# the given day. Expects that a `.session` file exists with the
# user's session key from the Advent of Code website. See the
# README for instructions on how to set it up.
#
# This script should be idempotent, so don't worry about things
# breaking if you run it for the same day multiple times.
#
# Usage:
# ./prep-day.sh <day> [year]
# Example: ./prep-day.sh 10     # Uses current year or December's year
# Example: ./prep-day.sh 10 2023 # Explicitly use year 2023

# Error handling function
handle_error() {
  echo "ERROR: $1" >&2
  exit 1
}

# Determine the year to use
if [ -n "$2" ]; then
  # Use the year provided as the second argument
  YEAR=$2
else
  # Auto-detect the year based on current date
  CURRENT_MONTH=$(date +%m)
  CURRENT_YEAR=$(date +%Y)
  
  # If it's December or earlier in the year, use the current year
  # Otherwise, use the previous year (since AoC runs in December)
  if [ "$CURRENT_MONTH" -ge 12 ]; then
    YEAR=$CURRENT_YEAR
  else
    # For January-November, we're likely working on the previous year's puzzles
    YEAR=$CURRENT_YEAR
  fi
fi

echo "Using year: $YEAR"

# Create necessary directories
mkdir -p .input || handle_error "Failed to create .input directory. Check permissions."
mkdir -p "$YEAR" || handle_error "Failed to create $YEAR directory. Check permissions."

# Validate day parameter
if test -z "$1"; then
  handle_error "Must provide day of month (not zero-padded) as first argument"
fi

if [[ 1 -gt "$1" || 25 -lt "$1" ]]; then
  handle_error "Day must be between 1 and 25, inclusive"
fi

# Check for session file
if [ ! -f .session ]; then
  handle_error "Session file (.session) not found. Please create this file with your AoC session token.
See README.md for instructions on how to obtain your session token."
fi

# Read session token
SESSION=$(cat .session)
if test -z "$SESSION"; then
  handle_error "Session token is empty. Please add your AoC session token to the .session file.
See README.md for instructions on how to obtain your session token."
fi

# Download input data if needed
if test -e ".input/$1.txt"; then
  echo "Data already exists for day $1, skipping download..."
else
  echo "Downloading data for day $1 to .input/$1.txt..."
  
  # Create a temporary file for the response
  TEMP_FILE=$(mktemp)
  
  # Download with error handling
  HTTP_STATUS=$(curl "https://adventofcode.com/$YEAR/day/$1/input" \
    --silent --max-time 30 \
    --cookie "session=$SESSION" \
    --write-out "%{http_code}" \
    --output "$TEMP_FILE")
  
  # Check HTTP status code
  if [ "$HTTP_STATUS" -eq 200 ]; then
    # Check if the response contains an error message about invalid session
    if grep -q "Please log in" "$TEMP_FILE"; then
      rm "$TEMP_FILE"
      handle_error "Session token is invalid or expired. Please update your session token in the .session file.
See README.md for instructions on how to obtain a fresh session token."
    fi
    
    # Success - move the temp file to the final location
    mv "$TEMP_FILE" ".input/$1.txt"
    echo "Download successful!"
  elif [ "$HTTP_STATUS" -eq 404 ]; then
    rm "$TEMP_FILE"
    handle_error "Puzzle not available yet. Puzzles unlock at midnight Eastern Time (UTC-5)."
  elif [ "$HTTP_STATUS" -eq 500 ]; then
    rm "$TEMP_FILE"
    handle_error "Server error. The Advent of Code server might be overloaded. Please try again later."
  else
    rm "$TEMP_FILE"
    handle_error "Failed to download input (HTTP status $HTTP_STATUS). Check your internet connection and try again."
  fi
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

  # Check if the file was created successfully
  if [ ! -f "src/day$1.rs" ]; then
    handle_error "Failed to create src/day$1.rs. Check permissions and disk space."
  fi

  # Determine which sed to use
  SED="sed"
  if [ "$(uname -s)" = "Darwin" ]; then
    if command -v gsed >/dev/null 2>&1; then
      SED=gsed
    else
      handle_error "GNU sed (gsed) is required on macOS but not found. Please install it with 'brew install gnu-sed'."
    fi
  fi

  # Update main.rs
  if [ ! -f "src/main.rs" ]; then
    handle_error "src/main.rs not found. Make sure you're running this script from the repository root."
  fi

  "$SED" -i "s|// MOD_MARKER|mod day$1;\nuse day$1::Day$1;\n// MOD_MARKER|" src/main.rs || handle_error "Failed to update main.rs with module declaration."
  "$SED" -i "s|        // DAY_MARKER|        $1 => Box::new(Day$1::from_lines(lines)),\n        // DAY_MARKER|" src/main.rs || handle_error "Failed to update main.rs with day solution."

  echo "Updated main.rs:"
  git diff src/main.rs
fi

echo "Happy coding!"
