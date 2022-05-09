package bin.gstalker.ring.test;

import android.util.Log;

public class Gstalker {
    static {
        instance = new Gstalker();
        System.loadLibrary("gstalker");
    }

    private static Gstalker instance;

    public static Gstalker getInstance() {
        return instance;
    }

    public static void start_test(String test_string) throws Throwable{
        Log.wtf("GSTALKER","Hello world from Gstalker::start_test");
        instance.target_function(test_string);
        Log.wtf("GSTALKER", "please do not jit me!");
        throw new RuntimeException("Fuck Opt");
    }

    public native void target_function(String native_bridge);
}
