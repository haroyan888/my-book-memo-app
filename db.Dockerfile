FROM postgres:latest

COPY ./init/init.sql /docker-entrypoint-initdb.d/init.sql

RUN apt-get update && apt-get install locales-all

# Time ZoneAc
ENV TZ Asia/Tokyo

# Language
ENV LANG ja_JP.UTF-8
ENV LANGUAGE ja_JP:ja
ENV LC_ALL ja_JP.UTF-8

ENV POSTGRES_USER ${DB_USER}
ENV POSTGRES_PASSWORD ${DB_PASSWORD}

RUN psql -U ${POSTGRES_USER} -f /docker-entrypoint-initdb.d/init.sql

EXPOSE 5432