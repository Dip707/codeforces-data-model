import json
import sys

with open('data.tql', 'w') as ff:
    sys.stdout = ff
    f = open('data.json')
    data = json.load(f)
    f.close()

    for user in data['result']:
        # this user stuff is a dictionary
        handle = user['handle']
        rating = user['rating']
        max_rating = user['maxRating']
        rank = user['rank']
        friends_number = user['friendOfCount']
        if 'country' in user:
            country = user['country']
            print(f'insert $p isa coder-with-nationality, has handle "{handle}", has rating {rating}, has max-rating {max_rating}, has rank "{rank}", has friends-number {friends_number}, has country "{country}";')
        else:
            print(f'insert $p isa coder, has handle "{handle}", has rating {rating}, has max-rating {max_rating}, has rank "{rank}", has friends-number {friends_number};')

