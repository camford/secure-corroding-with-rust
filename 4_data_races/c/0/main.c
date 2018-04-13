#include <stdio.h>
#include <pthread.h>
#define NTHREADS 100

int var = 0;

void* child_fn ( void* args ) {
    var++;
    return NULL;
}

int race ( void ) {
    pthread_t threads[NTHREADS];
    var = 0;
    for (int i=0; i<NTHREADS; i++) {
        pthread_create(&threads[i], NULL, child_fn, NULL);
    }
    for (int i=0; i<NTHREADS; i++) {
        pthread_join(threads[i], NULL);
    }
    return var;
}

int main ( void ) {
    while (1) {
        int i = race();
        if (i != NTHREADS) {
            printf("DATA RACE!!! (i = %d)\n", var);
        } else {
            printf("Everything is awesome!\n");
        }
    }
    return 0;
}
