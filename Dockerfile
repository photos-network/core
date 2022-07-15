FROM python:3.10
LABEL "description"="Photos.network core system"
LABEL "version"="0.5.1"
LABEL "maintainer"="github.com/photos-network"

WORKDIR /app

ADD . .

RUN python3 setup.py install

CMD [ "python3", "/usr/local/bin/core" ]
