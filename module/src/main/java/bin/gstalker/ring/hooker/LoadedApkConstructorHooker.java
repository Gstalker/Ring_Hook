package bin.gstalker.ring.hooker;

import java.lang.reflect.Method;

import android.app.ActivityThread;
import android.app.LoadedApk;
import android.content.pm.ApplicationInfo;
import android.content.res.CompatibilityInfo;
import android.util.Log;

import bin.gstalker.ring.Hooker;
import bin.gstalker.ring.Logger;

public class LoadedApkConstructorHooker implements Hooker {

    static LoadedApkConstructorHooker instance = new LoadedApkConstructorHooker();

    public static LoadedApkConstructorHooker getInstance() {
        return instance;
    }

    public static Object hook(Object thiz,ActivityThread at, ApplicationInfo ai, CompatibilityInfo ci,
                            ClassLoader cl, boolean a, boolean b, boolean c) throws Throwable{
        Logger.w("LoadedApk's Constructor Hit!");
        LoadedApkGetCLHooker.setActivate(true);
        return backup(thiz,at,ai,ci,cl,a,b,c);
    }

    public static Object backup(Object thiz, ActivityThread at, ApplicationInfo ai, CompatibilityInfo ci,
                                ClassLoader cl, boolean a, boolean b, boolean c) throws Throwable{
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
        return LoadedApk.class.getDeclaredConstructor(
                ActivityThread.class, ApplicationInfo.class, CompatibilityInfo.class,
                ClassLoader.class, boolean.class, boolean.class, boolean.class);
    }

    @Override
    public Method hookMethod() throws Exception {
        return LoadedApkConstructorHooker.class.getDeclaredMethod("hook", Object.class, ActivityThread.class, ApplicationInfo.class, CompatibilityInfo.class, ClassLoader.class, boolean.class, boolean.class, boolean.class);
    }

    @Override
    public Method backupMethod() throws Exception {
        return LoadedApkConstructorHooker.class.getDeclaredMethod("backup", Object.class, ActivityThread.class, ApplicationInfo.class, CompatibilityInfo.class, ClassLoader.class, boolean.class, boolean.class, boolean.class);
    }
}
