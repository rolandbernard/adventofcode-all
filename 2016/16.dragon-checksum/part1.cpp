
#include <bits/stdc++.h>

using namespace std;

int main() {
    string line;
    cin >> line;
    while (line.size() < 272) {
        string b = line;
        for (char& c : b) {
            c = (c == '0' ? '1' : '0');
        }
        for (size_t i = 0; i < b.size() / 2; i++) {
            swap(b[i], b[b.size() - 1 - i]);
        }
        line += "0" + b;
    }
    line = line.substr(0, 272);
    while (line.size() % 2 == 0) {
        string next(line.size() / 2, '0');
        for (size_t i = 0; i < line.size(); i += 2) {
            if (line[i] == line[i + 1]) {
                next[i / 2] = '1';
            }
        }
        line = next;
    }
    cout << "Result: " << line << endl;
}

