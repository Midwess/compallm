#include <stddef.h>
#include <stdint.h>

static _Thread_local unsigned stage;
static _Thread_local unsigned suppressed;
static _Thread_local uint64_t counts[2][3];

void codec_copy_stage(unsigned value) { stage = value; }
void codec_copy_push(void) { suppressed++; }
void codec_copy_pop(void) { suppressed--; }
void codec_copy_reset(void) {
    stage = 0;
    suppressed = 0;
    for (unsigned i = 0; i < 2; i++) {
        for (unsigned j = 0; j < 3; j++) counts[i][j] = 0;
    }
}
uint64_t codec_copy_count(unsigned i, unsigned j) { return counts[i][j]; }

static void record(unsigned kind, size_t length) {
    if (stage && !suppressed) counts[stage - 1][kind] += length;
}

/* Volatile accesses keep these implementations from becoming recursive intrinsics. */
void *memcpy(void *destination, const void *source, size_t length) {
    record(0, length);
    volatile unsigned char *out = destination;
    const volatile unsigned char *in = source;
    for (size_t i = 0; i < length; i++) out[i] = in[i];
    return destination;
}

void *memmove(void *destination, const void *source, size_t length) {
    record(1, length);
    volatile unsigned char *out = destination;
    const volatile unsigned char *in = source;
    if ((uintptr_t)destination <= (uintptr_t)source) {
        for (size_t i = 0; i < length; i++) out[i] = in[i];
    } else {
        for (size_t i = length; i > 0; i--) out[i - 1] = in[i - 1];
    }
    return destination;
}

void *memset(void *destination, int byte, size_t length) {
    record(2, length);
    volatile unsigned char *out = destination;
    for (size_t i = 0; i < length; i++) out[i] = (unsigned char)byte;
    return destination;
}

void *__wrap_memcpy(void *destination, const void *source, size_t length) {
    return memcpy(destination, source, length);
}
void *__wrap_memmove(void *destination, const void *source, size_t length) {
    return memmove(destination, source, length);
}
void *__wrap_memset(void *destination, int byte, size_t length) {
    return memset(destination, byte, length);
}
