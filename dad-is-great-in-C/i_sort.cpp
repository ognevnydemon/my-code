#include <iostream>
#include <random>
#include <ctime>
using namespace std;
int main() {
    int n;
    cin >> n;
    clock_t st = clock();
    int a[n];
    srand(time(NULL));
    for (int i = 0; i < n; i++) a[i] = rand();

    for (int i = 0; i < n; i++) {
        int j = i - 1;
        int k = a[i];
        while (a[j] > k and j >= 0) { a[j + 1] = a[j]; j--; }
        a[j + 1] = k; }

    //for (int i : a) cout << i << " ";
    clock_t end = clock();
    cout << (double)(end - st) / CLOCKS_PER_SEC;
    return 0; }