# Technologies

La motivation dans le choix des technologies pour ce projet fut de choisir des technologies fiables, limitant le périmètre de connaissances techniques, afin de rester simple, de limiter la courbe d'apprentissage pour le personnel non familier (et faciliter les recrutements).

D'autre part, certaines technologies ont été sélectionnées afin de bénéficier de leur scalabilité, permettant de donner à la solution une haute disponibilité si désirée.

## Backend

### Web API

La web API est écrite en Javascript, et fonctionne sur NodeJS >= 12. Le souhait étant de bénéficier des modules ES6 sans le surcout d'un transpiler de code. (Ainsi que toutes les features ES6+ comme le async/await, etc...)

* Koa permet de décrire le routing de l'API, décrire le comportement CORS, ou encore parser le contenu JSON entrant.
* Socket IO permet de synchroniser facilement les sockets via Redis dans le cas où on souhaite avoir plus d'une instance de l'API en production.
* ESLint garanti le format de code.
* Mocha est utilisé afin de lancer les tests unitaires
* Mongoose est utilisé comme ORM pour faciliter l'exploitation de la base de données.

### Base de données

Le système de base de données utilisé est MongoDB.
Il s'agit d'une base de données no-SQL, dite schemaless, c'est à dire qu'elle ne requiert pas la définition d'un schema en amont.

Notre utilisation de MongoDB reste basique : écriture / lecture / aggrégation.

## Frontend

### VueJS & NuxtJS

NuxtJS est utilisé pour cadrer l'organisation du code comme objectif premier.

Les trois dépots frontaux utilisent donc VueJS, ayant pour objectif 

### Storybook

Storybook est utilisé afin d'avoir une interface pour construire un design system. 

### lib-vue

Il s'agit d'un dépot interne au projet ayant pour objectif de partager du code entre les différents frontaux, afin de faciliter la maintenance du projet.

## Intégration & système

### Docker

Docker est utilisé afin de construire des conteneurs Linux pouvant être exécutés localement, en préproduction et en production. Il est utilisé pour garantir l'idempotence de déploiement entre deux conteneurs linux.

### Kubernetes

Kubernetes est utilisé en tant que système d'orchestration entre plusieurs machines physiques afin de construire un système redondé et fiable.

### Gitlab CI

Gitlab est utilisé pour son système d'intégration continue / déploiement continue afin de vérifier le code, lancer les tests unitaires, construire et héberger les images docker, et orchestrer les déploiements automatiquers sur Kubernetes.
