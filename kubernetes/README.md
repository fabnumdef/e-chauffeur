```
# Will create e-chauffeur namespace
kubectl create -f namespace.yml
# Setup mongodb in e-chauffeur namespace
helm install --name mongodb --namespace e-chauffeur -f helm/mongodb.yml --set mongodbPassword=<password> stable/mongodb
helm install --name redis --namespace e-chauffeur stable/redis --set usePassword=false
# Setup nginx cluster
helm install --name nginx --namespace e-chauffeur --set rbac.create=true stable/nginx-ingress
# Setup cert manager
kubectl apply --namespace=e-chauffeur -f https://raw.githubusercontent.com/jetstack/cert-manager/release-0.6/deploy/manifests/00-crds.yaml
kubectl label namespace e-chauffeur certmanager.k8s.io/disable-validation="true"
helm repo add jetstack https://charts.jetstack.io
helm install --name cert-manager --namespace e-chauffeur jetstack/cert-manager
helm install --name acme-issuer --namespace e-chauffeur --set email=<email> ./helm/acme-issuer 
# Setup redirection
helm upgrade --install --namespace e-chauffeur --set ingress.issuer="letsencrypt-prod" --wait e-chauffeur-redirect ./helm/redirect
```

# Setup monitoring

```
helm repo add loki https://grafana.github.io/loki/charts
helm repo update
helm upgrade --install loki loki/loki-stack
helm install prometheus stable/prometheus-operator -f helm/prometheus-operator.yml
```

# Setup automatic backup (with helm3)
```
helm install auto-backup --namespace e-chauffeur helm/backup --set-string swift.tenant=<tenant> --set swift.url=<url to bucket> --set swift.user=<user> --set swift.password=<password> --set db.password=<db password>
```