package bin.gstalker.ring;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

public class CrackAndroidBlackLists {
    public static void crack(){
        try{
            Method getDeclearedMethod = Class.class.getDeclaredMethod(
                    "getDeclaredMethod", String.class, Class[].class);

            Method classForName = Class.class.getDeclaredMethod(
                    "forName", String.class);

            Class<?> VMRuntime = (Class<?>) classForName.invoke(null, "dalvik.system.VMRuntime");

            Method getRuntime = (Method) getDeclearedMethod.invoke(VMRuntime, "getRuntime", null);

            Object VMRuntime_THE_ONE = getRuntime.invoke(null);

            Method setHiddenApiExemptions = (Method) getDeclearedMethod.invoke(
                    VMRuntime, "setHiddenApiExemptions", new Class[]{ String[].class });

            assert setHiddenApiExemptions != null;
            setHiddenApiExemptions.invoke(VMRuntime_THE_ONE, new Object[]{new String[]{ "L" }});
        }
        catch (Throwable t){
            Logger.wtf("Cannot crack the blackLIST!" + t);
        }
    }
}
