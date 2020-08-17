FROM ubuntu
USER root
COPY . /home/hyt/ 
ENV PATH=/home/hyt:$PATH LD_LIBRARY_PATH=/home/hyt/lib
WORKDIR /home/hyt
CMD ["app"]
