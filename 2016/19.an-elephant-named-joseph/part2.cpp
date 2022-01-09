
#include <bits/stdc++.h>

using namespace std;

int main() {
    int count;
    cin >> count;
    vector<bool> elim(count);
    queue<int> skip;
    int left = count;
    int about = count;
    int j = 0;
    while (left != 1) {
        for (int i = 0; i < count; i++) {
            if (!elim[i]) {
                if (!skip.empty() && skip.front() == j) {
                    left--;
                    elim[i] = true;
                    skip.pop();
                } else {
                    skip.push(j + (about / 2));
                    about--;
                    j++;
                }
            }
        }
    }
    cout << "Result: " << (find(elim.begin(), elim.end(), false) - elim.begin() + 1) << endl;
}

