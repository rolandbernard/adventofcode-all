
#include <bits/stdc++.h>

using namespace std;

struct ListElem {
    long value;
    ListElem* prev;
    ListElem* next;
};

int main() {
    long players, points;
    string ign;
    cin >> players >> ign >> ign >> ign >> ign >> ign >> points >> ign;
    points *= 100;
    long scores[players];
    memset(scores, 0, sizeof(scores));
    ListElem* current = new ListElem;
    current->value = 0;
    current->prev = current;
    current->next = current;
    for (long i = 1; i <= points; i++) {
        long player = (i - 1) % players;
        if (i % 23 == 0) {
            for (long j = 0; j < 7; j++) {
                current = current->prev;
            }
            scores[player] += i + current->value;
            ListElem* to_remove = current;
            current = current->next;
            to_remove->prev->next = to_remove->next;
            to_remove->next->prev = to_remove->prev;
            delete to_remove;
        } else {
            current = current->next;
            ListElem* new_elem = new ListElem;
            new_elem->value = i;
            new_elem->prev = current;
            new_elem->next = current->next;
            new_elem->prev->next = new_elem;
            new_elem->next->prev = new_elem;
            current = current->next;
        }
    }
    long best = 0;
    for (long i = 0; i < players; i++) {
        best = max(best, scores[i]);
    }
    cout << "Result: " << best << endl;
}

