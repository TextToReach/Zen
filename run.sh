if cargo build; then
	clear;
	./target/debug/ZenBackend "$@";
else
	echo "Build Failed.";
fi