
#include <bits/stdc++.h>

using namespace std;

bool supportsSsl(const string& s) {
    int inside = 0;
    set<pair<char, char>> abas[2];
    for (size_t i = 2; i < s.size(); i++) {
        if (s[i - 2] == s[i] && s[i] != s[i - 1]) {
            if (abas[inside].find(make_pair(s[i - 1], s[i])) != abas[inside].end()) {
                return true;
            } else {
                abas[1 - inside].insert(make_pair(s[i], s[i - 1]));
            }
        } else if (s[i] == '[') {
            inside = 1;
        } else if (s[i] == ']') {
            inside = 0;
        }
    }
    return false;
}

int main() {
    size_t count = 0;
    string line;
    while (getline(cin, line)) {
        if (supportsSsl(line)) {
            count++;
        }
    }
    cout << "Result: " << count << endl;
}

