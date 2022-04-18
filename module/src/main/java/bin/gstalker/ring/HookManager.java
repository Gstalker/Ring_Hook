package bin.gstalker.ring;

import java.util.Vector;
import lab.galaxy.yahfa.HookMain;

public class HookManager {
    static HookManager instance = new HookManager();

    private Vector<Hooker> hookers;

    private HookManager() {
        hookers = new Vector<>();
    }

    private void processHookersInner() {
        for (Hooker hooker: hookers) {
            try{
                Logger.i("Process hooker: " + hooker.getClass().getSimpleName());
                HookMain.backupAndHook(
                        hooker.targetMethod(),
                        hooker.hookMethod(),
                        hooker.backupMethod()
                );
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
        instance.processHookersInner();
    }

    public static void registerHooker(Hooker hook) {
        Logger.i("register hooker: " + hook.getClass().getSimpleName());
        instance.hookers.add(hook);
    }
}
