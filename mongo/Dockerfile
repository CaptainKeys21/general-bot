FROM mongo:6.0

COPY keyFile.txt /etc/mongo/
RUN chown mongodb:root /etc/mongo/keyFile.txt
COPY mongod.conf /etc/

COPY init-mongo.sh /docker-entrypoint-initdb.d/

EXPOSE 27017

CMD [ "mongod", "--config", "/etc/mongod.conf"]