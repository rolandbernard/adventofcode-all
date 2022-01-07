
#include <bits/stdc++.h>

using namespace std;

size_t decompressedSize(const string& s, size_t from, size_t to) {
    size_t res = 0;
    for (size_t i = from; i < to;) {
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
            res += mult * decompressedSize(s, i, i + length);
            i += length;
        } else {
            res += 1;
            i++;
        }
    }
    return res;
}

int main() {
    size_t count = 0;
    string line;
    while (getline(cin, line)) {
        count += decompressedSize(line, 0, line.size());
    }
    cout << "Result: " << count << endl;
}

