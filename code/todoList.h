#include <string>
#include <deque>
#pragma once

class todoList {
 private:
    std::deque<std::string> entries;
public:
    void listEnteries();
    void addEntery();
    void removeEntery();
};