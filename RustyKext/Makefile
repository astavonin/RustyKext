
OBJECTS=main.o

RustObjs: $(OBJECTS)


%.o: %.rs
	rustc -O --lib -o $@ -c $<

clean:
	@rm -f $(OBJECTS)
