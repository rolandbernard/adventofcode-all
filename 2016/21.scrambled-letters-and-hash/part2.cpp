
#include <bits/stdc++.h>

using namespace std;

int main() {
    string line;
    vector<string> lines;
    while (getline(cin, line)) {
        lines.push_back(line);
    }
    string passwd = "fbgdceah";
    for (size_t i = 0; i < lines.size(); i++) {
        stringstream ss(lines[lines.size() - 1 - i]);
        string part;
        int a, b;
        char c, d;
        ss >> part;
        if (part == "swap") {
            ss >> part;
            if (part == "position") {
                ss >> a >> part >> part >> b;
                swap(passwd[a], passwd[b]);
            } else {
                ss >> c >> part >> part >> d;
                for (char& x : passwd) {
                    if (x == c) {
                        x = d;
                    } else if (x == d) {
                        x = c;
                    }
                }
            }
        } else if (part == "rotate") {
            ss >> part;
            if (part == "based") {
                ss >> part >> part >> part >> part >> c;
                a = passwd.find(c);
                b = 0;
                while (a != (int)(2*b + (b >= 4 ? 2 : 1)) % passwd.size()) {
                    b++;
                }
                a = (b + (b >= 4 ? 2 : 1)) % passwd.size();
                rotate(passwd.begin(), passwd.begin() + a, passwd.end());
            } else {
                ss >> a;
                if (part == "left") {
                    a = (passwd.size() - a) % passwd.size();
                }
                rotate(passwd.begin(), passwd.begin() + a, passwd.end());
                ss >> part;
            }
        } else if (part == "reverse") {
            ss >> part >> a >> part >> b;
            reverse(passwd.begin() + a, passwd.begin() + b + 1);
        } else {
            ss >> part >> a >> part >> part >> b;
            c = passwd[b];
            passwd.erase(passwd.begin() + b);
            passwd.insert(passwd.begin() + a, c);
        }
    }
    cerr << "Result: " << passwd << endl;
}

