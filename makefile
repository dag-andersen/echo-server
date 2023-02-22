start:
	kind create cluster --name test

stop:
	kind delete cluster --name test

update:
	kubectl kustomize k8s/echo-client | kubectl apply -f -
	kubectl kustomize k8s/echo-server | kubectl apply -f -