git fetch --all --tags -p && git rebase origin/main --autostash
VERSION=`vnext`
V_VERSION=v$VERSION
mkdir .tmp
cd .tmp
git clone git@github.com:unbounded-tech/vnext.git
cd vnext
set-cargo-version Cargo.toml $VERSION
git add -A
git commit -m "chore(version): $V_VERSION"
git tag $V_VERSION
git push --tags
cd ../../
rm -rf .tmp/vnext