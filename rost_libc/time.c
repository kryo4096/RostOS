#include "time.h"
#include "syscall.h"
time_t time_now() {
    return get_ticks();
}