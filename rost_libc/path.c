#include "path.h"

bool path_is_abs(path_t path) {
    return path[0] == PATH_SEP;
}

bool path_is_rel(path_t path) {
    return !path_is_abs(path);
}

path_t path_base(path_t path) {
    int last_sep = -1;

    for(int i = 0; path[i]; i++) {
        if (path[i] == PATH_SEP) {
            last_sep = i;
        }
    }

    return path + last_sep;
}