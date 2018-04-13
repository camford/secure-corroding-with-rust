#include <stdio.h>
#include <stdint.h>

int main(int argc, char **argv) {
    uint32_t buf[5] = {1, 2, 3, 4, 5};
    printf("buf[%d]: %d\n", 4, buf[4]);
    printf("buf[%d]: %d\n", 9, buf[9]);
}
