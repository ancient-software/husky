examples_dir=examples
tests_dir=tests

include examples/makefile

update-python-requirements:
	pipreqs ./ --force

# ad hoc
install-toolchain:
	mkdir -p ~/.huskyup/toolchains/nightly/lib/rustlib/src/husky/library
	rsync -a library ~/.huskyup/toolchains/nightly/lib/rustlib/src/husky/

vscode: install-toolchain
	# scripts/vscode_prepublish.sh
	# rsync -a extensions/husky-analyzer ~/.vscode/extensions/husky-lang.husky-analyzer-0.1.0
	cargo install --path crates/apps/husky-analyzer --bin husky-analyzer-server
	spd-say "vscode is done"

vscode-server: install-toolchain
	# scripts/vscode_prepublish.sh
	# rsync -a extensions/husky-analyzer/language-configuration.json ~/.vscode-server/extensions/husky-lang.husky-analyzer-0.1.0/language-configuration.json
	cargo install --path crates/apps/husky-analyzer --bin husky-analyzer-server

install-compiler:
	cargo install --path crates/apps/husky-compiler --bin husky-compiler

count-todo:
	scripts/pattern_statistics.py "todo!()" crates 1 10
	scripts/pattern_statistics.py "todo!()" crates 2 10

update-jar-tree:
	UPDATE_EXPECT=1 cargo test -p husky-jar-utils
	cargo test -p husky-jar-utils
	cargo check

update-expect: update-jar-tree
	scripts/update_expect.sh

update-expect-plain:
	scripts/update_expect_plain.sh

update-expect-package:
	scripts/update_expect_package.sh husky-text

update-expect-local:
	cargo fmt
	cargo check --tests
	UPDATE_EXPECT=1 cargo test --features "allow-print" -- --nocapture\
		|| scripts/play_update_expect_failure_music.sh
	scripts/play_update_expect_success_music.sh

update-expect-local-job-1:
	cargo fmt
	cargo check --tests -j 1
	UPDATE_EXPECT=1 cargo test --features "allow-print" -- --nocapture\
		|| scripts/play_update_expect_failure_music.sh
	scripts/play_update_expect_success_music.sh

update-expect-server:
	cargo fmt
	cargo check --tests
	UPDATE_EXPECT=1 cargo test --features "allow-print" -- --nocapture

ubuntu-setup:
	scripts/ubuntu_setup.sh

test-digitize:
	cargo run --bin digitize -- data/typical-huskies0/n02109961_57.JPEG

test-digitize-ultraman:
	cargo run --bin digitize -- data/ultraman/leo/images.jpeg

install-devtools:
	cargo install --path crates/devtools/cargo-organise

organise: install-devtools
	cargo organise

adversarial:
	# cargo test
	ADVERSARIAL_ROUND=1000 cargo test

run-notebook:
	cargo run --bin husky-notebook

mnist-developer:
	# SKIP_COMPILATION=1 cargo run --bin husky-mnist-classifier-developer
	cargo run --bin husky-mnist-classifier-developer

save:
# git add -A
# git commit -m "save"
# git push
	cargo fmt
	cargo install --path crates/devtools/git-save
	git-save

fix:
	scripts/fix.sh

save-clean:
	cargo fmt
	git add -A
	git commit -m "clean"
	git push

save-improve-debug:
	cargo fmt
	git add -A
	git commit -m "improve-debug"
	git push

syn-tree-build-timings:
	cargo fmt
	cargo clean
	cargo build -p husky-entity-tree --timings
	cp target/cargo-timings/cargo-timing.html benchmarks/syn-tree-build-timings.html

build-timings:
	cargo clean
	cargo build --timings
	cp target/cargo-timings/cargo-timing.html benchmarks/build-timings.html

test-build-timings:
	cargo clean
	cargo build --tests --timings
	cp target/cargo-timings/cargo-timing.html benchmarks/test-build-timings.html

doc:
	cargo doc --open

clean-mnist:
	cd examples/mnist-classifier/target-rs && cargo clean

mnist-game:
	cargo run -p husky-mnist-game

typst-egui:
	cargo run -p husky-typst-egui

eliminate-mods:
	cargo run -p cargo-eliminate-mods

chicago-typewriter:
	cargo run -p husky-chicago-typewriter