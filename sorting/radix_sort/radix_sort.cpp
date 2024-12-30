/*
    Radix Sort Algorithm
    --------------------

    Time Complexity - O(d*(n + b)),
        where d is the max number of digits, n is the size of the input & b is the base of the system.

    Space Complexity - O(n + b),
        where n is the size of the input & b is the base of the system.
*/

#include <iostream>
#include <cstdlib>
#include <ctime>

using namespace std;

// Gets the maximum element in array (To find the max number of digits).
int getMax(int arr[], int size) {
    if (size == 0) {
        cout << "The Array to Sort is Empty!" << endl;
        exit(1);
    }
    int max = arr[0];
    for (int i = 1; i < size; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}

// Finds the smallest negative number to be used as shift to handle negative number sorting.
int getShiftVal(int arr[], int size) {
    int shift = 0;
    for (int i = 0; i < size; i++) {
        if (arr[i] < 0 && arr[i] < shift) {
            shift = arr[i];
        }
    }
    return abs(shift);
}

// Shifts the elements by shift value to handle negative number sorting.
void shiftElements(int arr[], int size, int shift) {
    for (int i = 0; i < size; i++) {
        arr[i] += shift;
    }
}

// Count Sort Algorithm.
void countSort(int arr[], int size, int pos) {
    int digit_frequency[10] = {0}; // Sorting decimal numbers here, hence base is 10.

    // Increments the freq of elements in each bucket.
    for (int i = 0; i < size; i++) {
        ++digit_frequency[(arr[i]/pos) % 10]; 
    }

    // Calculates the cumulative frequency.
    for (int i = 1; i < 10; i++) {
        digit_frequency[i] += digit_frequency[i - 1];
    }

    // Builds the sorted array from the right to maintain the stability property of Radix Sort.
    int sorted_arr[size];
    for (int i = size-1; i >= 0; i--) {
        int idx = (arr[i]/pos) % 10;
        sorted_arr[--digit_frequency[idx]] = arr[i]; 
    }

    // Copies the sorted array to the original array.
    for (int i = 0; i < size; i++) {
        arr[i] = sorted_arr[i];  
    }
}

// Radix Sort Algorithm.
void radixSort(int arr[], int size) {
    int max = getMax(arr, size);
    int shift = getShiftVal(arr, size);
    shiftElements(arr, size, shift); // Shifts the elements to remove negative values.
    for(int pos = 1; max/pos > 0; pos *= 10) { 
        countSort(arr, size, pos); // Calls countSort() for every digit position.
    }
    shiftElements(arr, size, -shift); // Shifts the elements back to original values after sorting.
}

// Helper function to print elements of array.
void printArray(int arr[], int size) {
    for (int i = 0; i < size; i++) {
        cout << arr[i] << " ";
    }
    cout << endl;
}

int main() {
    const int size = 20;
    int arr[size];

    // Create a worst case unsorted array (descending elements).
    // for (int i = 0; i < size; i++) {
    //     arr[i] = size - i; 
    // }

    // Create an unsorted array with random numbers.
    srand((unsigned) time(0));
    for (int i = 0; i < size; i++) {
        arr[i] = (rand() % 1001) - 500;
    }

    cout << "Unsorted Array: " << endl;
    printArray(arr, size);

    radixSort(arr, size);

    cout << "Sorted Array: " << endl;
    printArray(arr, size);

    return 0;
}
