VERSION=v`vnext`
mkdir .tmp
cd .tmp
git clone git@github.com:harmony-labs/vnext.git
cd vnext
set-cargo-version Cargo.toml $VERSION
git add -A
git commit -m "chore(version): $VERSION"
git tag $VERSION
git push --tags
cd ../../
rm -rf .tmp/vnext