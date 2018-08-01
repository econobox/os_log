//
//  oslog.h
//  liboslog
//
//  Created by Søren Mortensen on 29/07/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//

#ifndef oslog_h
#define oslog_h

#include <os/log.h>

os_log_t _os_log_default_ptr();

void _os_log(os_log_t log, const char *str);
void _os_log_info(os_log_t log, const char *str);
void _os_log_debug(os_log_t log, const char *str);
void _os_log_error(os_log_t log, const char *str);
void _os_log_fault(os_log_t log, const char *str);

#endif /* oslog_h */
