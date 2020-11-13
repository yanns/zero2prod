# Zero2Prod

Learning rust with https://www.zero2prod.com/

https://github.com/LukeMathWalker/zero-to-production

## local development

```
./scripts/init_db.sh
```

or
```
SKIP_DOCKER=true ./scripts/init_db.sh
```


## curl examples

```
curl --request POST --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' 127.0.0.1:8000/subscriptions --verbose
```


## docker image

Prepare docker image:

```
docker build --tag zero2prod --file Dockerfile .
```

Run in docker container:

```
docker run -p 8000:8000 zero2prod
```
