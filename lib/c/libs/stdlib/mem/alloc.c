/*
 * this file handles the following functions 
 * aswell as just general allocation functionality
 * calloc 
 * free
 * free_sized
 * free_aligned_sized
 * malloc
 * realloc
 */

#include <stdlib.h>
#include <stddef.h>

#define ALIGNMENT 16u
#define INUSE 1u

extern char _heap_end[];
extern char _heap_start[];

typedef struct block block_t;
struct block {
    size_t size;
    block_t *next;
};

static block_t *free_list;
static int ready;

static size_t capacity(block_t *b) {
    return b->size & ~INUSE;
}

static size_t align_up(size_t n) {
    if (n > (size_t)-1 - (ALIGNMENT - 1u)) return 0;
    return (n + (ALIGNMENT - 1u)) & ~(ALIGNMENT - 1u);
}

static void init_heap(void) {
    size_t lo = (size_t)_heap_end;
    size_t hi = (size_t)_heap_start;
    free_list = NULL;
    if (hi > lo + sizeof(block_t)) {
        block_t *first = (block_t *)lo;
        first->size = hi - lo - sizeof(block_t);
        first->next = NULL;
        free_list = first;
    }
    ready = 1;
}

void free(void *ptr) {
    block_t *b, *prev, *cur;
    if (!ptr) return;
    if (!ready) init_heap();
    b = (block_t *)((char *)ptr - sizeof(block_t));
    b->size &= ~INUSE;
    prev = NULL;
    cur = free_list;
    while (cur && cur < b) {
        prev = cur;
        cur = cur->next;
    }
    b->next = cur;
    if (prev) prev->next = b; else free_list = b;
    if (prev &&
        (char *)prev + sizeof(block_t) + capacity(prev) == (char *)b) {
        prev->size += sizeof(block_t) + capacity(b);
        prev->next = b->next;
        b = prev;
    }
    if (b->next &&
        (char *)b + sizeof(block_t) + capacity(b) == (char *)b->next) {
        b->size += sizeof(block_t) + capacity(b->next);
        b->next = b->next->next;
    }
}

void *malloc(size_t size) {
    block_t *prev, *b;
    size_t need, cap, rest;
    if (!ready) init_heap();
    need = align_up(size);
    if (need == 0) return NULL;
    prev = NULL;
    for (b = free_list; b; prev = b, b = b->next) {
        if (capacity(b) >= need) break;
    }
    if (!b) return NULL;
    cap = capacity(b);
    rest = cap - need;
    if (rest >= sizeof(block_t) + ALIGNMENT) {
        block_t *alloc = (block_t *)((char *)b + cap - need);
        block_t *rem = b;
        alloc->size = need | INUSE;
        alloc->next = NULL;
        rem->size = rest - sizeof(block_t);
        rem->next = b->next;
        if (prev) prev->next = rem; else free_list = rem;
        return (void *)(alloc + 1);
    }
    b->size = cap | INUSE;
    if (prev) prev->next = b->next; else free_list = b->next;
    return (void *)(b + 1);
}

void *realloc(void *ptr, size_t size) {
    block_t *b;
    size_t old, need, i;
    unsigned char *src, *dst;
    void *out;
    if (!ptr) return malloc(size);
    if (size == 0) {
        free(ptr);
        return NULL;
    }
    if (!ready) init_heap();
    b = (block_t *)((char *)ptr - sizeof(block_t));
    old = capacity(b);
    need = align_up(size);
    if (need == 0) return NULL;
    if (need == old) return ptr;
    out = malloc(size);
    if (!out) return NULL;
    src = (unsigned char *)ptr;
    dst = (unsigned char *)out;
    i = old < need ? old : need;
    while (i--) *dst++ = *src++;
    free(ptr);
    return out;
}

void *calloc(size_t nmemb, size_t size) {
    size_t total;
    unsigned char *p, *q;
    if (size && nmemb > (size_t)-1 / size) return NULL;
    total = nmemb * size;
    q = malloc(total);
    if (!q) return NULL;
    p = q;
    while (total--) *p++ = 0;
    return q;
}

void free_sized(void *ptr, size_t size) {
    (void)size;
    free(ptr);
}

void free_aligned_sized(void *ptr, size_t alignment, size_t size) {
    (void)alignment;
    (void)size;
    free(ptr);
}