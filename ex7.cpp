#include <iostream>
#include <string>
#include <cctype>

void ft_seed_inventory(std::string seed_type, int quantity, std::string unit)
{
    size_t i;

    for (i = 1; i <= seed_type.length(); i++) //.length()/.size() returns a size_t
        seed_type[i] = std::tolower(seed_type[i]);
    seed_type[0] = std::toupper(seed_type[0]);

    if (unit == "packets")
        //std::cout generally auto-converts int to str for the output
        std::cout << seed_type << " seeds: " << quantity << " packets available" << std::endl;
    else if (unit == "grams")
        std::cout << seed_type << " seeds: " << quantity << " grams total" << std::endl;
    else if (unit == "area")
        std::cout << seed_type << " seeds: " << "covers " << quantity << " square meters" << std::endl;
    else
        std::cout << "Unknown unit type" << std::endl;
}