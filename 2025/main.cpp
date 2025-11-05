#include <iostream>
#include <fstream>
#include <string>
#include "solution/day_one.h"

int main() {
    std::string day = "day_one"; // Change as needed
    std::string input_path = "input/" + day + ".txt";
    std::ifstream file(input_path);
    if (!file.is_open()) {
        std::cerr << "Failed to open input file: " << input_path << std::endl;
        return 1;
    }

    if (day == "day_one") {
        solve_day_one(file);
    } else {
        std::cerr << "Unknown day: " << day << std::endl;
        file.close();
        return 1;
    }

    file.close();
    return 0;
}