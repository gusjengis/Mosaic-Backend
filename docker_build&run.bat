docker build -t mosaic-backend .
docker run -e DB_HOST=host.docker.internal -e DB_USER=postgres -e DB_PASS=pass -e DB_NAME=mosaic-db -d -p 8080:8080 mosaic-backend
pause