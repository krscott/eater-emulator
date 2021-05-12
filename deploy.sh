#!/bin/bash
set -eu

./build_web.sh

# @3.0.0 required until this bug is fixed:
#   https://github.com/tschaub/gh-pages/issues/354
npx gh-pages@3.0.0 -d dist

