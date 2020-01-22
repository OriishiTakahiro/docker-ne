FROM python:3.7.4-stretch

RUN apt update

RUN pip install \
    networkx \
    numpy \
    matplotlib \
    pyyaml 
RUN mkdir /root/artifacts

COPY ./src /root/src


WORKDIR /root/src

ENTRYPOINT ["python", "main.py"]
