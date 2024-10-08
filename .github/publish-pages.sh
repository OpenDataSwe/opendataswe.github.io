#!/bin/sh
set -ex
git checkout -b pages-tmp-branch
rm .gitignore
printf "target\nCargo.lock\n" > .gitignore
CWD=$(pwd)
cd ${GITHUB_WORKSPACE}
bash ./.cmd/build-fe.sh
cd $CWD
git config --global user.email open_data_swe@proton.me
git config --global user.name OpenDataSwe
git fetch origin
git add .
git commit -m "Publish new page version"
git push origin `git subtree split --prefix dist pages-tmp-branch`:gh-pages --force