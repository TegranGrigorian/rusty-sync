import boto3

class MinioClient:
    def __init__(self, endpoint_url, access_key, secret_key):
        self.s3 = boto3.client(
            "s3",
            endpoint_url=endpoint_url,
            aws_access_key_id=access_key,
            aws_secret_access_key=secret_key,
            region_name="us-east-1"
        )
    def __test__(self):
        try:
            self.s3.list_buckets()
            print("Connection test succeeded!")
        except Exception as e:
            print("Connection test failed:", e)

    def upload_file(self, file_path, bucket, key):
        try:
            self.s3.upload_file(file_path, bucket, key)
            print("Upload succeeded!")
        except Exception as e:
            print("Upload failed:", e)

    def delete_file(self, bucket, key):
        try:
            self.s3.delete_object(Bucket=bucket, Key=key)
            print("Delete succeeded!")
        except Exception as e:
            print("Delete failed:", e)

    def list_files(self, bucket):
        try:
            response = self.s3.list_objects_v2(Bucket=bucket)
            files = [obj["Key"] for obj in response.get("Contents", [])]
            print("Files in bucket:", files)
        except Exception as e:
            print("List files failed:", e)

    def list_files_in_prefix(self, bucket, prefix):
        try:
            response = self.s3.list_objects_v2(Bucket=bucket, Prefix=prefix)
            files = [obj["Key"] for obj in response.get("Contents", [])]
            print(f"Files in bucket '{bucket}' with prefix '{prefix}':", files)
        except Exception as e:
            print("List files failed:", e)

    def upload_to_prefix(self, file_path, bucket, prefix): #hopefully a easier way to upload to a prefix
        try:
            key = f"{prefix}/{file_path.split('/')[-1]}"
            self.s3.upload_file(file_path, bucket, key)
            print("Upload to prefix succeeded!")
        except Exception as e:
            print("Upload to prefix failed:", e)