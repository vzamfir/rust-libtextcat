CC=gcc
FLAGS=-Wall -fPIC -c -g -O3
OUT_DIR?=out
LIBTEXTCAT_DIR?=libtextcat-2.2/src

LIBTEXTCAT_SOURCES=$(filter-out $(LIBTEXTCAT_DIR)/createfp.c,$(filter-out $(LIBTEXTCAT_DIR)/textcat.c,$(filter-out $(LIBTEXTCAT_DIR)/testtextcat.c,$(wildcard $(LIBTEXTCAT_DIR)/*.c))))
OBJECTS=$(notdir $(patsubst %.c,%.o,$(LIBTEXTCAT_SOURCES)))

target: static_lib

static_lib: $(OUT_DIR)/liblanguage_detection.a

$(OUT_DIR)/liblanguage_detection.a: $(OBJECTS) $(OUT_DIR)/language_detection.o
	ar -crs $(OUT_DIR)/liblanguage_detection.a $(addprefix $(OUT_DIR)/,$(OBJECTS)) $(OUT_DIR)/language_detection.o

# special rule for language_detection.o
$(OUT_DIR)/language_detection.o: src/language_detection.c
	$(CC) $(FLAGS) src/language_detection.c -o $(OUT_DIR)/language_detection.o -I$(LIBTEXTCAT_DIR)

%.o: $(addprefix $(LIBTEXTCAT_DIR)/,$(notdir %.c))
	$(CC) $(FLAGS) $< -o $(OUT_DIR)/$@

clean:
	rm -f $(wildcard $(OUT_DIR)/*.o)

