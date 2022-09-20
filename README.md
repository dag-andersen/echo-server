# images

## echo-server
`dagandersen/echo-server:v0.1.0`

## echo-client
`dagandersen/echo-client:v0.1.0`

# How to deploy

## Examples
**echo-server** to prod:
`$ kustomize build k8s/echo-server/overlays/prod | kubectl apply -f -`

**echo-client** to stage:
`$ kustomize build k8s/echo-client/overlays/stage | kubectl apply -f -`

# How to test

I suggest installing stern with homebrew.
`brew install stern`

Now you can:
- run `stern -A -l "system=echo-system"` to print logs from all echo servers and clients
- run `stern -A -l "app=echo-client"` to print logs from all echo clients
- run `stern -A -l "app=echo-server"` to print logs from all echo servers
  
  This should print something like 
  ```
  default echo-server-dpl-7dd4c6d49c-n7t7c echo-server client called with name: client-167
  default echo-server-dpl-7dd4c6d49c-pb5bm echo-server client called with name: client-7
  default echo-server-dpl-7dd4c6d49c-n7t7c echo-server client called with name: client-15
  default echo-server-dpl-7dd4c6d49c-pb5bm echo-server client called with name: client-216
  ```
