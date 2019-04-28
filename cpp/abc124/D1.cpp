#include "iostream"
#include "vector"
#include "algorithm"
#include <functional>

using namespace std;

int main() {
    int N, K;
    cin >> N >> K;
    string S;
    cin >> S;
    int ans = 0;
    int c0 = 0;
    vector<int> counts;
    bool b1 = true;
    for (int i = 0; i < S.length(); i++)  {
        char s = S[i];
        if (s == '0') {
            if (b1) {
                counts.push_back(i - 1);
            }
            b1 = false;
            c0++;
        } else {
            b1 = true;
            c0 = 0;
        }
    }
    if (b1) {
        if (counts.size() == 0) {
            cout << N << endl;
            return 0;
        }
//        counts[counts.size() - 1] = S.length() - 1;
        counts.push_back(S.length());
    } else {
        counts.push_back(S.length());
    }
    for (int i = 0; i < counts.size(); i++) {
        int c = counts[i];
        if (i < K) {
            ans = max(ans, c);
        } else {
            ans = max(ans, c - counts[i - K]);
        }
    }
    cout << ans << endl;
}