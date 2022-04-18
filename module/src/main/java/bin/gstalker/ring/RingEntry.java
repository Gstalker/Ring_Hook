package bin.gstalker.ring;

import bin.gstalker.hook_instance.LogE;

public class RingEntry {
    public static void init() {
        Logger.wtf("Process JavaLayer Hooker");
        registerJavaHookers();
        HookManager.processHookers();
    }

    private static void registerJavaHookers() {
        HookManager.registerHooker(LogE.getInstance());
    }
}
