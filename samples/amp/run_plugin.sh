cargo build --release;
cp target/release/libeg_amp_rs.so eg-amp-rs.lv2/libEgAmpRs.so;
rm -r -f ~/.lv2/eg-amp-rs.lv2/;
cp -r --parents eg-amp-rs.lv2 ~/.lv2/;
jalv https://github.com/Janonard/eg-amp-rs;