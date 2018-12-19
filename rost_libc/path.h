#ifndef PATH_H
#define PATH_H
#include "std.h"

#define PATH_SEP '/'
typedef char* path_t;

bool path_is_abs(path_t path);

bool path_is_rel(path_t path);

path_t path_base(path_t path);

#endif