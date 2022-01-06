
#include <bits/stdc++.h>

using namespace std;

int main() {
    int keypad[7][7] = {
        {0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 1, 0, 0, 0},
        {0, 0, 2, 3, 4, 0, 0},
        {0, 5, 6, 7, 8, 9, 0},
        {0, 0, 10, 11, 12, 0, 0},
        {0, 0, 0, 13, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0},
    };
    int value = 0;
    int x = 1, y = 3;
    string line;
    while (getline(cin, line)) {
        for (char c : line) {
            if (c == 'U' && keypad[y - 1][x] != 0) {
                y--;
            } else if (c == 'D' && keypad[y + 1][x] != 0) {
                y++;
            } else if (c == 'L' && keypad[y][x - 1] != 0) {
                x--;
            } else if (c == 'R' && keypad[y][x + 1] != 0) {
                x++;
            }
        }
        value *= 16;
        value += keypad[y][x];
    }
    cout << "Result: " << uppercase << hex << value << endl;
}

