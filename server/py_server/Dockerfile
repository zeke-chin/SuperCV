#FROM python:3.10
FROM registry.cn-shanghai.aliyuncs.com/zekechin/python:3.10

RUN apt update && apt install libmagic1 -y

ADD ./requirements.txt /workspace/requirements.txt
WORKDIR /workspace

RUN pip install --upgrade pip -i https://mirror.baidu.com/pypi/simple && \
    pip install -r /workspace/requirements.txt

COPY . /workspace

CMD ["sh", "-c", "sleep 5 && python run.py"]

