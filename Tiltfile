# Load the helm_remote extension so we can create services from remote Helm charts
load('ext://helm_remote', 'helm_remote')
# Load the kim_build extension so we can build images using kim
load('ext://kim', 'kim_build')

# PSQL from standard Helm chart
helm_remote('postgresql',
           repo_name='bitnami',
           repo_url='https://charts.bitnami.com/bitnami',
           values=['pipeline/value/psql.yaml'])

docker_build('scs-core', '.')


# Tell Tilt how to configure and deploy our service with Helm
# Apply values overrides from pipeline/values/common.yaml
k8s_yaml(
    helm(
        './pipeline/helm-charts',
        values=['pipeline/helm-charts/values.yaml']
    )
)

# Tell Tilt how to build our service with Kim
kim_build(
    'docker.io',
    '.',
    extra_flags=['-f', '.dockerfile']
)