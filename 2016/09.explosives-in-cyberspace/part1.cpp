
#include <bits/stdc++.h>

using namespace std;

string decompress(const string& s) {
    string res = "";
    for (size_t i = 0; i < s.size();) {
        if (s[i] == '(') {
            size_t length = 0;
            i++;
            while (isdigit(s[i])) {
                length *= 10;
                length += s[i] - '0';
                i++;
            }
            i++;
            size_t mult = 0;
            while (isdigit(s[i])) {
                mult *= 10;
                mult += s[i] - '0';
                i++;
            }
            i++;
            for (size_t m = 0; m < mult; m++) {
                for (size_t j = i; j < i + length; j++) {
                    res += s[j];
                }
            }
            i += length;
        } else {
            res += s[i];
            i++;
        }
    }
    return res;
}

int main() {
    size_t count = 0;
    string line;
    while (getline(cin, line)) {
        count += decompress(line).size();
        cerr << decompress(line) << endl;
    }
    cout << "Result: " << count << endl;
}

