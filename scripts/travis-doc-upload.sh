#!/bin/sh
set -e

# License: CC0 1.0 Universal
# https://creativecommons.org/publicdomain/zero/1.0/legalcode
#
# To encrypt the SSH key:
#
#     travis encrypt-file <privkey>
#
# More details: https://github.com/kmcallister/travis-doc-upload/blob/master/README.md

# Configuration
PROJECT_NAME=telegram-groups-bot
DOCS_REPO=coredump-ch/rust-docs.git
SSH_KEY_TRAVIS_ID=ee923565f942

# Exit if the branch isn't master
[ "$TRAVIS_BRANCH" = master ]

# Exit if this is a pull request
[ "$TRAVIS_PULL_REQUEST" = false ]

# Exit if this isn't the rust stable build
[ "$TRAVIS_RUST_VERSION" = stable ]

# Set some variables
eval key=\$encrypted_${SSH_KEY_TRAVIS_ID}_key
eval iv=\$encrypted_${SSH_KEY_TRAVIS_ID}_iv

# Install deploy SSH key
mkdir -p ~/.ssh
openssl aes-256-cbc -K $key -iv $iv -in scripts/github_rust_docs_key.enc -out ~/.ssh/id_ecdsa -d
chmod 600 ~/.ssh/id_ecdsa

# Get docs repo
git clone --branch gh-pages git@github.com:$DOCS_REPO deploy_docs

# Upload docs
cd deploy_docs
git config user.name "travis doc upload bot"
git config user.email "nobody@example.com"
rm -rf $PROJECT_NAME
mv ../target/doc $PROJECT_NAME
git add -A $PROJECT_NAME
git commit -qm "doc upload for $PROJECT_NAME ($TRAVIS_REPO_SLUG)"
git push -q origin gh-pages
