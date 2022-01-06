
#include <bits/stdc++.h>

using namespace std;

int main() {
    int count = 0;
    regex pattern("([a-z-]+)-(\\d+)\\[([a-z]+)\\]");
    string line;
    while (getline(cin, line)) {
        smatch match;
        regex_search(line, match, pattern);
        int freq[26];
        memset(freq, 0, sizeof(freq));
        for (char c : match[1].str()) {
            freq[c - 'a']++;
        }
        char ord[26];
        iota(ord, ord + 26, 'a');
        sort(ord, ord + 26, [&freq](char a, char b) {
            if (freq[a - 'a'] != freq[b - 'a']) {
                return freq[a - 'a'] > freq[b - 'a'];
            } else {
                return a < b;
            }
        });
        string csm = match[3].str();
        bool valid = true;
        for (int i = 0; valid && i < 5; i++) {
            if (ord[i] != csm[i]) {
                valid = false;
            }
        }
        if (valid) {
            count += stoi(match[2].str());
        }
    }
    cout << "Result: " << count << endl;
}

