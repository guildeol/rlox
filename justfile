setup:
    @pip install -r requirements.txt

build:
    @cargo build

unit-tests: build
    @cargo test

integration-tests: build
    @python -m pytest

all-tests: unit-tests integration-tests
