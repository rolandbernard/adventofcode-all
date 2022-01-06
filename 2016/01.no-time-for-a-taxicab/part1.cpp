
#include <bits/stdc++.h>

using namespace std;

int main() {
    int dr[] = {1, 0, -1, 0, 1};
    int x = 0, y = 0;
    int r = 0;
    char d;
    int a;
    while (cin >> d) {
        cin >> a;
        if (d == 'L') {
            r = (r + 4 - 1) % 4;
        } else if (d == 'R') {
            r = (r + 1) % 4;
        }
        x += dr[r] * a;
        y += dr[r + 1] * a;
        cin >> d;
    }
    cout << "Result: " << (abs(x) + abs(y)) << endl;
}

