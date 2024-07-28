import io
from xmlrpc.client import ResponseError

from minio import Minio
from configs.settings import config


class FileHandler:
    def __init__(self, bucket_name):
        self.bucket_name = bucket_name
        try:
            self.minio_client = Minio(
                config.get('MINIO', 'url'),
                access_key=config.get('MINIO', 'access_key'),
                secret_key=config.get('MINIO', 'secret_key'),
                secure=False)
            # 判断桶是否存在
            if not self.minio_client.bucket_exists(self.bucket_name):
                # 创桶
                self.minio_client.make_bucket(self.bucket_name)

        except ResponseError as e:
            raise Exception("minio 连接失败") from e
        except Exception as e:
            raise Exception("minio 初始化失败") from e

    def put_file(self, filename, filebyte):
        filename = str(filename)
        fileio = io.BytesIO(filebyte)
        try:
            self.minio_client.put_object(self.bucket_name, filename, fileio, len(filebyte))
            return filename
        except ResponseError as e:
            raise Exception(f"minio创建文件失败 文件名:{filename}") from e

    def get_file(self, filename):
        try:
            return self.minio_client.get_object(self.bucket_name, filename).data
        except ResponseError as e:
            raise Exception(f"minio获取文件失败 文件名:{filename}") from e
