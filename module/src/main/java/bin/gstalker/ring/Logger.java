package bin.gstalker.ring;

import android.util.Log;

import static bin.gstalker.ring.Utils.LOG_TAG;

public class Logger {
    public static void wtf(String info) {
        Log.wtf(LOG_TAG,info);
    }
    public static void w(String info) {
        Log.w(LOG_TAG,info);
    }
    public static void i(String info) {
        Log.i(LOG_TAG,info);
    }
    public static void v(String info) {
        Log.v(LOG_TAG,info);
    }
    public static void d(String info) {
        Log.d(LOG_TAG,info);
    }
    public static void e(String info) {
        Log.e(LOG_TAG,info);
    }
}
