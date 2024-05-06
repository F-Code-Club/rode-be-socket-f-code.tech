import math
import os
import random
import re
import sys

#
# Complete the 'climbingLeaderboard' function below.
#
# The function is expected to return an INTEGER_ARRAY.
# The function accepts following parameters:
#  1. INTEGER_ARRAY ranked
#  2. INTEGER_ARRAY player
#

def remove_duplicate(reverse_sorted_list):
    result = []
    for element in reverse_sorted_list:
        if len(result) == 0 or element != result[-1]:
            result.append(element)
    return result

def reverse_binary_search(value, reverse_sorted_list):
    left = 0
    right = len(reverse_sorted_list)
    while (left < right):
        mid = (left + right) // 2
        element = reverse_sorted_list[mid]
        if value == element:
            return mid

        if value < element:
            left = mid + 1
        elif value > element:
            right = mid
    
    return left

def climbingLeaderboard(ranked, player):
    ranked = remove_duplicate(ranked)
    ranked.append(0)
    result = []
    for score in player:
        result.append(reverse_binary_search(score, ranked) + 1)
                
                
    return result

if __name__ == '__main__':
    ranked_count = int(input().strip())

    ranked = list(map(int, input().rstrip().split()))

    player_count = int(input().strip())

    player = list(map(int, input().rstrip().split()))

    result = climbingLeaderboard(ranked, player)

    print('\n'.join(map(str, result)))
