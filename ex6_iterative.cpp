#include <iostream>

void ft_count_harvest_iterative()
{
    int val;
    int i;

    std::cout << "Days until harvest: ";
    std::cin >> val;

    for (i = 1; i <= val; i++)
        std::cout << "Day " << i << std::endl;
    
    std::cout << "Harvest time!" << std::endl;
}