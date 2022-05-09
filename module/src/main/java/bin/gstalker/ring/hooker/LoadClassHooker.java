package bin.gstalker.ring.hooker;

import android.util.Log;

import java.lang.reflect.Method;

import bin.gstalker.ring.HookManager;
import bin.gstalker.ring.Hooker;
import bin.gstalker.ring.Logger;

public class LoadClassHooker implements Hooker {
    static LoadClassHooker instance = new LoadClassHooker();

    static boolean ACTIVATE = false;

    public static LoadClassHooker getInstance() {
        return instance;
    }

    public static void setActivate(boolean status) {
        ACTIVATE = status;
    }

    public static Class<?> hook(Object thiz, String name) throws Throwable{
        Logger.w("ClassLoader.loacClass hit! target ClassName: " + name);
//        if(ACTIVATE){
//            ACTIVATE = false;
            Logger.w("process Remaining hookers!");
            HookManager.processHookers();
//        }
        return backup(thiz,name);
    }

    public static Class<?> backup(Object thiz, String name) throws Throwable{
        try{
            Log.wtf("RING_TEST","Fatal: this is a PlaceHolder for JavaLayerHooker");
            System.exit(1919810);
        }
        catch(Exception e){
            Log.wtf("RING_TEST",e.getLocalizedMessage());
        }
        throw new ClassNotFoundException("Should not be here!");
    }
    @Override
    public Object targetMethod(ClassLoader ignored) throws Exception {
        return ClassLoader.class.getDeclaredMethod("loadClass", String.class);
    }

    @Override
    public Method hookMethod() throws Exception {
        return LoadClassHooker.class.getDeclaredMethod("hook", Object.class, String.class);
    }

    @Override
    public Method backupMethod() throws Exception {
        return LoadClassHooker.class.getDeclaredMethod("backup", Object.class, String.class);
    }
}