docker run -d --privileged=true -v /root/data:/home/work monitor:app

将 /root/data目录映射到 /home/work/目录下  目前config/monitor_config.json 路径是死的

docker build -t monitor:app .
