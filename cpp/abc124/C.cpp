#include "iostream"
using namespace std;

int main() {
    string S;
    cin >> S;
    int b_count = 0;
    int w_count = 0;
    for (int i = 0; i < S.length(); i++) {
        if ((i % 2) == 0) {
            if(S[i] == '0') {
                w_count++;
            } else {
                b_count++;
            }
        } else {
            if(S[i] == '1') {
                w_count++;
            } else {
                b_count++;
            }
        }
    }
    cout << min(w_count, b_count) << endl;
}