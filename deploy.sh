docker buildx build --network=host --platform linux/arm7/v7 -t sorting-vis . --load
docker save sorting-vis | bzip2 | pv | ssh pi@192.168.1.116 docker load 
