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

    print('insert')

    coder_cnt = 0
    for user in data['result']:
        # this user stuff is a dictionary
        handle = user['handle']
        rating = user['rating']
        max_rating = user['maxRating']
        rank = user['rank']
        friends_number = user['friendOfCount']
        if 'country' in user:
            country = user['country']
            print(f'$c{coder_cnt} isa coder-with-nationality, has handle "{handle}", has rating {rating}, has max-rating {max_rating}, has rank "{rank}", has friends-number {friends_number}, has country "{country}";')
        else:
            print(f'$c{coder_cnt} isa coder, has handle "{handle}", has rating {rating}, has max-rating {max_rating}, has rank "{rank}", has friends-number {friends_number};')
        coder_cnt += 1

    tags = open('cftags.txt', 'r')
    taglines = tags.readlines()
    tag_cnt = 0
    tag_dictionary = {}

    for line in taglines:
        print(f'$m{tag_cnt} isa topic, has topic-name "{line[:-1]}";')
        tag_dictionary[line[:-1]] = tag_cnt
        tag_cnt += 1

    tags.close()

    data = open('problems.json', 'r')
    problems = json.load(data)
    data.close()

    problem_data = problems['result']
    problem_cnt = 0

    for problem in problem_data['problems']:
        if problem['contestId'] < 1710:
            continue
        problem_number = str(problem['contestId']) + problem['index']
        problem_name = str(problem['name'])
        if '"' in problem_name:
            continue
        if "'" in problem_name:
            continue
        if 'rating' in problem:
            rating = problem['rating']
            print(f'$p{problem_cnt} isa problem, has problem-number "{problem_number}", has problem-name "{problem_name}", has rating {rating};')
            if 'tags' in problem:
                num_tag = 0
                for tag in problem['tags']:
                    if num_tag > 5:
                        continue
                    if str(tag) not in tag_dictionary:
                        continue
                    current_tag = tag_dictionary[str(tag)]
                    print(f'$t{problem_cnt * 7 + num_tag} (problem: $p{problem_cnt}, topic: $m{current_tag}) isa possesses-tag;')
                    num_tag += 1
            problem_cnt += 1

