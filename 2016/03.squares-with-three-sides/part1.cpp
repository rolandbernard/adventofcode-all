
#include <bits/stdc++.h>

using namespace std;

int main() {
    int count = 0;
    int a[3];
    while (cin >> a[0] >> a[1] >> a[2]) {
        if (a[0] + a[1] > a[2] && a[1] + a[2] > a[0] && a[2] + a[0] > a[1]) {
            count++;
        }
    }
    cout << "Result: " << count << endl;
}

