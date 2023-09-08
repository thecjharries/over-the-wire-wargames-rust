# Aliases for executables
CARGO ?= cargo
GH ?= gh
GIT ?= git
RM ?= rm
XDG_OPEN ?= xdg-open

COVERAGE_THRESHOLD ?= 60

# Get the current branch
CURRENT_BRANCH := $(shell $(GIT) rev-parse --abbrev-ref HEAD)
WARGAME := $(word 2, $(subst /, ,$(CURRENT_BRANCH)))
LEVEL := $(word 3, $(subst /, ,$(CURRENT_BRANCH)))

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

# Ensure specific variable is nonempty
# https://stackoverflow.com/a/7367903
.PHONY: guard-%
guard-%:
	@ if [ "${${*}}" = "" ]; then \
		echo "Environment variable $* not set"; \
		exit 1; \
	fi

# Create the stub for a wargame level
.PHONY: git-stub
git-stub: guard-WARGAME guard-LEVEL
	$(GIT) commit src/$(WARGAME) -m "Stub $(WARGAME) $(LEVEL)"

# Create the test for a wargame level
.PHONY: git-test
git-test: guard-WARGAME guard-LEVEL
	$(GIT) commit src/$(WARGAME) -m "Test $(WARGAME) $(LEVEL)"

# Implement a wargame level
.PHONY: git-implement
git-implement: guard-WARGAME guard-LEVEL
	$(GIT) commit src/$(WARGAME) -m "Solve $(WARGAME) $(LEVEL)"

# Add a wargame level password
.PHONY: git-pass
git-pass: guard-WARGAME guard-LEVEL
	$(GIT) commit settings/$(WARGAME).yaml -m "Add $(WARGAME) $(LEVEL) password"

# Disable a wargame level's tests
.PHONY: git-disable
git-disable: guard-WARGAME guard-LEVEL
	$(GIT) commit src/$(WARGAME) -m "Disable $(WARGAME) $(LEVEL) to reduce connections"
