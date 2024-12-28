/*
    Radix Sort Algorithm
    ----------------------

    Time Complexity - O(d*(n + b)) 
        where d is the max number of digits, n is the size of the input & b is the base of the system

    Space Complexity - O(n + b)
        where n is the size of the input & b is the base of the system
*/

#include <iostream>
#include <cstdlib>
#include <ctime>

using namespace std;

// Gets the maximum element in array (To find the max number of digits)
int getMax(int arr[], int num) {
    int max = arr[0];
    for (int i = 1; i < num; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}

// Count Sort Algorithm
void countSort(int arr[], int num, int pos) {
    int count[10] = {0}; // Sorting numbers here, hence base 10

    // Increments the freq of elements in each bucket
    for (int i = 0; i < num; i++) {
        ++count[(arr[i]/pos) % 10]; 
    }

    // Calculates the cumulative freq 
    for (int i = 1; i < 10; i++) {
        count[i] += count[i - 1];
    }

    // Builds the sorted array from the right to maintain the stability property of Radix Sort
    int sorted_arr[num];
    for (int i = num-1; i >= 0; i--) {
        int idx = (arr[i]/pos) % 10;
        sorted_arr[--count[idx]] = arr[i]; 
    }

    // Copies the sorted array to the original array
    for (int i = 0; i < num; i++) {
        arr[i] = sorted_arr[i];  
    }
}

// Radix Sort Algorithm
void radixSort(int arr[], int num) {
    int max = getMax(arr, num);
    for(int pos = 1; max/pos > 0; pos *= 10) { 
        countSort(arr, num, pos); // Calls countSort() for every digit position
    }
}

// Helper function to print elements of array
void printArray(int arr[], int num) {
    for (int i = 0; i < num; i++) {
        cout << arr[i] << " ";
    }
    cout << endl;
}

int main() {
    const int num = 100; // Number of elements in the unsorted array
    int arr[num];

    // Create a worst case unsorted array (descending elements)
    // for (int i = 0; i < num; i++) {
    //     arr[i] = num - i; 
    // }

    // Create an unsorted array with random numbers
    srand((unsigned) time(0));
    for (int i = 0; i < num; i++) {
        arr[i] = (rand() % 1000) + 1;
    }

    cout << "Unsorted Array: " << endl;
    printArray(arr, num);

    radixSort(arr, num);

    cout << "Sorted Array: " << endl;
    printArray(arr, num);

    return 0;
}
