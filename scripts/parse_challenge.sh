#! /bin/bash

# Install html2md if needed
if ! hash html2md 2>/dev/null; then
  npm install -g to-markdown-cli
fi

cat days/day${1}/src/challenge.html | tr '\n' '\f' | perl -pe 's/.*?(<article.*<\/article>).*/\1/g' | tr '\f' '\n'> days/day${1}/src/challenge1.html

# Extract challenge from HTML page.
html2md -i days/day${1}/src/challenge1.html -o /tmp/challenge00.md

# Format
cat /tmp/challenge00.md | sed 's/ \././g' | sed 's/` ,/`,/g' | sed 's/` \./`./g' | sed 's/^Your puzzle answer.*$//g' > days/day${1}/src/README.md

# Delete HTML
rm days/day${1}/src/challeng*.html
