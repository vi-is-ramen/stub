check:
    @echo "  With features: [ .. ]"
    @cargo clippy -- -D warnings
    @echo
    @echo "  With features: [ nightly ]"
    @cargo clippy --features nightly -- -D warnings

    @# Unbuildable as independent crate if no_core
    @# @echo "Features: [ nightly, no_core ]"
    @# @cargo clippy --features nightly,no_core -- -D warnings
