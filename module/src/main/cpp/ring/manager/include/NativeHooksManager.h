#pragma once

class NativeHooksManager{
private:
    static NativeHooksManager* instance;

private:
    NativeHooksManager() = default;
public:

    static NativeHooksManager* GetInstance();
};