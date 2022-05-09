package bin.gstalker.ring.hooker;

import android.util.Log;
import java.lang.reflect.Method;

import bin.gstalker.ring.Hooker;
import bin.gstalker.ring.Logger;

public class ClassLoaderHooker implements Hooker {

    static ClassLoaderHooker instance = new ClassLoaderHooker();

    public static Hooker getInstance(){
        return instance;
    }

    @Override
    public Object targetMethod(ClassLoader ignored) throws Exception {
        return Class.forName("java.lang.ClassLoader").getDeclaredConstructor(Class.forName("java.lang.ClassLoader"));
    }

    @Override
    public Method hookMethod() throws Exception {
        return Class.forName("bin.gstalker.ring.hooker.ClassLoaderHooker").getDeclaredMethod(
                "hook", Object.class, ClassLoader.class);
    }

    @Override
    public Method backupMethod() throws Exception {
        return Class.forName("bin.gstalker.ring.hooker.ClassLoaderHooker").getDeclaredMethod(
                "backup", Object.class, ClassLoader.class);
    }

    public static Object hook(Object thiz, ClassLoader parent) throws Throwable {
        Logger.w("ClassLoader(parent) captured!");
        Logger.w("    PrePare for Processing Hookers!");
//        setDefineClassNativeHookerReady();
        LoadClassHooker.setActivate(true);
        return backup(thiz, parent);
    }

    public static Object backup(Object thiz, ClassLoader parent) throws Throwable {
        try{
            Log.wtf("RING_TEST","Fatal: this is a PlaceHolder for JavaLayerHooker");
            System.exit(1919810);
        }
        catch(Exception e){
            Log.wtf("RING_TEST",e.getLocalizedMessage());
        }
        throw new ClassNotFoundException("Should not be here!");
    }

//    private static native void setDefineClassNativeHookerReady();
}
