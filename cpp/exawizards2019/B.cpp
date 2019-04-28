#include <iostream>
using namespace std;
int main() {
    int N;
    cin >> N;
    string s;
    cin >> s;
    int rc = 0, bc = 0;
    for (int i = 0; i < N; i++) {
        if (s[i] == 'R') {
            rc++;
        } else {
            bc++;
        }
    }
    if (rc > bc) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}
