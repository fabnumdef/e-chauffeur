# Matrice de flux applicatifs

| Source | Destination | Port(s) | Protocole | Description |
|--------|-------------|---------|-----------|-------------|
|Client|Proxy|80/443*|HTTP/HTTPS*|Accès web utilisateur|
|Chauffeur|Proxy|80/443*|HTTP/HTTPS*|Accès web utilisateur|
|Régulateur|Proxy|80/443*|HTTP/HTTPS*|Accès web utilisateur|
|Administrateur applicatif|Proxy|80/443*|HTTP/HTTPS*|Accès web utilisateur|
|Administrateur technique|Cluster (control plane kubernetes)|6443|TCP (chiffré par SSL)|Connexion API administration de kubernetes|
|API|Serveur SMTP OVH|587|SMTP|Envoie des notifications mail |
|API|Prestataire SMS (Twilio)|443|HTTPS|Envoie des notifications SMS |
|API|Serveur(s) notifications web|443|HTTPS|Envoie des notifications web |
|API|Redis|6379|TCP|Synchronise les sockets|
|API|MongoDB|27017|TCP|Connexion DB|
|Proxy|Let's Encrypt|443|HTTPS|Demande le renouvellement des certificats SSL|
|Let's Encrypt|Proxy|443|HTTPS|Challenge en vue du renouvellement des certificats SSL|
|Proxy|Redirect|80|HTTP|Une routine existe pour rediriger le domaine e-chauffeur (inscrit sur QRCode) vers echauffeur (sans tiret)|
|Proxy|API|80|HTTP|L'API est accessible sur internet via le proxy|
|Proxy|App Dashboard|80|HTTP|L'App Dashboard est accessible sur internet via le proxy|
|Proxy|App User|80|HTTP|L'App User est accessible sur internet via le proxy|
|Proxy|App Driver|80|HTTP|L'App Driver est accessible sur internet via le proxy|
|App Dashboard|API|80|HTTP|L'App Dashboard interroge l'API pour le rendu coté serveur|
|App Driver|API|80|HTTP|L'App Driver interroge l'API pour le rendu coté serveur|
|App User|API|80|HTTP|L'App User interroge l'API pour le rendu coté serveur|
|Job de cleanup|MongoDB|27017|TCP|Nettoie la DB chaque jour|
|Job de backup|MongoDB|27017|TCP|Exporte la DB chaque jour|
|Job de backup|ObjectStorage OVH|443|HTTPS|Exfiltre le backup sur un système séparé|

Note* : Le proxy accepte les connexions HTTP (port 80), et renvoie une 301 vers le HTTPS.

# Matrice de flux metrics

Complément d'informations : Le projet remonte logs et metrics

| Source | Destination | Port(s) | Protocole | Description |
|--------|-------------|---------|-----------|-------------|
|Administrateur|Proxy|443|HTTP/HTTPS|Accès web|
|Proxy|Grafana|80|HTTP|L'API est accessible sur internet via le proxy|
|Grafana|Loki|3100|HTTP|Requete les logs (remontés directement via kubernetes) |
|Grafana|Prometheus|9090|HTTP|Requete les métriques|
|Prometheus|InfluxDB|8086|HTTP|TimeSeries database|
|Prometheus|API|9091|HTTP|Scrape métriques|
|Prometheus|App Dashboard|9091|HTTP|Scrape métriques|
|Prometheus|App Driver|9091|HTTP|Scrape métriques|
|Prometheus|App User|9091|HTTP|Scrape métriques|
|Prometheus|MongoDB|9091|HTTP|Scrape métriques|