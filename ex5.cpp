#include <iostream>

void ft_water_reminder()
{
    int days;

    std::cout << "Days since last watering: ";
    std::cin >> days;

    if (days > 2)
        std::cout << "Water the plants!" << std::endl;
    else
        std::cout << "Plants are fine" << std::endl;
}