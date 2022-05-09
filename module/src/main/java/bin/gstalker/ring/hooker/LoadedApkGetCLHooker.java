package bin.gstalker.ring.hooker;

import android.app.LoadedApk;
import android.util.Log;

import java.lang.reflect.Method;

import bin.gstalker.ring.HookManager;
import bin.gstalker.ring.Hooker;
import bin.gstalker.ring.Logger;

public class LoadedApkGetCLHooker implements Hooker {

    static LoadedApkGetCLHooker instance = new LoadedApkGetCLHooker();

    static boolean ACTIVATE = false;

    public static LoadedApkGetCLHooker getInstance() {
        return instance;
    }

    public static void setActivate(boolean status) {
        ACTIVATE = status;
    }

    public static ClassLoader hook(Object thiz) throws Throwable{
        LoadedApk apk_backup = (LoadedApk) thiz;
        Logger.w("LoadedApk.getClassLoader Hit! package name:" + apk_backup.getPackageName());

        ClassLoader result = backup(thiz);

        if(result == null){
            Logger.w("LoadedApk.getCL: result == null, bootClassLoader!");
            return result;
        }

        if (ACTIVATE) {
            ACTIVATE = false;
            Logger.w("process remaining java hookers");
            HookManager.processRemainHookers(result);
        }
        return result;
    }

    public static ClassLoader backup(Object thiz) throws Throwable {
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
        return LoadedApk.class.getDeclaredMethod("getClassLoader");
    }

    @Override
    public Method hookMethod() throws Exception {
        return LoadedApkGetCLHooker.class.getDeclaredMethod("hook", Object.class);
    }

    @Override
    public Method backupMethod() throws Exception {
        return LoadedApkGetCLHooker.class.getDeclaredMethod("backup", Object.class);
    }
}
