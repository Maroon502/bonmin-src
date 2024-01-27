include Bonmin/src.mk

bonmin_lib_sources.txt: Bonmin/src.mk
	@echo -n "${LIB_SOURCES}" | tr ' ' '\n' > bonmin_lib_sources.txt

gen_lib_sources: bonmin_lib_sources.txt