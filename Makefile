# Which target to run by default (when no target is passed to make)
.DEFAULT_GOAL := help

help: ## Show help
	@echo "\nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:"
	@grep -E '^[a-zA-Z0-9_/%\-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-28s\033[0m %s\n", $$1, $$2}'

.PHONY: help setup devbox

devbox:
	k3d registry list | grep local-registry > /dev/null || k3d registry create local-registry --port 40001
	k3d cluster list | grep local-chat-cluster > /dev/null || $(MAKE) devbox/cluster
	kubectl config use-context k3d-local-chat-cluster

devbox/cluster:
	k3d cluster create local-chat-cluster --image rancher/k3s \
		--agents 1 -p "80:80@loadbalancer" \
		--registry-config $(PWD)/resources/tilt/registries.yaml \
		--registry-use local-registry:40001; \

devbox/start:
	k3d cluster start local-chat-cluster

devbox/stop:
	k3d cluster stop local-chat-cluster
