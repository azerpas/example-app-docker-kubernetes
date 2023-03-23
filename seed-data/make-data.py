import json

votes = [{'file': 'postb', 'vote': 'Dogs'}, {'file': 'posta', 'vote': 'Cats'}]
for vote in votes:
    outfile = open(vote['file'], 'w')
    params = ({ 'vote': vote['vote'] })
    encoded = json.dumps(params)
    outfile.write(encoded)
    outfile.close()
