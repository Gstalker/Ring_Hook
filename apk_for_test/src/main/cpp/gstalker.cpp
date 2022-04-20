#include <jni.h>
#include <string>
#include <dlfcn.h>
#include <thread>
#include <unistd.h>

#include "utils.h"

using std::string;

JNIEXPORT jint JNI_OnLoad(JavaVM* vm, void* reserved){
    LOGW("libgstalker.so has been load");
    return JNI_VERSION_1_6;
}

void dlopen_test() {
    const static char* lib_name = "libadreno_utils.so";
    LOGW("Dlopen test: try to open dylib %s",lib_name);
    auto handle = dlopen(lib_name,RTLD_NOW);
    if( handle ){
        LOGW("    SUCCESS!");
        LOGW("    dlopen result: %p", handle);
        dlclose(handle);
    }
    else{
        LOGW("    FAILED!");
        LOGW("    dlopen return nullptr, dlopen failed! %s", dlerror());
    }
}

void thread_test() {
    std::thread t([](){
        for(int i = 0 ; ; ++i){
            LOGF("Thread_Test: RUNNING...... %d", i);
            sleep(100);
        }
    });
    t.detach();
}

extern "C"
JNIEXPORT void JNICALL
Java_bin_gstalker_ring_test_Gstalker_target_1function(
        JNIEnv *env,
        jobject thiz,
        jstring native_bridge) {
    auto* tmp_str = env->GetStringUTFChars(native_bridge,JNI_FALSE);
    string string_from_java(tmp_str);
    env->ReleaseStringUTFChars(native_bridge,tmp_str);
//    dlopen_test();
//    thread_test();
    LOGW("libgstalker.so get the string from java: %s", string_from_java.c_str());
    LOGW("    self addr: %p", Java_bin_gstalker_ring_test_Gstalker_target_1function);
    LOGW("    test finish");
}