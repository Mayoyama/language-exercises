#include <iostream>
#include <string>

void ft_garden_name()
{
    std::string var;
    std::cout << "Enter garden name: ";
    std::getline(std::cin, var);
    std::cout << "Garden: " << var << std::endl;

    std::cout << "Status: Growing well!" << std::endl;
}

int main()
{
    ft_garden_name();
    return 0;
}