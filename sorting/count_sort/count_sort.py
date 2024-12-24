"""
Logic to handle negative numbers in counting sort was referred from the below link:
https://www.ripublication.com/ijaer18/ijaerv13n1_28.pdf
"""
def counting_sort(L):
    if not L:
        return L
        
    largest = max(L)
    min_value = min(L)
    offset = -min_value if min_value < 0 else 0
    n = len(L)

    # Initialise the frequency Array.
    frequencyArray = [0]*(largest + 1 + offset)
    for i in range(n):
        # Shift the element by the offset which ensures negative numbers 
        # are shifted to non-negative indices in the frequencyArray.
        frequencyArray[L[i] + offset] = frequencyArray[L[i] + offset]+1
        
    # Sorting part.
    j = 0
    for i in range(len(frequencyArray)):
        while(frequencyArray[i] > 0):
            # Restores our original numbers after sorting.
            L[j] = i - offset 
            frequencyArray[i] = frequencyArray[i] - 1
            j+=1
            
    return L


# Test cases
L = [1, 4, 1, 3, 2, 4, 3, 7]
print(counting_sort(L)) # Output: [1, 1, 2, 3, 3, 4, 4, 7]

L2 = [5, -4, -3, -2, 4, 7, 9]
print(counting_sort(L2)) # Output: [-4, -3, -2, 4, 5, 7, 9]

L3 = []
print(counting_sort(L3)) # Output: []

