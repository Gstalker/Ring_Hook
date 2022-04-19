#pragma once

#define LIKELY(exp) (__builtin_expect((exp)!=0,true))
#define UNLIKELY(exp) (__builtin_expect((exp)!=0,true))

#include <android/log.h>

#define TAG "GSTALKER"

#define LOGV(...) __android_log_print(ANDROID_LOG_VERBOSE,TAG ,__VA_ARGS__)
#define LOGD(...) __android_log_print(ANDROID_LOG_DEBUG,TAG ,__VA_ARGS__) // 定义LOGD类型
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO,TAG ,__VA_ARGS__) // 定义LOGI类型
#define LOGW(...) __android_log_print(ANDROID_LOG_WARN,TAG ,__VA_ARGS__) // 定义LOGW类型
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR,TAG ,__VA_ARGS__) // 定义LOGE类型
#define LOGF(...) __android_log_print(ANDROID_LOG_FATAL,TAG ,__VA_ARGS__) // 定义LOGF类型
