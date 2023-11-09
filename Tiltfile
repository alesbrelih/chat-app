default_registry('localhost:5005')

k8s_yaml('./resources/tilt/namespaces.yaml')

docker_build(
  'backend',
  context='./',
  dockerfile='./resources/docker/backend/Dockerfile',
  only=['./code/backend/chat-app'],
  ignore=['./code/backend/chat-app/target']
)

k8s_yaml(
  helm(
    './resources/charts/backend',
    namespace='app',
    values=['./resources/tilt/backend-overrides.yaml'],
  )
)
