# Current setup is tailored to a single handler only for demonstration purposes,
# yet it should not be too difficult to adapt it for an "all 4 triggers" setup.
# You also do not need to stick to make, of course.

# Using some of the shorthand variables, see docs here:
# https://www.gnu.org/software/make/manual/html_node/Automatic-Variables.html

PKG_NAME    = wasm_pkg
WASM_OUTDIR = build.$(PKG_NAME)
FN_OUTDIR   = build.function
ZIP_OUTDIR  = lambda
OUTDIRS     = $(ZIP_OUTDIR) $(FN_OUTDIR) $(WASM_OUTDIR)

WASM_TARGET = $(WASM_OUTDIR)/$(PKG_NAME)_bg.wasm
FN_TARGET   = $(FN_OUTDIR)/origin-request.js \
              $(FN_OUTDIR)/$(PKG_NAME)_bg.wasm
ZIP_TARGET  = $(ZIP_OUTDIR)/function.zip

build: $(FN_TARGET)

wasm: $(WASM_TARGET)

# create the wasm package
$(WASM_TARGET):
	cd rust \
		&& wasm-pack build \
			--target nodejs \
			--release \
			--out-name $(PKG_NAME) \
			--out-dir ../$(WASM_OUTDIR)

fn: $(FN_TARGET)

# create the final artefact which should be one JS and one WASM file only
$(FN_TARGET): $(WASM_TARGET)
	cd node \
		&& npm install \
		&& npm run build
	cp $^ $(@D)/
	ls -ahlF $(@D)/

rebuild: clean build

zip: $(ZIP_TARGET)

# zip the artefact for upload to AWS Lambda;
# keep in mind: for Lambda@Edge you must upload to us-east-1!
# furthermore pay attention to the zipped size, there are limits:
# origin req/res -> 50MB max, viewer req/res -> 1MB max
$(ZIP_TARGET): $(FN_OUTDIR)
	rm -rf $(@D)
	mkdir -p $(@D)
	rm -f $@
	cd $< && zip -9 -r ../$@ .
	ls -ahlF $@

# make a test call to the function;
# you can change details in the code to test different triggers
call:
	node fixtures/call-test.js

test:
	cargo test --bins --examples --tests --benches --all-targets --all-features
	cd rust && wasm-pack test --node

clean:
	rm -rf $(OUTDIRS) $(ZIP_TARGET)

ci: build call zip

ci.artifact: build zip

# customized step since actions-rs/audit-check@v1 does not support work dirs
ci.audit:
	@command -v cargo-audit >/dev/null || cargo install cargo-audit
	@cargo fetch && cargo generate-lockfile
	@cargo audit

ci.checks:
	@echo "-- clippy linting"
	@cargo fetch
	@cargo clippy --tests --examples -- -D warnings
	@echo "-- format checking"
	@cargo fmt --all -- --check
	@echo "-- documentation building"
	@cargo doc --no-deps
