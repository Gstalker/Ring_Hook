#include <jni.h>
#include "classfinder.h"
#include "classloader.h"
#include "logger.h"

JNIEXPORT jclass JNICALL
Java_bin_gstalker_ring_ClassFinder_findClassNative(JNIEnv *env, jclass clazz, jstring name,
                                                   jclass class_loader_type) {
    return findClassFromHeapRefs((C_JNIEnv *)env, name, class_loader_type);
}

JNINativeMethod classfinder_methods[] = {
        [0] = {
                "findClassNative",
                "(Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;",
                &Java_bin_gstalker_ring_ClassFinder_findClassNative
        },
};

__attribute__ ((visibility("default")))
JNINativeMethod* get_classfinder_methods(){
    return classfinder_methods;
}