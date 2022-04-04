<h2> etlcdb-parser </h2>

This program produces separate BMP images for each handwritten or printed character from the [ETL Character Database](http://etlcdb.db.aist.go.jp/). Useful for ML applications. To get it running:

1. [Get rust](https://www.rust-lang.org/tools/install)
2. Request the image data from etlcdb by filing a download request [here](http://etlcdb.db.aist.go.jp/download-request).
3. Download all of the archives, and extract them into the `data/` directory, so that you have `data/ETL1`, `data/ETL2` and so on. Each subdirectory should contain files like `ETL1C_07`, `ETL1INFO`, etc.
4. Run `./clean.sh` - the resulting images will be contained in `data/images`. Rerunning the script will first delete all images and then recreate them (hence its name).

The handwritten or printed symbols are encoded as the first character of the image filename, eg.: for the symbol `ス`, we get a filename `ス-ETL1C_08-6997.bmp`. The second part of the filename (`ETL1C_08`) is the data file it originated from, and the last (`6997`) - the index of the symbol within said data file.
