
#include <bits/stdc++.h>

using namespace std;

bool supportsTls(const string& s) {
    bool inside = false;
    bool valid = false;
    for (size_t i = 3; i < s.size(); i++) {
        if (s[i - 3] == s[i] && s[i - 2] == s[i - 1] && s[i] != s[i - 1]) {
            if (inside) {
                return false;
            } else {
                valid = true;
            }
        } else if (s[i] == '[') {
            inside = true;
        } else if (s[i] == ']') {
            inside = false;
        }
    }
    return valid;
}

int main() {
    size_t count = 0;
    string line;
    while (getline(cin, line)) {
        if (supportsTls(line)) {
            count++;
        }
    }
    cout << "Result: " << count << endl;
}

