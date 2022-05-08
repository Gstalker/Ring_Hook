package bin.gstalker.ring;

import bin.gstalker.hook_instance.LogE;
import bin.gstalker.hook_instance.StartTestHooker;
import bin.gstalker.ring.hooker.ClassLoaderHooker;

public class RingEntry {
    public static void init() {
        Logger.wtf("Process JavaLayer Hooker");
        registerJavaHookers();
        HookManager.processHookers();
        ClassFinder.setActivate(true);
    }

    private static void registerJavaHookers() {
        HookManager.registerHooker(LogE.getInstance());
        HookManager.registerHooker(ClassLoaderHooker.getInstance());
        HookManager.registerHooker(StartTestHooker.getInstance());
    }
}
