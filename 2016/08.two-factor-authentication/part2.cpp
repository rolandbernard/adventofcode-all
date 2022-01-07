
#include <bits/stdc++.h>

using namespace std;

int main() {
    bool display[6][50];
    memset(display, 0, sizeof(display));
    string line;
    while (cin >> line) {
        if (line == "rect") {
            cin >> line;
            int x = line.find('x');
            int width = stoi(line.substr(0, x));
            int height = stoi(line.substr(x + 1));
            for (int i = 0; i < height; i++) {
                for (int j = 0; j < width; j++) {
                    display[i][j] = true;
                }
            }
        } else if (line == "rotate") {
            cin >> line;
            bool row = line == "row" ? true : false;
            cin >> line;
            int equal = line.find('=');
            int pos = stoi(line.substr(equal + 1));
            cin >> line >> line;
            int by = stoi(line);
            for (int k = 0; k < by; k++) {
                if (row) {
                    int tmp = display[pos][49];
                    for (int i = 49; i > 0; i--) {
                        display[pos][i] = display[pos][i - 1];
                    }
                    display[pos][0] = tmp;
                } else {
                    int tmp = display[5][pos];
                    for (int i = 5; i > 0; i--) {
                        display[i][pos] = display[i - 1][pos];
                    }
                    display[0][pos] = tmp;
                }
            }
        }
    }
    cout << "Result: " << endl;
    for (int i = 0; i < 6; i++) {
        for (int j = 0; j < 50; j++) {
            cout << (display[i][j] ? "█" : " ");
        }
        cout << endl;
    }
}

