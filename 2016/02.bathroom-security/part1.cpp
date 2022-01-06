
#include <bits/stdc++.h>

using namespace std;

int main() {
    int value = 0;
    int x = 1, y = 1;
    string line;
    while (getline(cin, line)) {
        for (char c : line) {
            if (c == 'U' && y > 0) {
                y--;
            } else if (c == 'D' && y < 2) {
                y++;
            } else if (c == 'L' && x > 0) {
                x--;
            } else if (c == 'R' && x < 2) {
                x++;
            }
        }
        value *= 10;
        value += 1 + 3 * y + x;
    }
    cout << "Result: " << value << endl;
}

