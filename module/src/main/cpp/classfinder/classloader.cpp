#include "classloader.h"
#include "plt.h"
#include "art.h"
#include "obfs-string.h"
#include "logger.h"
#include "hash.h"

static jclass CLASS_RESULT = nullptr;

static jclass findTargetClassByName(C_JNIEnv *env, jobject classLoader, jstring targetClassName){
    jclass targetClass = nullptr;
    jclass classOfClassLoader;
    jmethodID findClass;

    LOGV("Get Class of This classloader");
    classOfClassLoader = (*env)->GetObjectClass((JNIEnv *) env, classLoader);
    if ((*env)->ExceptionCheck((JNIEnv *) env)) {
        (*env)->ExceptionClear((JNIEnv *) env);
        goto done;
    }

    LOGV("Get loadClass of this classloader");
    findClass = (*env)->GetMethodID((JNIEnv *) env, classOfClassLoader, "loadClass", "(Ljava/lang/String;)Ljava/lang/Class;");
    if ((*env)->ExceptionCheck((JNIEnv *) env)) {
        (*env)->ExceptionClear((JNIEnv *) env);
        goto cleanClassOfClassLoader;
    }

    LOGV("Find target class");
    targetClass = (jclass) (*env)->CallObjectMethod((JNIEnv *) env, classLoader, findClass, targetClassName);

    if ((*env)->ExceptionCheck((JNIEnv *) env)) {
        (*env)->ExceptionClear((JNIEnv *) env);
    }

    cleanClassOfClassLoader:
    (*env)->DeleteLocalRef((JNIEnv *) env, classOfClassLoader);
    done:
    LOGV("result: %p", targetClass);

    return targetClass;
}

void doFindClass(C_JNIEnv *env, jobject class_loader_ref, jstring targetClassName, intptr_t hash){
    if (CLASS_RESULT != nullptr ){
        return;
    }

    if(!add(hash)){
        return;
    }

    jclass result = findTargetClassByName(env, class_loader_ref, targetClassName);

    if( result != nullptr ){
        CLASS_RESULT = result;
    }
}

static jobject newLocalRef(JNIEnv *env, void *object) {
    static jobject (*NewLocalRef)(JNIEnv *, void *) = nullptr;
    if (object == nullptr) {
        return nullptr;
    }
    if (NewLocalRef == nullptr) {
        NewLocalRef = (jobject (*)(JNIEnv *, void *)) plt_dlsym("_ZN3art9JNIEnvExt11NewLocalRefEPNS_6mirror6ObjectE"_iobfs.c_str(), nullptr);;
    }
    if (NewLocalRef != nullptr) {
        return NewLocalRef(env, object);
    } else {
        return nullptr;
    }
}

static void deleteLocalRef(JNIEnv *env, jobject object) {
    static void (*DeleteLocalRef)(JNIEnv *, jobject) = nullptr;
    if (DeleteLocalRef == nullptr) {
        DeleteLocalRef = (void (*)(JNIEnv *, jobject)) plt_dlsym("_ZN3art9JNIEnvExt14DeleteLocalRefEP8_jobject"_iobfs.c_str(), nullptr);
    }
    if (DeleteLocalRef != nullptr) {
        DeleteLocalRef(env, object);
    }
}

class ClassLoaderVisitor : public art::SingleRootVisitor {
public:
    ClassLoaderVisitor(C_JNIEnv *env, jclass classLoader, jstring name) :
    env_(env), classLoader_(classLoader), targetClassName_(name) {}

    void VisitRoot(art::mirror::Object *root, const art::RootInfo &info ATTRIBUTE_UNUSED) final {
        jobject object = newLocalRef((JNIEnv *) env_, (jobject) root);
        if (object != nullptr) {
            if ((*env_)->IsInstanceOf((JNIEnv *) env_, object, classLoader_)) {
                LOGV("Visit Global Ref classloader %p", root);
                doFindClass(env_, object, targetClassName_, reinterpret_cast<intptr_t>(root));
            }
            deleteLocalRef((JNIEnv *) env_, object);
        }
    }

private:
    C_JNIEnv *env_;
    jclass classLoader_;
    jstring targetClassName_;
};

