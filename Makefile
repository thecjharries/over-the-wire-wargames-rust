# Aliases for executables
CARGO ?= cargo
GH ?= gh
GIT ?= git
RM ?= rm
XDG_OPEN ?= xdg-open

COVERAGE_THRESHOLD ?= 60

# Get the current branch
CURRENT_BRANCH := $(shell $(GIT) rev-parse --abbrev-ref HEAD)

# Run the tests
.PHONY: test
test:
	$(CARGO) test

# Get code coverage
.PHONY: coverage
coverage:
	$(CARGO) tarpaulin -v --fail-under=$(COVERAGE_THRESHOLD)

# Build coverage report
.PHONY: coverage-report
coverage-report:
	$(CARGO) tarpaulin -v --fail-under=$(COVERAGE_THRESHOLD) --out HTML; $(XDG_OPEN) tarpaulin-report.html

# Convenience target to finish the feature branch
.PHONY: finish
finish: coverage clean
	$(GIT) push -u origin $(CURRENT_BRANCH)
	$(GH) pr create --fill
	$(GH) pr merge --merge --delete-branch

# Convenience target for making a patch commit on main.rs
.PHONY: patch
patch:
	$(GIT) add src --patch

# Remove any built artifacts
.PHONY: clean
clean:
	$(RM) -rf target
	$(RM) -rf Cargo.lock
	$(RM) -rf tarpaulin-report.html

# https://stackoverflow.com/a/7367903
guard-%:
	@ if [ "${${*}}" = "" ]; then \
		echo "Environment variable $* not set"; \
		exit 1; \
	fi
