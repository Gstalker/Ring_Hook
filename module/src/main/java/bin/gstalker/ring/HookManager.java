package bin.gstalker.ring;

import java.util.Vector;
import lab.galaxy.yahfa.HookMain;

public class HookManager {
    static HookManager instance = new HookManager();

    private Vector<Hooker> hookers;
    private Vector<Boolean> hookers_enable_status;

    private HookManager() {
        hookers = new Vector<>();
        hookers_enable_status = new Vector<>();
    }

    private void processHookersInner(ClassLoader loader) {
        ClassLoader providedLoader;
        if(loader == null){
            providedLoader = getHookerClassLoader();
        }
        else{
            providedLoader = loader;
        }
        for (int i = 0; i < hookers.size(); ++i) {
            Hooker hooker = hookers.get(i);
            if(hookers_enable_status.get(i)){
                continue;
            }
            try{
                Logger.i("Process hooker: " + hooker.getClass().getSimpleName());
                HookMain.backupAndHook(
                        hooker.targetMethod(loader),
                        hooker.hookMethod(),
                        hooker.backupMethod()
                );
                hookers_enable_status.set(i, true);
            }
            catch(Exception e) {
                Logger.wtf("Cannot process hooker: " + hooker.getClass().getSimpleName());
                Logger.wtf("    Cause: " + e);
            }
        }
    }

//    public static HookManager getInstance() {
//        return instance;
//    }

    public static void processHookers() {
        instance.processHookersInner(null);
    }

    public static void processRemainHookers(ClassLoader loader){
        instance.processHookersInner(loader);
    }

    public static void registerHooker(Hooker hook) {
        Logger.i("register hooker: " + hook.getClass().getSimpleName());
        instance.hookers.add(hook);
        instance.hookers_enable_status.add(false);
    }

    public static native ClassLoader getHookerClassLoader();
}
