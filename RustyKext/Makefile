
RUSTC=/usr/local/bin/rustc
OBJECTS=main.o

RustObjs: $(OBJECTS)


%.o: %.rs
	$(RUSTC) -O --lib -o $@ -c $<

clean:
	@rm -f $(OBJECTS)
