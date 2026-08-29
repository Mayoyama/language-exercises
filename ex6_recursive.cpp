#include <iostream>

void helper(int count, int max)
{
    if (count <= max)
    {
        std::cout << "Day " << count << std::endl;
        helper(count + 1, max);
    }
}

void ft_count_harvest_recursive()
{
    int val;

    std::cout << "Days until harvest: ";
    std::cin >> val;
    helper(1, val);
    std::cout << "Harvest time!" << std::endl;
}