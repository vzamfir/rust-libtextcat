/**
 * Function 
 * @param tc libtextcat handle
 * @param text the text which is analyzed
 * @param text_size the size of the text
 * @return c-string containing the language of the text
 */
char *l_get_language(void *tc_handle, const char *text, size_t text_size);

/**
 * The code from the textcat library, slightly adapted, to avoid external config files.
 * @param LIBTEXT_LANGUAGES_PATH path to the libtextcat language files.
 * @return libtextac handle.
 */
void *l_textcat_Init(const char* LIBTEXT_LANGUAGES_PATH);


