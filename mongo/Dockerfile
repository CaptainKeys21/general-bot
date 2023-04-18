FROM mongo:6.0

COPY keyFile.txt /etc/mongo/
RUN chown mongodb:root /etc/mongo/keyFile.txt

COPY mongod.conf /etc/

COPY init-mongo.sh /docker-entrypoint-initdb.d/
RUN chown mongodb:root /docker-entrypoint-initdb.d/init-mongo.sh

EXPOSE 27017

CMD [ "mongod", "--config", "/etc/mongod.conf"]