
#include <bits/stdc++.h>

using namespace std;

#define vec vector
#define pos pair<size_t, size_t>

vec<size_t> getDistsToAll(const vec<vec<bool>>& walls, pos from, const vec<pos>& to) {
    size_t left = to.size();
    vec<vec<size_t>> dist(walls.size(), vec<size_t>(walls[0].size(), SIZE_MAX));
    queue<pos> todo;
    dist[from.first][from.second] = 0;
    todo.push(from);
    while (left > 0 && !todo.empty()) {
        pos t = todo.front();
        todo.pop();
        if (find(to.begin(), to.end(), t) != to.end()) {
            left--;
        }
        size_t x = t.first, y = t.second;
        size_t d = dist[x][y] + 1;
        if (!walls[x - 1][y] && dist[x - 1][y] > d) {
            dist[x - 1][y] = d;
            todo.push(make_pair(x - 1, y));
        }
        if (!walls[x + 1][y] && dist[x + 1][y] > d) {
            dist[x + 1][y] = d;
            todo.push(make_pair(x + 1, y));
        }
        if (!walls[x][y - 1] && dist[x][y - 1] > d) {
            dist[x][y - 1] = d;
            todo.push(make_pair(x, y - 1));
        }
        if (!walls[x][y + 1] && dist[x][y + 1] > d) {
            dist[x][y + 1] = d;
            todo.push(make_pair(x, y + 1));
        }
    }
    vec<size_t> res;
    for (pos dst : to) {
        res.push_back(dist[dst.first][dst.second]);
    }
    return res;
}

vec<vec<size_t>> getDistsMap(const vec<vec<bool>>& walls, const vec<pos>& goals) {
    vec<vec<size_t>> ret;
    for (pos start : goals) {
        ret.push_back(getDistsToAll(walls, start, goals));
    }
    return ret;
}

size_t shortestPath(const vec<vec<size_t>>& dists, size_t cur, size_t visited, vec<unordered_map<size_t, size_t>>& mem) {
    if (visited == ((size_t)1 << dists.size()) - 1) {
        return 0;
    } else {
        if (mem[cur].find(visited) == mem[cur].end()) {
            size_t best = SIZE_MAX;
            for (size_t i = 0; i < dists.size(); i++) {
                if ((visited & (1 << i)) == 0) {
                    size_t dist = shortestPath(dists, i, visited | (1 << i), mem);
                    if (dist != SIZE_MAX && dist + dists[cur][i] < best) {
                        best = dist + dists[cur][i];
                    }
                }
            }
            mem[cur][visited] = best;
        }
        return mem[cur][visited];
    }
}

size_t shortestPath(const vec<vec<size_t>>& dists) {
    vec<unordered_map<size_t, size_t>> mem(dists.size());
    return shortestPath(dists, 0, 1, mem);
}

int main() {
    vec<pos> goals(1);
    vec<vec<bool>> walls;
    size_t j = 0;
    string row;
    while (getline(cin, row)) {
        walls.push_back(vec<bool>(row.size()));
        for (size_t i = 0; i < row.size(); i++) {
            if (row[i] == '#') {
                walls.back()[i] = true;
            } else if (row[i] == '0') {
                goals[0] = make_pair(j, i);
            } else if (isdigit(row[i])) {
                goals.push_back(make_pair(j, i));
            }
        }
        j++;
    }
    vec<vec<size_t>> dists = getDistsMap(walls, goals);
    size_t best = shortestPath(dists);
    cout << "Result: " << best << endl;
}

