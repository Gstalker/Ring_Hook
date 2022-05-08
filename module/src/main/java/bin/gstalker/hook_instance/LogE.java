package bin.gstalker.hook_instance;

import android.util.Log;

import java.lang.reflect.Method;

import bin.gstalker.ring.Hooker;

public class LogE implements Hooker{
    static LogE instance = new LogE();

    public static Hooker getInstance(){
        return instance;
    }

    @Override
    public Object targetMethod() throws Exception{
        return Class.forName("android.util.Log").getDeclaredMethod(
                "e", String.class, String.class);
    }

    @Override
    public Method hookMethod() throws Exception {
        return Class.forName("bin.gstalker.hook_instance.LogE").getDeclaredMethod(
                "hook", String.class, String.class
        );
    }

    @Override
    public Method backupMethod() throws Exception {
        return Class.forName("bin.gstalker.hook_instance.LogE").getDeclaredMethod(
                "backup", String.class, String.class
        );
    }

    public static int hook(String tag, String inner) throws Throwable {
        Log.wtf("RING_TEST", "android.utils.Log.e() captured!");
        Log.wtf("RING_TEST", "    Tag: " + tag);
        Log.wtf("RING_TEST", "    inner: " + inner);
        return backup(tag,inner);
    }

    public static int backup(String tag, String inner) throws Throwable {
        try{
            Log.wtf("RING_TEST","Fatal: this is a PlaceHolder for JavaLayerHooker");
            System.exit(1919810);
        }
        catch(Exception e){
            Log.wtf("RING_TEST",e.getLocalizedMessage());
        }
        throw new ClassNotFoundException("Should not be here!");
    }
}
