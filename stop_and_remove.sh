#!/bin/sh
if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters. Use container_id as parameter"
    exit
fi

sudo kanto-cm stop --force $1 2>/dev/null
sudo kanto-cm remove $1 2>/dev/null
