# This is a template Makefile that I use for automating projects that might
# require one.
#
# For more guidance around Makefiles, the checkout out the helpful
# [makefiletutorial](https://makefiletutorial.com/).

check_defined = \
    $(strip $(foreach 1,$1, \
        $(call __check_defined,$1,$(strip $(value 2)))))
__check_defined = \
    $(if $(value $1),, \
      $(error Undefined $1$(if $2, ($2))))

.PHONY: clean
clean: ## Clean target directory.
	@cargo clean

.PHONY: release
release: clean ## Build a release binary for Tick.
	@cargo build --release

.PHONY: test
test: clean ## Run the tests for Tick.
	@cargo test

.PHONY: update
update: clean ## Update Cargo.lock file with Cargo.toml configuration.
	@cargo update

.PHONY: help
help: ## Outputs this help message.
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(firstword $(MAKEFILE_LIST)) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
