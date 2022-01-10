# Deploying EiffelVis

## Using Docker *(recommended)*

This method combines the whole EiffelVis stack on a single image,
and only exposes a single port (`80/tcp`).

### Building the image

From the root of the git tree, run:
```sh
docker build -t eiffelvis .
```

Currently, the `Dockerfile` does not have any special arguments.

_Et voilà_, you should have an image; try running it in a temporary container:
```sh
docker run --rm -it eiffelvis --help
```

### Running the image

You can pass parameters to the backend directly at the end of the `docker run` command.
Just do make sure not to change the HTTP address and port of the backend itself,
since the frontend depends on them. Similarly, do not attempt to make the backend use SSL/TLS,
as it will not work, and would not be useful anyway since it is not directly reachable from the outside.


By accessing the port 80 (HTTP) of the running container, you will reach the frontend,
and it will transparently reach back to the backend at the `/api` path.
This makes it easy to deploy in a larger setup, as only a single routing rule needs be setup.


Security-wise, while in practice for local use HTTP is just fine, our general recommendation is to put this image behind
a reverse-proxy you own and control, itself requiring HTTPS with strong algorithms.
Since our image itself handles all traffic through an Nginx instance, you can also modify the [configuration](deployment/nginx/eiffelvis.conf)
to use SSL/TLS with your own keys and certificates.


### Connecting to RabbitMQ

Because EiffelVis is designed to tap into an existing RabbitMQ broker,
the image does not contain any, thus the backend will need to be pointed to the target broker.

By default, EiffelVis attempts to reach `localhost`, which would point inside the container,
however for simplicity & correctness sake, when running an instance of the image with **NO** arguments,
the default is overridden and `host.docker.internal` is used instead, which will look for a RabbitMQ broker on your host machine
(albeit on whichever IP interface your container is binding to, which is not the loopback interface).  \
Should the Docker host be running Linux (i.e. most of the time), you will need to manually tell `docker run` to
bind that DNS name to the host using `--add-host=host.docker.internal:host-gateway`.

Beyond that, see the [AMQP URI specification](https://www.rabbitmq.com/uri-spec.html), and the [`docker run` documentation](https://docs.docker.com/engine/reference/run/)


## Manual deployment

### Backend-specifics

For the backend, this is the vanilla experience, so refer to its `--help`.

Security-wise, do not run it as `root`, and use TLS certificates when `localhost` is not the [only] intended client.

### Frontend-specifics

Before building (with `npm run build` under `frontend/`),  you will want to edit [`frontend/.env`](frontend/.env).
There are currently two options to set:
- `EIFFELVIS_URL` is the hostname, port and path at which the backend is deployed; the prefix `@origin` can be used
  to indicate that the backend is reachable on the same host as the frontend. Alternatively, the prefix `@hostname`
  can be used to only keep the origin hostname, thus allowing the port to be different.
- `EIFFELVIS_SSL` indicates whether the backend is reachable through SSL/TLS

Once you have built the frontend, simply copy `frontend/public` wherever your webserver is expecting the built frontend tree to be.

## Questions?

Ask in our repository's [GitHub Discussions](https://github.com/ItJustWorksTM/EiffelVis/discussions)!