static void checkGlobalRef(C_JNIEnv *env, jclass clazz, jstring name) {
    auto VisitRoots = (void (*)(void *, void *)) plt_dlsym("_ZN3art9JavaVMExt10VisitRootsEPNS_11RootVisitorE"_iobfs.c_str(), nullptr);
#ifdef DEBUG
    LOGI("VisitRoots: %p", VisitRoots);
#endif
    if (VisitRoots == nullptr) {
        return;
    }
    JavaVM *jvm;
    (*env)->GetJavaVM((JNIEnv *) env, &jvm);
    ClassLoaderVisitor visitor(env, clazz, name);
    VisitRoots(jvm, &visitor);
}

class WeakClassLoaderVisitor : public art::IsMarkedVisitor {
public :
    WeakClassLoaderVisitor(C_JNIEnv *env, jclass classLoader, jstring name) :
    env_(env), classLoader_(classLoader), targetClassName_(name) {}

    art::mirror::Object *IsMarked(art::mirror::Object *obj) override {
        jobject object = newLocalRef((JNIEnv *) env_, (jobject) obj);
        if (object != nullptr) {
            if ((*env_)->IsInstanceOf((JNIEnv *) env_, object, classLoader_)) {
                LOGV("Visit Local Ref classloader %p", obj);
                doFindClass(env_, object, targetClassName_, reinterpret_cast<intptr_t>(obj));
            }
            deleteLocalRef((JNIEnv *) env_, object);
        }
        return obj;
    }

private:
    C_JNIEnv *env_;
    jclass classLoader_;
    jstring targetClassName_;
};

void findClassFromGlobalRefs(C_JNIEnv* env, jstring name, jclass class_loader_type){
    LOGV("Getting Symbol: VisitRoots");
    auto VisitRoots = (void (*)(void *, void *)) plt_dlsym("_ZN3art9JavaVMExt10VisitRootsEPNS_11RootVisitorE"_iobfs.c_str(), nullptr);
    if(VisitRoots == nullptr) {
        LOGF("Cannot Get Symbol VisitRoots! Visit global refs FAIL!");
        return;
    }
    JavaVM *jvm;
    (*env)->GetJavaVM((JNIEnv *) env, &jvm);
    ClassLoaderVisitor visitor(env, class_loader_type, name);
    VisitRoots(jvm, &visitor);
}

void findClassFromLocalRefs(C_JNIEnv* env, jstring name, jclass class_loader_type){
    LOGV("Getting Symbol: MarkedVisitor");
    auto SweepJniWeakGlobals = (void (*)(void *, void *)) plt_dlsym("_ZN3art9JavaVMExt19SweepJniWeakGlobalsEPNS_15IsMarkedVisitorE"_iobfs.c_str(), nullptr);
    if (SweepJniWeakGlobals == nullptr) {
        LOGF("Cannot Get Symbol MarkedVisitor! Visit Local refs FAIL!");
        return;
    }
    JavaVM *jvm;
    (*env)->GetJavaVM((JNIEnv *) env, &jvm);
    WeakClassLoaderVisitor visitor(env, class_loader_type, name);
    SweepJniWeakGlobals(jvm, &visitor);
}

__attribute__ ((visibility ("default")))
extern "C"
jclass findClassFromHeapRefs(C_JNIEnv* env, jstring name, jclass class_loader_type){
    LOGV("Processing find Class");
    CLASS_RESULT = nullptr;
    findClassFromGlobalRefs(env,name,class_loader_type);
    findClassFromLocalRefs(env,name,class_loader_type);
    clear();
    LOGV("FINISHED");
    return CLASS_RESULT;
}

