
#include <bits/stdc++.h>

using namespace std;

char leastCommonAt(const vector<string>& signal, size_t idx) {
    size_t freq[26];
    memset(freq, 0, sizeof(freq));
    for (const string& s : signal) {
        freq[s[idx] - 'a']++;
    }
    int min = 0;
    for (int i = 1; i < 26; i++) {
        if (freq[i] != 0 && freq[i] <= freq[min]) {
            min = i;
        }
    }
    return min + 'a';
}

int main() {
    vector<string> lines;
    string line;
    while (getline(cin, line)) {
        lines.push_back(line);
    }
    char res[lines[0].size() + 1];
    res[lines[0].size()] = 0;
    for (size_t i = 0; i < lines[0].size(); i++) {
        res[i] = leastCommonAt(lines, i);
    }
    cout << "Result: " << res << endl;
}

