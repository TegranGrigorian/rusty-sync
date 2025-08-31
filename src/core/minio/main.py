import argparse
import os
from dotenv import load_dotenv
from src.minio_util import MinioClient

if __name__ == "__main__":
    env_path = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../../.env"))
    load_dotenv(env_path)

    parser = argparse.ArgumentParser(description="MinIO Client")
    parser.add_argument("--upload", nargs=3, metavar=("FILE_PATH", "BUCKET", "KEY"), help="Upload a file to MinIO")
    parser.add_argument("--download", nargs=3, metavar=("BUCKET", "KEY", "LOCAL_PATH"), help="Download a file from MinIO")
    parser.add_argument("--delete", nargs=2, metavar=("BUCKET", "KEY"), help="Delete a file from MinIO")
    parser.add_argument("--list", metavar="BUCKET", help="List files in a MinIO bucket")
    parser.add_argument("--list-buckets", action="store_true", help="List all buckets")
    parser.add_argument("--create-bucket", metavar="BUCKET", help="Create a new bucket")
    parser.add_argument("--check-bucket", metavar="BUCKET", help="Check if bucket exists")
    args = parser.parse_args()

    Client = MinioClient(
        endpoint_url=os.getenv("MINIO_ENDPOINT_URL"),
        access_key=os.getenv("MINIO_ACCESS_KEY"),
        secret_key=os.getenv("MINIO_SECRET_KEY")
    )
    Client.__test__()

    if args.upload:
        Client.upload_file(args.upload[0], args.upload[1], args.upload[2])
    if args.download:
        Client.download_file(args.download[0], args.download[1], args.download[2])
    if args.delete:
        Client.delete_file(args.delete[0], args.delete[1])
    if args.list:
        Client.list_files(args.list)
    if args.list_buckets:
        Client.list_buckets()
    if args.create_bucket:
        Client.create_bucket(args.create_bucket)
    if args.check_bucket:
        Client.check_bucket_exists(args.check_bucket)
    # Client.upload_file("/home/tegran-grigorian/Documents/Projects/rusty-sync/hi.mp3", "rusty-sync", "hi.mp3")