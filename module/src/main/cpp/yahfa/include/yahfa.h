//
// Created by Dreamstalker on 2021/9/7.
//

#ifndef UNPACKER_AADEBUGMODULE_YAHFA_H
#define UNPACKER_AADEBUGMODULE_YAHFA_H

#include "jni.h"

__attribute__ ((visibility("default")))
JNINativeMethod* get_yahfa_hook_main_methods();

__attribute__ ((visibility("default")))
JNINativeMethod* get_yahfa_utils_methods();


#endif //UNPACKER_AADEBUGMODULE_YAHFA_H