
#include <bits/stdc++.h>

using namespace std;

int main() {
    regex pattern("([a-z-]+)-(\\d+)\\[([a-z]+)\\]");
    string line;
    while (getline(cin, line)) {
        smatch match;
        regex_search(line, match, pattern);
        int freq[26];
        memset(freq, 0, sizeof(freq));
        string name = match[1].str();
        for (char c : name) {
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
            int id = stoi(match[2].str());
            for (char& c : name) {
                if (isalpha(c)) {
                    c = 'a' + (c - 'a' + id) % 26;
                }
            }
            if (name.find("northpole") == 0) {
                cout << "Result: " << id << endl;
                return 0;
            }
        }
    }
}

