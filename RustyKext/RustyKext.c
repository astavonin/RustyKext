//
//  RustyKext.c
//  RustyKext
//
//  Created by Alexander Stavonin on 16/12/13.
//  Copyright (c) 2013 Alexander Stavonin. All rights reserved.
//

#include <mach/mach_types.h>

kern_return_t RustyKext_start(kmod_info_t * ki, void *d);
kern_return_t RustyKext_stop(kmod_info_t *ki, void *d);

kern_return_t RustyKext_start(kmod_info_t * ki, void *d)
{
    return KERN_SUCCESS;
}

kern_return_t RustyKext_stop(kmod_info_t *ki, void *d)
{
    return KERN_SUCCESS;
}
