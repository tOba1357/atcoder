#include "iostream"
#include "vector"
using namespace std;
int main() {
    int N;
    cin >> N;
    int count = 0;
    vector<int> H(N, 0);
    for (int i = 0; i < N; i++) {
        int h;
        cin >> h;
        H[i] = h;
        bool r = true;
        for (int j = 0; j < i; j++) {
            if(H[j] > h) {
                r = false;
            }
        }
        if (r) count++;
    }
    cout << count << endl;

}