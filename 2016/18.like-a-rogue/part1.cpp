
#include <bits/stdc++.h>

using namespace std;

int main() {
    size_t count = 0;
    string line;
    cin >> line;
    bool tiles[40][2 + line.size()];
    memset(tiles, 0, sizeof(tiles));
    for (size_t i = 0; i < line.size(); i++) {
        tiles[0][1 + i] = line[i] == '^';
        if (!tiles[0][1 + i]) {
            count ++;
        }
    }
    for (size_t r = 1; r < 40; r++) {
        for (size_t i = 1; i <= line.size(); i++) {
            tiles[r][i] = tiles[r - 1][i - 1] != tiles[r - 1][i + 1];
            if (!tiles[r][i]) {
                count ++;
            }
        }
    }
    cout << "Result: " << count << endl;
}

