#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

int main(int argc, char** argv) {
    uint32_t buf[5] = {1, 2, 3, 4, 5};
    int i;

    printf("buf[%d]: %d\n", 4, buf[4]);
    if (argc == 2) {
        i = atoi(argv[1]);
        if (i != 0) {
            printf("buf[%d]: %d\n", i, buf[i]);
        }
    }
}
