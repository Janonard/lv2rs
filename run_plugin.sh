cargo build --release;
cp target/release/libexamp_rs.so ExAmp.lv2/libExAmp.so;
rm -f ~/.lv2/ExAmp.lv2/;
mkdir -p ~/.lv2/ExAmp.lv2/;
cp -r ExAmp.lv2 ~/.lv2/ExAmp.lv2/;
jalv https://github.com/Janonard/ExAmp;