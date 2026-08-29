#include <iostream>
#include <limits>
#include <string>

int readValidInt(const std::string &prompt)
{
    int value;

    while (true)
    {
        std::cout << prompt;
        std::cin >> value;
        if (!std::cin)
        {
            std::cin.clear();
            std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
            std::cout << "Invalid value. Try again" << std::endl;
            continue;
        }
        break;
    }
    return value;
}

void ft_plot_area()
{
    int width;
    int length;
    int res;

    length = readValidInt("Enter length: ");
    width = readValidInt("Enter width: ");
    res = length * width;

    std::cout << "Plot area: " << res << std::endl;
}