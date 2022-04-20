package bin.gstalker.ring.test;

public class Gstalker {
    static {
        instance = new Gstalker();
        System.loadLibrary("gstalker");
    }

    private static Gstalker instance;

    public static Gstalker getInstance() {
        return instance;
    }

    public static void start_test(String test_string){
        instance.target_function(test_string);
    }

    public native void target_function(String native_bridge);
}
