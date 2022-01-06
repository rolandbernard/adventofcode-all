
#include <bits/stdc++.h>

using namespace std;

int main() {
    int count = 0;
    int t[3][3];
    while (
        cin >> t[0][0] >> t[1][0] >> t[2][0]
        >> t[0][1] >> t[1][1] >> t[2][1]
        >> t[0][2] >> t[1][2] >> t[2][2]
    ) {
        for (auto a : t) {
            if (a[0] + a[1] > a[2] && a[1] + a[2] > a[0] && a[2] + a[0] > a[1]) {
                count++;
            }
        }
    }
    cout << "Result: " << count << endl;
}

