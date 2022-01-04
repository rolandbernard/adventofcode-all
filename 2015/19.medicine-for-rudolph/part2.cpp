
#include <bits/stdc++.h>

using namespace std;

typedef vector<int> Molecule;
typedef unordered_map<int, vector<Molecule>> Replacements;
typedef vector<vector<vector<int>>> Cache;

int minSteps(int atom, Molecule& med, size_t from, size_t to, Replacements& reps, Cache& cache);

int tryReplacement(Molecule& rep, size_t i, Molecule& med, size_t from, size_t to, Replacements& reps, Cache& cache) {
    if (i == rep.size() - 1) {
        return minSteps(rep[i], med, from, to, reps, cache);
    } else {
        int min = -1;
        for (size_t j = from + 1; j + rep.size() <= to + i + 1; j++) {
            int head = minSteps(rep[i], med, from, j, reps, cache);
            if (head != -1) {
                int tail = tryReplacement(rep, i + 1, med, j, to, reps, cache);
                if (tail != -1 && (min == -1 || head + tail < min)) {
                    min = head + tail;
                }
            }
        }
        return min;
    }
}

int minSteps(int atom, Molecule& med, size_t from, size_t to, Replacements& reps, Cache& cache) {
    if (from + 1 == to && atom == med[from]) {
        return 0;
    } else if (cache[atom][from][to] != 0) {
        return cache[atom][from][to];
    } else {
        int min = -1;
        cache[atom][from][to] = -1;
        for (Molecule& rep : reps[atom]) {
            int tmp = tryReplacement(rep, 0, med, from, to, reps, cache);
            if (tmp != -1 && (min == -1 || tmp < min)) {
                min = tmp + 1;
            }
        }
        cache[atom][from][to] = min;
        return min;
    }
}

Molecule splitAtoms(string& s, unordered_map<string, int>& atoms) {
    Molecule ret;
    size_t start = 0;
    for (size_t i = 1; i <= s.size(); i++) {
        if (i == s.size() || isupper(s[i])) {
            string atom = s.substr(start, i - start);
            if (atoms.find(atom) == atoms.end()) {
                atoms[atom] = atoms.size();
            }
            ret.push_back(atoms[atom]);
            start = i;
        }
    }
    return ret;
}

int main() {
    unordered_map<string, int> atoms;
    Replacements reps;
    Molecule meds;
    string atom;
    while (cin >> atom) {
        string rep;
        if (cin >> rep) {
            if (atoms.find(atom) == atoms.end()) {
                atoms[atom] = atoms.size();
            }
            cin >> rep;
            reps[atoms[atom]].push_back(splitAtoms(rep, atoms));
        } else {
            meds = splitAtoms(atom, atoms);
        }
    }
    Cache cache(atoms.size(), vector<vector<int>>(meds.size() + 1, vector<int>(meds.size() + 1)));
    cout << "Result: " << minSteps(atoms["e"], meds, 0, meds.size(), reps, cache) << endl;
}

