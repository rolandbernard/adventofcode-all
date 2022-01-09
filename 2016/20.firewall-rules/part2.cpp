
#include <bits/stdc++.h>

using namespace std;

int main() {
    vector<uint32_t> changes;
    unordered_multiset<uint32_t> starts;
    unordered_multiset<uint32_t> ends;
    size_t from, to;
    while (cin >> from) {
        cin.ignore(1);
        cin >> to;
        changes.push_back(from);
        changes.push_back(to);
        starts.insert(from);
        ends.insert(to);
    }
    sort(changes.begin(), changes.end());
    changes.resize(unique(changes.begin(), changes.end()) - changes.begin());
    uint32_t active = 0;
    uint32_t i = 0;
    uint32_t pos = 0;
    uint32_t count = 0;
    uint32_t allowed_since = 0;
    while (pos < UINT32_MAX) {
        if (active == 0) {
            count += pos - allowed_since;
        }
        active += starts.count(pos);
        if (active == 0) {
            allowed_since = pos;
        }
        active -= ends.count(pos);
        if (active == 0 && ends.count(pos) != 0) {
            pos++;
            allowed_since = pos;
        } else {
            while (i < changes.size() && pos >= changes[i]) {
                i++;
            }
            if (i < changes.size()) {
                pos = changes[i];
            } else {
                pos = UINT_MAX;
            }
        }
    }
    if (active == 0) {
        count += pos - allowed_since;
    }
    cout << "Result: " << count << endl;
}

