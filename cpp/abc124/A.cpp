#include "iostream"

using namespace std;
int main() {
    int a,b;
    cin >> a >> b;
    if (a == b) {
        cout << a + b << endl;
    } else {
        int v = max(a, b);
        cout << v + v - 1 << endl;
    }
}