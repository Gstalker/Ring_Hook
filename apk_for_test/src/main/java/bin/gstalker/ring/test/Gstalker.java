package bin.gstalker.ring.test;

public class Gstalker {
    static {
        System.loadLibrary("gstalker");
    }

    public static native void target_function(String native_bridge);
}
