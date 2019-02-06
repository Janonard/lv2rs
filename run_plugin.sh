cargo build --release;
cp target/release/libexamp_rs.so ExAmp.lv2/libExAmp.so;
rm -r -f ~/.lv2/ExAmp.lv2/;
cp -r --parents ExAmp.lv2 ~/.lv2/;
jalv https://github.com/Janonard/ExAmp;