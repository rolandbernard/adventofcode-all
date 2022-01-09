
#include <bits/stdc++.h>

using namespace std;

int main() {
    int count;
    cin >> count;
    vector<int> left(count);
    iota(left.begin(), left.end(), 1);
    while (left.size() != 1) {
        for (size_t i = 0, j = 0; i < left.size(); i += 2, j++) {
            left[j] = left[i];
        }
        if (left.size() % 2 == 1) {
            left.erase(left.begin());
        }
        left.resize(left.size() / 2);
    }
    cout << "Result: " << left[0] << endl;
}

