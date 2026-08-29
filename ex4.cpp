#include <iostream>

void ft_plant_age()
{
    int val;

    std::cout << "Enter plant age in days: ";
    std::cin >> val;

    if (val > 60)
        std::cout << "Plant is ready to harvest!" << std::endl;
    else
        std::cout << "Plant needs more time to grow." << std::endl;
}