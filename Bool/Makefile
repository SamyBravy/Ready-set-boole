EXERCISES := ex01 ex02 ex03 ex04 ex05 ex06 ex07 ex08 ex09 ex10 ex11 ex12

all: $(EXERCISES)

$(EXERCISES):
    rustc src/$@.rs -o bin/$@

clean:
    rm -f bin/ex*

.PHONY: all clean $(EXERCISES)