#pragma once

#include <jni.h>

#ifdef __cplusplus
extern "C"
#endif
__attribute__ ((visibility ("default")))
jclass findClassFromHeapRefs(C_JNIEnv* env, jstring name, jclass class_loader_type);