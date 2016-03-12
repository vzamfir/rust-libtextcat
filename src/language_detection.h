
#include "../../libtextcat-2.2/src/textcat.h"
char *l_get_language(void *tc_handle, const char *text, size_t text_size);


void *l_textcat_Init(const char* LIBTEXT_LANGUAGES_PATH);    //returns a textcat handle (has to be freed by calling textcat_Done)


