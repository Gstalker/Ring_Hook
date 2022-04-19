#include <jni.h>
#include <string>

#include "utils.h"

using std::string;

JNIEXPORT jint JNI_OnLoad(JavaVM* vm, void* reserved){
    LOGW("libgstalker.so has been load");
    return JNI_VERSION_1_6;
}

extern "C"
JNIEXPORT void JNICALL
Java_bin_gstalker_ring_test_Gstalker_target_1function(
        JNIEnv *env,
        jclass clazz,
        jstring native_bridge) {
    auto* tmp_str = env->GetStringUTFChars(native_bridge,JNI_FALSE);
    string string_from_java(tmp_str);
    env->ReleaseStringUTFChars(native_bridge,tmp_str);
    LOGW("libgstalker.so get the string from java: %s", string_from_java.c_str());
    LOGW("    test finish");
}