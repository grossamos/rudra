# Contributing

## Local setup
### Install nix
Rudra uses nix for its dependency management, to get started install it and enable flakes.

#### On GNU/Linux systems or MacOS
Install nix onto your system
```bash
curl -L https://nixos.org/nix/install | sh
```

Enable flakes by adding the following line to `~/.config/nix/nix.conf` or `/etc/nix/nix.conf` (preferably the first).
```
experimental-features = nix-command flakes
```

#### On NixOS 
Enable flakes by adding the following options to your `nix.conf`
```nix
{ pkgs, ... }: {
  nix = {
    package = pkgs.nixFlakes;
    extraOptions = ''
      experimental-features = nix-command flakes
    '';
   };
}
```

### Build rudra
Rudra can then be built using nix:
```bash
nix build .
```

For development, rudra uses a nix shell.
The nix shell can be opened via:
```bash
nix develop
```

Rudras docker container can be built using docker (this will be migrated to a nix workflow in the future);
```bash
docker build -t rudra .
```

A typical testing environment would include `rudra-example` running as `app` in the rudra network.
This setup can be emulated by running:
```bash
docker network create rudra

docker run --name=app --network=rudra -d --rm rudra-example

docker run --env RUDRA_APP_BASE_URL=http://app:8080 --env RUDRA_OPENAPI_SOURCE=/swagger.yaml --volume $PWD/test/resource/swagger.yaml:/swagger.yaml -p 13750:80 --network rudra --name rudra --rm --env RUDRA_DEBUG=0 --env RUDRA_ACCOUNT_FOR_SECURITY=1 rudra
```

## Submitting Issues
- Please search for existing issues first, it is possible your issue has been reported already
- Titles of issues should have the following structure: ``<Subsystem>: Description of issue``
- Issues pertaining to ``rudra-example`` or documentation can also be reported in this repository
