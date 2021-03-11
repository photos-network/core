FROM python:3.9
LABEL "description"="Photos.network core system"
LABEL "version"="0.0.1"
LABEL "maintainer"="github.com/photos-network"

WORKDIR /app

ADD . .

RUN python3 setup.py install

CMD [ "python3", "/usr/local/bin/core" ]
