cargo build $(if [ -n "$1" ]; then echo "--$1"; fi)
cp ./target/${1:-debug}/libdmi_diff.so ./index.node
