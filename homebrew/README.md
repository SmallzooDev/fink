# Homebrew Tap for fink

This directory contains the Homebrew formula for installing fink.

## Installation

Once this is set up in a tap repository, users will be able to install fink using:

```bash
brew tap SmallzooDev/fink
brew install fink
```

## Setting up the Tap

1. Create a new GitHub repository named `homebrew-fink`
2. Copy the `fink.rb` formula to the root of that repository
3. Update the URLs and SHA256 in the formula
4. Push to GitHub

## Updating the Formula

When releasing a new version:

1. Update the `url` to point to the new release tarball
2. Update the `sha256` with the checksum of the new release
3. Commit and push the changes

The GitHub Actions workflow in `.github/workflows/release.yml` will help automate this process.