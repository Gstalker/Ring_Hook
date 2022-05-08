package bin.gstalker.ring;

import android.util.Log;

import dalvik.system.BaseDexClassLoader;

public class ClassFinder {
    private static Boolean ACTIVATE = false;

    public static void setActivate(Boolean value){
        ACTIVATE = value;
    }

    public static Class findClass(String name) throws ClassNotFoundException{
        if(ACTIVATE == false) {
            throw new ClassNotFoundException("ClassFinder Not Activate yet: " + name);
        }
        Log.w("RING_TEST","ClassFinder: find " + name);
        Class result = findClassNative(name, BaseDexClassLoader.class);
        Log.w("RING_TEST","ClassFinder: finish " + name);
        if(result == null){
            throw new ClassNotFoundException("Cannot find target Class: " + name);
        }
        return result;
    }

    public static native Class findClassNative(String name, Class class_loader_type);
}
