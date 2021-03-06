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

void _os_log(const char *str);
void _os_log_info(const char *str);
void _os_log_debug(const char *str);
void _os_log_error(const char *str);
void _os_log_fault(const char *str);

#endif /* oslog_h */
