#include <iostream>
#include <vector>

using namespace std;


int find_l(int N, int Q, string &s, vector<char> &ts, vector<char> &ds) {
//    左に落ちるのを探す
    int l_index = 0, r_index = N + 1;
    while ((r_index - l_index) > 1) {
        int target_index = (l_index + r_index) / 2;
        int current_cell = target_index;
        for (int i = 0; i < Q; i++) {
            char t = ts[i], d = ds[i];
            if (s[current_cell - 1] == t) {
                if (d == 'L') {
                    current_cell--;
                } else {
                    current_cell++;
                }
            }
            if (current_cell == 0) break;
            if (current_cell == (N + 1)) break;
        }
        if (current_cell != 0) {
            r_index = target_index;
        } else {
            l_index = target_index;
        }
    }
    return l_index;
}

int find_r(int N, int Q, string &s, vector<char> &ts, vector<char> &ds) {
//    右に落ちるのを探す
    int l_index = 0, r_index = N + 1;
    while ((r_index - l_index) > 1) {
        int target_index = (l_index + r_index) / 2;
        int current_cell = target_index;
        for (int i = 0; i < Q; i++) {
            char t = ts[i], d = ds[i];
            if (s[current_cell - 1] == t) {
                if (d == 'L') {
                    current_cell--;
                } else {
                    current_cell++;
                }
            }
            if (current_cell == (N + 1)) break;
            if (current_cell == 0) break;
        }
        if (current_cell != (N + 1)) {
            l_index = target_index;
        } else {
            r_index = target_index;
        }
    }
    return r_index;
}


int main() {
    int N, Q;
    cin >> N >> Q;
    string s;
    cin >> s;
    vector<char> ts(Q), ds(Q);
    for (int i = 0; i < Q; i++) {
        char t, d;
        cin >> t >> d;
        ts[i] = t;
        ds[i] = d;
    }

    int l_ans = find_l(N, Q, s, ts, ds);
    int r_ans = find_r(N, Q, s, ts, ds);
    cout << r_ans - l_ans - 1 << endl;
}
