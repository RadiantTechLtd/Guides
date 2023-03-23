#!/bin/zsh

baseURL="http://127.0.0.1:3000"
# baseURL="https://0hku56rtzd.execute-api.eu-west-1.amazonaws.com"
URL=$baseURL"/plot"

mkdir output
output="output/plot.json"

echo ""
echo "URL: "$URL

curl -i -X POST $URL \
    -H 'Content-Type: application/json' \
    --output $output \
    --data-binary @- << EOF
{ 
    "real": "-0.5", 
    "imag": "0.0", 
    "scale": "2.0" 
}
EOF
