#include <iostream>
#include "todoList.h"

void todoList::listEnteries() {
    for (std::size_t i = 0; i!= entries.size(); ++i) {
        std::cout << i << "." << entries[i] << "\n";
    }
}

void todoList::addEntery() {
    std::string entry;
    std::cout << "What is the name of the task(press enter to cancel)" << std::endl;
    std::getline(std::cin, entry);
    entries.push_back(entry);
}

void todoList::removeEntery(){
    std::string input;
    std::cout << "What is the number of the task you want to remove" << std::endl;
    std::getline(std::cin, input);
    int entry;

    try
    {
        entry = std::stoi(input);
    }
    catch(const std::exception& e)
    {
        std::cout << "You need to input a number, you inputed: " << input << std::endl;
        return;
    }

    std::string choise;
    std::cout << "Do you want to remove \"" << entries[entry] << "\"?(Y/n)" ;
    if(!(choise == "Y" || choise == "y")) {
        return;
    }
    entries.erase(entries.begin()+entry);
}