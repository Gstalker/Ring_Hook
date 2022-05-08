package bin.gstalker.hook_instance;

import android.util.Log;

import java.lang.reflect.Method;

import bin.gstalker.ring.ClassFinder;
import bin.gstalker.ring.Hooker;

public class StartTestHooker implements Hooker {
    static StartTestHooker instance;

    static {
        instance = new StartTestHooker();
    }

    public static StartTestHooker getInstance() {
        return instance;
    }

    public static void hook(String test_string) throws Throwable {
        Log.wtf("RING_TEST","bin.gstalker.ring.test.Gstalker::start_test captured!");
        Log.wtf("RING_TEST","    test_string: " + test_string);
        backup(test_string);
    }

    public static void backup(String test_string) throws Throwable {
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
    public Object targetMethod() throws Exception {
        Log.wtf("RING_TEST","getting targetMethod");
        return ClassFinder.findClass("bin.gstalker.ring.test.Gstalker")
                .getDeclaredMethod("start_test", String.class);
    }

    @Override
    public Method hookMethod() throws Exception {
        Log.wtf("RING_TEST","getting targetMethod");
        return Class.forName("bin.gstalker.hook_instance.StartTestHooker")
                .getDeclaredMethod("hook", String.class);
    }

    @Override
    public Method backupMethod() throws Exception {
        return Class.forName("bin.gstalker.hook_instance.StartTestHooker")
                .getDeclaredMethod("backup", String.class);
    }
}
