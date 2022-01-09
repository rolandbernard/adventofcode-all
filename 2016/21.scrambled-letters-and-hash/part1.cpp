
#include <bits/stdc++.h>

using namespace std;

int main() {
    string passwd = "abcdefgh";
    string part;
    int a, b;
    char c, d;
    while (cin >> part) {
        if (part == "swap") {
            cin >> part;
            if (part == "position") {
                cin >> a >> part >> part >> b;
                swap(passwd[a], passwd[b]);
            } else {
                cin >> c >> part >> part >> d;
                for (char& x : passwd) {
                    if (x == c) {
                        x = d;
                    } else if (x == d) {
                        x = c;
                    }
                }
            }
        } else if (part == "rotate") {
            cin >> part;
            if (part == "based") {
                cin >> part >> part >> part >> part >> c;
                a = passwd.find(c);
                a = (2*passwd.size() - a - (a >= 4 ? 2 : 1)) % passwd.size();
                rotate(passwd.begin(), passwd.begin() + a, passwd.end());
            } else {
                cin >> a;
                if (part == "right") {
                    a = (passwd.size() - a) % passwd.size();
                }
                rotate(passwd.begin(), passwd.begin() + a, passwd.end());
                cin >> part;
            }
        } else if (part == "reverse") {
            cin >> part >> a >> part >> b;
            reverse(passwd.begin() + a, passwd.begin() + b + 1);
        } else {
            cin >> part >> a >> part >> part >> b;
            c = passwd[a];
            passwd.erase(passwd.begin() + a);
            passwd.insert(passwd.begin() + b, c);
        }
    }
    cerr << "Result: " << passwd << endl;
}

