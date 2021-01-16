# Current setup is tailored to a single handler only for demonstration purposes,
# yet it should not be too difficult to adapt it for an "all 4 triggers" setup.
# You also do not need to stick to make, of course.

# Using some of the shorthand variables, see docs here:
# https://www.gnu.org/software/make/manual/html_node/Automatic-Variables.html

WASM_OUTDIR = build.wasm_pkg
FN_OUTDIR   = build.function
ZIP_OUTDIR  = lambda
OUTDIRS     = $(ZIP_OUTDIR) $(FN_OUTDIR) $(WASM_OUTDIR)

WASM_TARGET = $(WASM_OUTDIR)/mod_bg.wasm
FN_TARGET   = $(FN_OUTDIR)/origin-request.js \
              $(FN_OUTDIR)/mod_bg.wasm
ZIP_TARGET  = $(ZIP_OUTDIR)/function.zip

build: $(FN_TARGET)

wasm: $(WASM_TARGET)

# create the wasm package
$(WASM_TARGET):
	cd rust \
		&& wasm-pack build \
			--target nodejs \
			--release \
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
	cd rust && cargo test --bins --examples --tests --benches --all-targets --all-features
	cd rust && wasm-pack test --node

clean:
	rm -rf $(OUTDIRS) $(ZIP_TARGET)

ci: build call zip
