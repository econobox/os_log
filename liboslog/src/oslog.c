//
//  oslog.c
//  liboslog
//
//  Created by Søren Mortensen on 29/07/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//

#include "oslog.h"

os_log_t _os_log_default_ptr() {
    return OS_LOG_DEFAULT;
}

void _os_log(os_log_t log, const char *str) {
    os_log(log, "%{public}s", str);
}

void _os_log_info(os_log_t log, const char *str) {
    os_log_info(log, "%{public}s", str);
}

void _os_log_debug(os_log_t log, const char *str) {
    os_log_debug(log, "%{public}s", str);
}

void _os_log_error(os_log_t log, const char *str) {
    os_log_error(log, "%{public}s", str);
}

void _os_log_fault(os_log_t log, const char *str) {
    os_log_fault(log, "%{public}s", str);
}
