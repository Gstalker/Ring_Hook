package bin.gstalker.ring;

import bin.gstalker.hook_instance.LogE;
import bin.gstalker.hook_instance.StartTestHooker;
import bin.gstalker.ring.hooker.ClassLoaderHooker;
import bin.gstalker.ring.hooker.LoadClassHooker;
import bin.gstalker.ring.hooker.LoadedApkGetCLHooker;
import bin.gstalker.ring.hooker.LoadedApkConstructorHooker;

public class RingEntry {
    public static void init() {
        Logger.wtf("Process JavaLayer Hooker");
        CrackAndroidBlackLists.crack();
        registerJavaHookers();
        HookManager.processHookers();
        ClassFinder.setActivate(true);
    }

    private static void registerJavaHookers() {
        HookManager.registerHooker(LogE.getInstance());
//        HookManager.registerHooker(ClassLoaderHooker.getInstance());
        HookManager.registerHooker(StartTestHooker.getInstance());
//        HookManager.registerHooker(LoadClassHooker.getInstance());
        HookManager.registerHooker(LoadedApkConstructorHooker.getInstance());
        HookManager.registerHooker(LoadedApkGetCLHooker.getInstance());

    }
}
