#include <iostream>
#include <string>
#include <conio.h>
#include "todoList.h"
#define ever (;;)
#define waitUntilEnter getch();

int main() {

    todoList list;

    for ever
    {
        std::cout << "Todo App\n1-List all entries\n2-Add a entery to the list\n3-Remove a entery from the list\n4-exit program" << std::endl;
        std::string input;
        int option;
        std::cin >> input;
        try
        {
            option = std::stoi(input);
        }
        catch(const std::exception& e)
        {
            std::cout << "Invalid option: " << input << std::endl;
            continue;
        }


        switch (option)
        {
        case 1:
            list.listEnteries();
            break;
        
        case 2:
           list.addEntery();
            break;
        case 3:
            list.removeEntery();
            break;
        case 4:
            return 0;
        default:
        std::cout << "invalid option" << std::endl;
        continue;
        }
        waitUntilEnter
    }
}