package bin.gstalker.ring;

import java.lang.reflect.Method;

public interface Hooker {
    Object targetMethod(ClassLoader loader) throws Exception;
    Method hookMethod() throws Exception;
    Method backupMethod() throws Exception;
}
