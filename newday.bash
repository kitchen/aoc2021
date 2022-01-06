echo "creating day ${1}"

day="${1}"
dir="day${day}"

hx Cargo.toml

cargo new $dir
(
  cd $dir
  mkdir src/bin
  rm src/main.rs
  cp ../template/bin.rs src/bin/part1.rs
  cp ../template/bin.rs src/bin/part2.rs
  cp ../template/lib.rs src/lib.rs
  sed -I '' -e "s/DAY/${day}/g" src/bin/part1.rs src/bin/part2.rs src/lib.rs
  touch sample1.txt input.txt

  cargo add nom
  echo -n "1,2,3" | cargo run --bin part1 
  echo -n "1,2,3" | cargo run --bin part2 
)

git add Cargo.toml Cargo.lock ${dir}
