
#include <bits/stdc++.h>

using namespace std;

#define GOAL_X 31
#define GOAL_Y 39

struct pair_hash {
    template <class T1, class T2>
    std::size_t operator () (const std::pair<T1,T2> &p) const {
        auto h1 = std::hash<T1>{}(p.first);
        auto h2 = std::hash<T2>{}(p.second);
        return h1 ^ h2;  
    }
};

bool isFree(int x, int y, int magic) {
    int m = x*x + 3*x + 2*x*y + y + y*y + magic;
    int bits = 0;
    while (m != 0) {
        bits++;
        m -= m & -m;
    }
    return (bits % 2) == 0;
}

int main() {
    int magic;
    cin >> magic;
    unordered_map<pair<int, int>, int, pair_hash> dists;
    queue<pair<int, int>> queue;
    auto try_to_add = [&queue, &dists] (pair<int, int> p, int d) {
        if (dists.find(p) == dists.end()) {
            dists[p] = d;
            queue.push(p);
        }
    };
    dists[make_pair(1, 1)] = 0;
    queue.push(make_pair(1, 1));
    while (!queue.empty()) {
        pair<int, int> pos = queue.front();
        queue.pop();
        if (pos.first == GOAL_X && pos.second == GOAL_Y) {
            break;
        } else {
            int dist = dists[pos];
            if (pos.first > 0 && isFree(pos.first - 1, pos.second, magic)) {
                try_to_add(make_pair(pos.first - 1, pos.second), dist + 1);
            }
            if (pos.second > 0 && isFree(pos.first, pos.second - 1, magic)) {
                try_to_add(make_pair(pos.first, pos.second - 1), dist + 1);
            }
            if (isFree(pos.first + 1, pos.second, magic)) {
                try_to_add(make_pair(pos.first + 1, pos.second), dist + 1);
            }
            if (isFree(pos.first, pos.second + 1, magic)) {
                try_to_add(make_pair(pos.first, pos.second + 1), dist + 1);
            }
        }
    }
    cout << "Result: " << dists[make_pair(GOAL_X, GOAL_Y)] << endl;
}

