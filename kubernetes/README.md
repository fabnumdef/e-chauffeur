```
# Will create e-chauffeur namespace
kubectl create -f namespace.yml
# Will create tiller service account
kubectl create -f rbac-config.yml
# Will use tiller service account with helm
helm init --service-account tiller --upgrade
# Will create gitlab service account
kubectl create -f gitlab-admin-service-account.yml
# Setup mongodb in e-chauffeur namespace
helm install --name mongodb --namespace e-chauffeur -f helm/mongodb.yml stable/mongodb
# Setup nginx cluster
helm install --name nginx --namespace e-chauffeur --set rbac.create=true stable/nginx-ingress
# Setup cert manager
kubectl apply --namespace=e-chauffeur -f https://raw.githubusercontent.com/jetstack/cert-manager/release-0.6/deploy/manifests/00-crds.yaml
kubectl label namespace e-chauffeur certmanager.k8s.io/disable-validation="true"
helm repo add jetstack https://charts.jetstack.io
helm install --name cert-manager --namespace e-chauffeur jetstack/cert-manager
helm install --name acme-issuer --namespace e-chauffeur --set email=<email> ./helm/acme-issuer 
```

Setup gitlab runner
```
helm repo add gitlab https://charts.gitlab.io
helm install --namespace gitlab --name gitlab-runner -f helm/gitlab-runner.yml --set runnerRegistrationToken=M_ShGAyE2CXuksoPyjMx gitlab/gitlab-runner
```
