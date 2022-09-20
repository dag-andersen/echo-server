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