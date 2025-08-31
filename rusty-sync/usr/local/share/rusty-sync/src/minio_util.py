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

    def download_file(self, bucket, key, local_path):
        try:
            # Ensure directory exists
            import os
            os.makedirs(os.path.dirname(local_path), exist_ok=True)
            
            self.s3.download_file(bucket, key, local_path)
            print("Download succeeded!")
        except Exception as e:
            print("Download failed:", e)

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

    def get_file_timestamp(self, bucket, key):
        try:
            response = self.s3.head_object(Bucket=bucket, Key=key)
            return response["LastModified"]
        except Exception as e:
            print("Get file timestamp failed:", e)
            return None
    def get_file_size(self, bucket, key):
        try:
            response = self.s3.head_object(Bucket=bucket, Key=key)
            return response["ContentLength"]
        except Exception as e:
            print("Get file size failed:", e)
            return None

    def create_bucket(self, bucket_name):
        try:
            self.s3.create_bucket(Bucket=bucket_name)
            print(f"Bucket '{bucket_name}' created successfully!")
        except Exception as e:
            if "BucketAlreadyExists" in str(e) or "BucketAlreadyOwnedByYou" in str(e):
                print(f"Bucket '{bucket_name}' already exists.")
            else:
                print(f"Create bucket failed: {e}")

    def check_bucket_exists(self, bucket_name):
        try:
            self.s3.head_bucket(Bucket=bucket_name)
            print(f"Bucket '{bucket_name}' exists.")
            return True
        except Exception as e:
            print(f"Bucket '{bucket_name}' does not exist: {e}")
            return False

    def list_buckets(self):
        try:
            response = self.s3.list_buckets()
            buckets = [bucket["Name"] for bucket in response.get("Buckets", [])]
            print("Available buckets:", buckets)
            return buckets
        except Exception as e:
            print("List buckets failed:", e)
            return [] 