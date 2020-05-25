# Liste des droits eChauffeur

## Note
Note à jour de la version 1.4.6
- Tous Les droits sont systématiquement hérités aux échelons supérieurs

### Liste des roles
- __Anonyme__ : Visiteur sans compte
- __Utilisateur__ : Utilisateur inscrit sans droit particulier
- __Chauffeur__ : Utilisateur inscrit avec un droit chauffeur attribué par campus
- __Régulateur__ : Utilisateur inscrit avec un droit régulateur attribué par campus
- __Administrateur local__ : Utilisateur inscrit avec un droit administrateur local attribué par campus
- __Super administrateur__ : A tous les droits

## Matrice des droits

| Peut... | Nomenclature | Anonyme | Utilisateur | Chauffeur | Régulateur | Adminitrateur local | Super administrateur |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Se connecter | `CAN_LOGIN` | ✔️ | __x__ | __x__ | __x__ | __x__ | __x__ |
| Accéder à une course avec un token | `CAN_GET_RIDE_WITH_TOKEN` | ✔️ | __x__ | __x__ | __x__ | __x__ | __x__ |
| Accéder à la position d’une course | `CAN_GET_RIDE_POSITION` | ✔️ | __x__ | __x__ | __x__ | __x__ | __x__ |
| Accéder à la liste basique des bases  | `CAN_LIST_CAMPUS_BASIC` |  ✔️ | __x__ | __x__ | __x__ | __x__ | __x__ |
| Envoyer un token de création | `CAN_SEND_CREATION_TOKEN` | ✔️ | __x__ | __x__ | __x__ | __x__ | __x__ |
| Envoyer une appréciation | `CAN_CREATE_RATING` | ✔️ | __x__ | __x__ | __x__ | __x__ | __x__ |
| Accéder à ses données en course | `CAN_ACCESS_OWN_DATA_ON_RIDE` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Editer son nom | `CAN_EDIT_SELF_USER_NAME` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Editer son mot de passe | `CAN_EDIT_SELF_USER_PASSWORD` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Editer ses données sensibles | `CAN_EDIT_SELF_USER_SENSITIVE_DATA` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Accéder aux informations basiques d'une base | `CAN_GET_CAMPUS_BASIC` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Demander une course | `CAN_REQUEST_RIDE` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Accéder à sa course | `CAN_GET_OWNED_RIDE` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Editer sa course | `CAN_EDIT_OWNED_RIDE` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Editer le statut de sa course | `CAN_EDIT_OWNED_RIDE_STATUS` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Lister ses courses | `CAN_LIST_SELF_RIDE` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Supprimer sa course | `CAN_DELETE_SELF_RIDE` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Accéder aux lieux | `CAN_GET_POI` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Lister les lieux | `CAN_LIST_POI` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Editer son compte sans droits supérieurs | `CAN_EDIT_USER_WITHOUT_UPPER_RIGHTS` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Supprimer son compte | `CAN_REMOVE_SELF_USER` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Lister les navettes | `CAN_LIST_SHUTTLE` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Liste les trajets de navettes | `CAN_LIST_SHUTTLE_FACTORIES` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Editer une navette | `CAN_EDIT_SHUTTLE` | ✔️ | ✔️ | __x__ | __x__ | __x__ | __x__ |
| Lister les bases | `CAN_LIST_CAMPUS` | ✔️ | ✔️ | ✔️ | __x__ | __x__ | __x__ |
| Accéder à une base | `CAN_GET_CAMPUS` | ✔️ | ✔️ | ✔️ | __x__ | __x__ | __x__ |
| Lister un modèle de véhicule | `CAN_LIST_CAR_MODEL` | ✔️ | ✔️ | ✔️ | __x__ | __x__ | __x__ |
| Accéder à un modèle de véhicule | `CAN_GET_CAR_MODEL` | ✔️ | ✔️ | ✔️ | __x__ | __x__ | __x__ |
| Lister les événements utilisateurs | `CAN_LIST_USER_EVENT` | ✔️ | ✔️ | ✔️ | __x__ | __x__ | __x__ |
| Editer les événements utilisateurs | `CAN_GET_USER_EVENT` | ✔️ | ✔️ | ✔️ | __x__ | __x__ | __x__ |
| Lister les catégories | `CAN_LIST_CATEGORY` | ✔️ | ✔️ | ✔️ | __x__ | __x__ | __x__ |
| Editer le statut d'une course | `CAN_EDIT_RIDE_STATUS` | ✔️ | ✔️ | ✔️ | __x__ | __x__ | __x__ |
| Lister les courses chauffeurs sur un campus | `CAN_LIST_CAMPUS_DRIVER_RIDE` | ✔️ | ✔️ | ✔️ | __x__ | __x__ | __x__ |
| Accéder aux données personnelles pendant la course | `CAN_ACCESS_PERSONNAL_DATA_ON_RIDE` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Lister les utilisateurs | `CAN_LIST_USER` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Accéder à un utilisateur | `CAN_GET_USER` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Editer un utilisateur | `CAN_EDIT_USER` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Créer un utilisateur | `CAN_CREATE_USER` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Lister les véhicules | `CAN_LIST_CAR` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Editer un véhicule  | `CAN_EDIT_CAR` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Créer un véhicule | `CAN_CREATE_CAR` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Accéder à un véhicule | `CAN_GET_CAR` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Supprimer un véhicule | `CAN_REMOVE_CAR` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Accéder aux statistiques d'une base | `CAN_GET_CAMPUS_STATS` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Lister les véhicules d'un base | `CAN_LIST_CAMPUS_CAR` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Lister les chauffeurs d'une base | `CAN_LIST_CAMPUS_DRIVER` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Accéder à un chauffeur d'une base | `CAN_GET_CAMPUS_DRIVER` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Créer un chauffeur dans une base | `CAN_CREATE_CAMPUS_DRIVER` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Editer un chauffeur d'une base | `CAN_EDIT_CAMPUS_DRIVER` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Supprimer un chauffeur d'une base | `CAN_REMOVE_CAMPUS_DRIVER` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Lister les événements liés aux véhicules | `CAN_LIST_CAR_EVENT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Accéder à un événement lié aux véhicules | `CAN_GET_CAR_EVENT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Créer un événement lié aux véhicules | `CAN_CREATE_CAR_EVENT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Editer un événement lié aux véhicules | `CAN_EDIT_CAR_EVENT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Supprimer un événement lié aux véhicules | `CAN_REMOVE_CAR_EVENT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Créer un événement lié aux utilisateur | `CAN_CREATE_USER_EVENT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Editer un événement lié aux utilisateur | `CAN_EDIT_USER_EVENT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Supprimer un événement lié aux utilisateur | `CAN_REMOVE_USER_EVENT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Lister les modèles de téléphone | `CAN_LIST_PHONE_MODEL` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Accéder à un modèle de téléphone | `CAN_GET_PHONE_MODEL` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Envoyer un feedback | `CAN_SEND_FEEDBACK` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Lister les courses | `CAN_LIST_RIDE` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Créer une course | `CAN_CREATE_RIDE` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Editer une course | `CAN_EDIT_RIDE` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Accéder à une course | `CAN_GET_RIDE` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Révoquer le rôle chauffeur localement | `CAN_REVOKE_ROLE_LOCAL_DRIVER` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Ajouter le rôle chauffeur localement| `CAN_ADD_ROLE_LOCAL_DRIVER` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Lister les téléphones d'une base | `CAN_LIST_PHONE_LOCAL` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Accéder à un téléphone d'une base | `CAN_GET_PHONE_LOCAL` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Créer un créneau horaire | `CAN_CREATE_TIME_SLOT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Lister les créneaux horaires | `CAN_LIST_TIME_SLOT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Editer un créneau horaire | `CAN_EDIT_TIME_SLOT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Supprimer un créneau horaire | `CAN_REMOVE_TIME_SLOT` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Créer un trajet de navette | `CAN_CREATE_SHUTTLE_FACTORIES` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Accéder à un trajet de navette | `CAN_GET_SHUTTLE_FACTORIES` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Editer un trajet de navette | `CAN_UPDATE_SHUTTLE_FACTORIES` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Supprimer un trajet de navette | `CAN_DELETE_SHUTTLE_FACTORIES` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Supprimer une navette | `CAN_DELETE_SHUTTLE` | ✔️ | ✔️ | ✔️ | ✔️ | __x__ | __x__ |
| Editer un lieu d'une base | `CAN_EDIT_POI_LOCAL` |  ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Créer un lieu dans une base | `CAN_CREATE_POI_LOCAL` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Supprimer un lieu dans une base | `CAN_REMOVE_POI_LOCAL` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Créer un téléphone dans une base | `CAN_CREATE_PHONE_LOCAL` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Editer un téléphone dans une base | `CAN_EDIT_PHONE_LOCAL` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Supprimer un téléphone dans une base | `CAN_REMOVE_PHONE_LOCAL` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Editer sa base | `CAN_EDIT_SELF_CAMPUS` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Lister les utilisateurs de la base  | `CAN_LIST_CAMPUS_USER` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Accéder à un utilisateur de la base | `CAN_GET_CAMPUS_USER` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Créer un utilisateur de la base | `CAN_CREATE_CAMPUS_USER` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Editer un utilisateur de la base | `CAN_EDIT_CAMPUS_USER` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Supprimer un utilisateur de la base | `CAN_REMOVE_CAMPUS_USER` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Révoquer le rôle régulateur dans la base | `CAN_REVOKE_ROLE_LOCAL_REGULATOR` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Ajouter le rôle régulateur dans la base | `CAN_ADD_ROLE_LOCAL_REGULATOR` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Editer les informations sensibles d'un utilisateur | `CAN_EDIT_USER_SENSITIVE_DATA` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | __x__ |
| Créer un modèle de véhicule | `CAN_CREATE_CAR_MODEL` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Editer un modèle de véhicule | `CAN_EDIT_CAR_MODEL` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Supprimer un modèle de véhicule | `CAN_REMOVE_CAR_MODEL` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Supprimer un utilisateur | `CAN_REMOVE_USER` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Créer une base | `CAN_CREATE_CAMPUS` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Editer une base | `CAN_EDIT_CAMPUS` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Supprimer une base | `CAN_REMOVE_CAMPUS` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Editer une catégorie | `CAN_EDIT_CATEGORY` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Créer une catégorie | `CAN_CREATE_CATEGORY` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Accéder à une catégorie | `CAN_GET_CATEGORY` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Supprimer une catégorie | `CAN_REMOVE_CATEGORY` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Accéder à l'historique des positions | `CAN_GET_POSITION_HISTORY` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Créer un modèle de téléphone | `CAN_CREATE_PHONE_MODEL` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Editer un modèle de téléphone | `CAN_EDIT_PHONE_MODEL` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Supprimer un modèle de téléphone | `CAN_REMOVE_PHONE_MODEL` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Révoquer le rôle super administrateur | `CAN_REVOKE_ROLE_SUPERADMIN` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Ajouter le rôle super administrateur | `CAN_ADD_ROLE_SUPERADMIN` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Révoquer le rôle administrateur local | `CAN_REVOKE_ROLE_ADMIN` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Ajouter le rôle administrateur local | `CAN_ADD_ROLE_ADMIN` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Ajouter le rôle régulateur | `CAN_REVOKE_ROLE_REGULATOR` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Révoquer le rôle régulateur | `CAN_ADD_ROLE_REGULATOR` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Révoquer le rôle chauffeur | `CAN_REVOKE_ROLE_DRIVER` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Ajouter le rôle chauffeur | `CAN_ADD_ROLE_DRIVER` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Lister les bases | `CAN_LIST_ALL_CAMPUSES` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Lister les appréciations | `CAN_LIST_RATING` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Accéder à une appréciation | `CAN_GET_RATING` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |
| Accéder aux statistiques | `CAN_GET_STATS` | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |