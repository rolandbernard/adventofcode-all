
#include <bits/stdc++.h>

using namespace std;

#define ELEMS 7
#define ITEMS (2 * ELEMS)

struct GameState {
    uint64_t state = 0;

    uint64_t ele() {
        return state & 0b11;
    }

    void ele(uint64_t ele) {
        state = (state & ~(uint64_t)0b11) | ele;
    }

    uint64_t floor(uint64_t f) {
        return (state >> (2 + f * ITEMS)) & ((1UL << ITEMS) - 1);
    }

    void floor(uint64_t f, uint64_t val) {
        state |= (uint64_t)val << (2 + f * ITEMS);
    }

    bool solved() {
        return ((state >> 2) & ((1UL << (3 * ITEMS)) - 1)) == 0;
    }

    bool canMove(uint64_t t, uint64_t val) {
        uint64_t n = floor(ele()) & ~val;
        uint64_t m = floor(ele() + t) | val;
        return
            ((n >> ELEMS) == 0 || ((n & ((1UL << ELEMS) - 1)) & ~(n >> ELEMS)) == 0)
            && ((m >> ELEMS) == 0 || ((m & ((1UL << ELEMS) - 1)) & ~(m >> ELEMS)) == 0);
    }

    void move(uint64_t t, uint64_t val) {
        state &= ~((uint64_t)val << (2 + ele() * ITEMS));
        state |= (uint64_t)val << (2 + (ele() + t) * ITEMS);
        ele(ele() + t);
    }

    size_t stepsToSolve() {
        unordered_map<uint64_t, size_t> best;
        queue<uint64_t> queue;
        best[state] = 0;
        queue.push(state);
        while (!queue.empty()) {
            GameState state;
            state.state = queue.front();
            queue.pop();
            if (state.solved()) {
                return best[state.state];
            } else {
                size_t cur = best[state.state];
                for (uint64_t ki = state.floor(state.ele()), i = 0; ki != 0; ki -= i, i = ki & -ki) {
                    for (uint64_t kj = state.floor(state.ele()), j = 0; kj != 0; kj -= j, j = kj & -kj) {
                        if (i < j) {
                            if (state.ele() != 3 && state.canMove(+1, i | j)) {
                                GameState next(state);
                                next.move(+1, i | j);
                                if (best.find(next.state) == best.end()) {
                                    best[next.state] = cur + 1;
                                    queue.push(next.state);
                                }
                            }
                            if (state.ele() != 0 && state.canMove(-1, i | j)) {
                                GameState next(state);
                                next.move(-1, i | j);
                                if (best.find(next.state) == best.end()) {
                                    best[next.state] = cur + 1;
                                    queue.push(next.state);
                                }
                            }
                        }
                    }
                }
            }
        }
        return SIZE_MAX;
    }
};

size_t getType(unordered_map<string, size_t>& types, string type) {
    if (types.find(type) == types.end()) {
        types[type] = types.size();
    }
    return types[type];
}

int main() {
    unordered_map<string, size_t> types;
    GameState initial;
    string line;
    for (int i = 0; cin >> line; i++) {
        cin >> line >> line >> line >> line;
        if (line == "a") {
            int floor = 0;
            while (line == "a") {
                cin >> line;
                size_t x = line.find('-');
                if (x != line.npos) {
                    floor |= 1 << getType(types, line.substr(0, x));
                } else {
                    floor |= 1 << (getType(types, line) + ELEMS);
                }
                cin >> line;
                if (line[line.size() - 1] != '.') {
                    cin >> line;
                    if (line == "and") {
                        cin >> line;
                    }
                }
            }
            initial.floor(i, floor);
        }
    }
    size_t res = initial.stepsToSolve();
    cout << "Result: " << res << endl;
}

