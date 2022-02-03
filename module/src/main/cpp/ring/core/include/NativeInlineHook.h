#pragma once

class NativeInlineHook{
private:
    static NativeInlineHook* instance;

private:
    NativeInlineHook() = default;

public:
    static NativeInlineHook* GetInstance();
};