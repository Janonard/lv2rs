cargo build --release;
cp target/release/liburid_test.so UridTest.lv2/libUridTest.so;
rm -r -f ~/.lv2/UridTest.lv2/;
cp -r --parents UridTest.lv2 ~/.lv2/;
jalv https://github.com/Janonard/UridTest;