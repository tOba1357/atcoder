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
    vector<int> counts;
    int before_ans = 0;

    int ci = 0;
    int count = 0;
    int count1 = 0;
    bool init = true;
    bool init1 = false;
    bool start = false;
    for (int i = 0; i < S.length(); i++) {
        char s = S[i];
        if (!start) {
            if (s == '1') {
                count++;
                count1++;
                continue;
            } else {
                cout << count << endl;
                start = true;
            }
        }
        if (s == '0') {
            if (init) {
                int index = ci - K;
                printf("index: %d\n", index);
                before_ans += count1;
                if (index < 0) {
                    ans = before_ans;
                } else {
                    before_ans = before_ans - counts[index];
                    cout << before_ans << endl;
                    ans = max(ans, before_ans);
                }
                init = false;
                count1 = 0;
            }
            count++;
            count1++;
            init1 = true;
        } else {
            if (init1) {
                counts.push_back(count);
                ci++;
                count = 0;
                init1 = false;
            }
            init = true;
            count++;
            count1++;
        }
    }
    if (count > 0) {
        if (init) {
            printf("count: %d before_ans: %d\n", count, before_ans);
            ans = max(ans, before_ans + count);
        } else {
            int index = ci - K;
            before_ans += count1;
            if (index < 0) {
                ans = before_ans;
            } else {
                before_ans = before_ans - counts[index];
                ans = max(ans, before_ans);
            }
        }
    }

    for (int i = 0; i < counts.size(); i++) cout << counts[i] << endl;
    cout << count << endl;
    cout << ans << endl;
}