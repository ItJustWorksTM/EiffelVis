#!/bin/sh
# docker commamd for rabbitmq in background
docker run -d --rm --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3.10-management

# frontend start:
sh ./aws-front.sh &
sh ./aws-back.sh &