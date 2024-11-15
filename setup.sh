#!/bin/sh

# This script sets up the development environment.
# This script is supposed to run within a git repository clone.

# Exit on error; unset variables are an error.
set -eu

# Helper:
# Ensure root privileges for the installation.
# In a testing container, we are root but there is no sudo.
if [ $(id --user) != 0 ]; then
  SUDO=sudo
  if [ $($SUDO id --user) != 0 ]; then
    echo "We are not root and cannot sudo, cannot continue."
    exit 1
  fi
else
  SUDO=""
fi

# Required packages
$SUDO zypper --non-interactive install \
  gcc \
  gcc-c++ \
  make \
  rustup \
  libzypp-devel || exit 1

rustup install stable
