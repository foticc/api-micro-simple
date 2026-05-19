```bash
sea-orm-cli generate entity -u postgres://admin:123456@192.168.160.138:5432/ng-antd-admin-db -o src/entity
```

```bash
docker compose up -d
```

docker cp emqx:/opt/emqx/etc /home/to_del/emqx
docker cp emqx:/opt/emqx/lib /home/to_del/emqx
docker cp emqx:/opt/emqx/data /home/to_del/emqx
docker cp emqx:/opt/emqx/log /home/to_del/emqx


[root@localhost emqx]# ls
docker-compose.yml
[root@localhost emqx]# vim docker-compose.yml

name: emqx
services:
emqx:
container_name: emqx
ports:
- 1883:1883
- 8083:8083
- 8084:8084
- 8883:8883
- 18083:18083
volumes:
- /docker/emqx/data:/opt/emqx/data
- /docker/emqx/etc:/opt/emqx/etc
- /docker/emqx/log:/opt/emqx/log
image: emqx/emqx:5.1.4
