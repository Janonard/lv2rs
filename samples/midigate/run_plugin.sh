cargo build --release;
cp target/release/libeg_midigate_rs.so eg-midigate-rs.lv2/midigate.so;
rm -r -f ~/.lv2/eg-midigate-rs.lv2/;
cp -r --parents eg-midigate-rs.lv2 ~/.lv2/;
jalv https://github.com/Janonard/eg-midigate-rs;