
#include <bits/stdc++.h>

using namespace std;

int main() {
    vector<pair<uint64_t, uint64_t>> discs;
    string line;
    while (getline(cin, line)) {
        size_t has = line.find("has ");
        size_t pos = line.find(" positions");
        uint64_t n = stoi(line.substr(has + 4, pos - has - 4));
        has = line.find("position ");
        pos = line.find(".");
        uint64_t o = stoi(line.substr(has + 9, pos - has - 9));
        discs.push_back(make_pair(n, o));
    }
    uint64_t time = 0;
    uint64_t mul = 1;
    for (uint64_t i = 0; i < discs.size(); i++) {
        while ((time + 1 + i + discs[i].second) % discs[i].first != 0) {
            time += mul;
        }
        mul = lcm(mul, discs[i].first);
    }
    cout << "Result: " << time << endl;
}

