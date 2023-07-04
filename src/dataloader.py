import json
import sys
import os

path = '/Users/yashb/Desktop/Vaticle/onboarding-project-cf/src'

os.chdir(path)

with open('data.tql', 'w') as ff:
    sys.stdout = ff
    f = open('data.json', 'r')
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

    tags = open('cftags.txt', 'r')
    taglines = tags.readlines()

    for line in taglines:
        print(f'insert $p isa topic, has topic-name "{line[:-1]}";')

    tags.close()

    data = open('problems.json', 'r')
    problems = json.load(data)
    data.close()

    problem_data = problems['result']

    for problem in problem_data['problems']:
        problem_number = str(problem['contestId']) + problem['index']
        problem_name = str(problem['name'])
        if '"' in problem_name:
            continue
        if "'" in problem_name:
            continue
        if 'rating' in problem:
            rating = problem['rating']
            print(f'insert $p isa problem, has problem-number "{problem_number}", has problem-name "{problem_name}", has rating {rating};')
            # if 'tags' in problem:
            #     for tag in problem['tags']:
            #         print(f'match $p isa problem, has problem-number "{problem_number}";')
            #         print(f'match $q isa topic, has topic-name "{tag}";')
            #         print(f'(problem: $p, topic: $q) isa possesses-tag;')

