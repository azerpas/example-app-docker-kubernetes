# this creates urlencode-friendly files without EOL
import urllib.parse
import json

outfile = open('postb', 'w')
params = ({ 'vote': 'Dogs' })
encoded = json.dumps(params)
outfile.write(encoded)
outfile.close()
outfile = open('posta', 'w')
params = ({ 'vote': 'Cats' })
encoded = json.dumps(params)
outfile.write(encoded)
outfile.close()
