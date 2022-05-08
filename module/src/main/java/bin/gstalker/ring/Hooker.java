package bin.gstalker.ring;

import java.lang.reflect.Method;

public interface Hooker {
    Object targetMethod() throws Exception;
    Method hookMethod() throws Exception;
    Method backupMethod() throws Exception;
}
