
all: hw3.elf

hw3.elf: hw3.rs
	rustc -o hw3.elf hw3.rs

clean:
	-rm hw3.elf
