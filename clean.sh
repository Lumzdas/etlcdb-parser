#!/bin/bash

set -e

clear

mkdir -p data/images

for i in ETL1 ETL2 ETL3 ETL4 ETL5 ETL6 ETL7 ETL8B ETL8G ETL9B ETL9G
do
  echo cleaning "data/images/$i ..."
  rm -rf "data/images/$i"
  mkdir "data/images/$i"
done

cargo build --release
cargo run --release
