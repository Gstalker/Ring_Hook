#pragma once

class Ring{
private:
    static Ring* instance;

private:
    Ring() = default;

public:
    static Ring* GetInstance();
};