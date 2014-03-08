BIN_DIR = bin
RUSTC = rustc -O -Z lto
SRC_DIR = src

.PHONY: all clean

all:
	mkdir -p $(BIN_DIR)
	$(RUSTC) $(SRC_DIR)/eulermark.rs --out-dir $(BIN_DIR)

clean:
	rm bin/*
