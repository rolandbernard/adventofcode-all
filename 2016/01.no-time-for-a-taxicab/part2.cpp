
#include <bits/stdc++.h>

using namespace std;

int main() {
    int dr[] = {1, 0, -1, 0, 1};
    int x = 0, y = 0;
    int r = 0;
    char d;
    int a;
    set<pair<int, int>> visited;
    visited.insert(make_pair(0, 0));
    while (cin >> d) {
        cin >> a;
        if (d == 'L') {
            r = (r + 4 - 1) % 4;
        } else if (d == 'R') {
            r = (r + 1) % 4;
        }
        for (int i = 0; i < a; i++) {
            x += dr[r];
            y += dr[r + 1];
            if (visited.find(make_pair(x, y)) != visited.end()) {
                cout << "Result: " << (abs(x) + abs(y)) << endl;
                return 0;
            }
            visited.insert(make_pair(x, y));
        }
        cin >> d;
    }
}

