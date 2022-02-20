
#include <bits/stdc++.h>

using namespace std;

#define LENGTH 35651584

char dataAt(size_t cs, size_t at, const string& orig) {
    if (cs == 0) {
        size_t n = orig.size() + 1;
        if (at % n == n - 1) {
            size_t t = at / n + 1;
            return (t & ((t & -t) << 1)) == 0 ? '0' : '1';
        } else if ((at / n) % 2 == 0) {
            return orig[at % n];
        } else {
            return orig[orig.size() - 1 - at % n] == '0' ? '1' : '0';
        }
    } else if (dataAt(cs - 1, 2*at, orig) == dataAt(cs - 1, 2*at + 1, orig)) {
        return '1';
    } else {
        return '0';
    }
}

int main() {
    string line;
    cin >> line;
    size_t length = LENGTH;
    size_t comp_level = 0;
    while (length % 2 == 0) {
        length /= 2;
        comp_level++;
    }
    string check(length, '0');
    for (size_t i = 0; i < length; i++) {
        check[i] = dataAt(comp_level, i, line);
    }
    cout << "Result: " << check << endl;
}

