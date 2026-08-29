#include <iostream>

void ft_harvest_total()
{
    int total;
    int i;
    int val;

    total = 0;
    for (i = 1; i <= 3; i++)
    {
        std::cout << "Day " << i << " harvest: ";
        std::cin >> val;
        total += val;
    }

    std::cout << "Total harvest: " << total << std::endl;
}