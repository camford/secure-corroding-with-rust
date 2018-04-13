#include <stdio.h>
#include <string.h>

int main(void) {
    char* ptr = NULL;

    /* Stuff happens here. We expect ptr to be valid now. */
    /* e.g. ptr = malloc(BUF_SIZE)*/

    if (ptr != NULL) {
        *ptr = 'c';
    }
    printf("We'll get here\n");
}
