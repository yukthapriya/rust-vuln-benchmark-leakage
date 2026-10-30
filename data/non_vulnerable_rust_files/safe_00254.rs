// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0138 (FIXED)
// Crate: mz-avro
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: MaterializeInc/materialize
// Commit: 265f7af
// File: src/mz-debug/src/k8s_dumper.rs


};
 use k8s_openapi::api::apps::v1::{DaemonSet, Deployment, ReplicaSet, StatefulSet};
 use k8s_openapi::api::core::v1::{
    ConfigMap, Event, Node, PersistentVolume, PersistentVolumeClaim, Pod, Service, ServiceAccount,
 };
 use k8s_openapi::api::networking::v1::NetworkPolicy;
 use k8s_openapi::api::rbac::v1::{Role, RoleBinding};
     api: Api<K>,
     namespace: Option<String>,
     resource_type: String,
 }
 
 impl<'n, K> K8sResourceDumper<'n, K>
 where
     K: kube::Resource<DynamicType = ()> + Clone + Debug + Serialize + DeserializeOwned,
 {
    fn cluster(context: &'n Context, client: Client) -> Self {
         Self {
             context,
             api: Api::<K>::all(client),
             namespace: None,
             resource_type: K::plural(&()).into_owned(),
         }
     }
 
    fn namespaced(context: &'n Context, client: Client, namespace: String) -> Self
     where
         K: kube::Resource<Scope = NamespaceResourceScope>,
     {
             api: Api::<K>::namespaced(client, namespace.as_str()),
             namespace: Some(namespace),
             resource_type: K::plural(&()).into_owned(),
         }
     }
 
             ));
             let mut file = File::create(&file_name)?;
 
            serde_yaml::to_writer(&mut file, &item)?;
 
             info!("Exported {}", file_name.display());
         }
     k8s_additional_namespaces: Option<Vec<String>>,
     /// The kubernetes context to use.
     k8s_context: Option<String>,
 }
 
 impl<'n> K8sDumper<'n> {
         k8s_namespace: String,
         k8s_additional_namespaces: Option<Vec<String>>,
         k8s_context: Option<String>,
     ) -> Self {
         Self {
             context,
             client,
             k8s_namespace,
             k8s_additional_namespaces,
             k8s_context,
         }
     }
 
 
     /// Write cluster-level k8s resources to a yaml file per resource.
     async fn dump_cluster_resources(&self) {
        K8sResourceDumper::<Node>::cluster(self.context, self.client.clone())
            .dump()
            .await;
 
        K8sResourceDumper::<StorageClass>::cluster(self.context, self.client.clone())
            .dump()
            .await;
 
        K8sResourceDumper::<PersistentVolume>::cluster(self.context, self.client.clone())
            .dump()
            .await;
 
         K8sResourceDumper::<MutatingWebhookConfiguration>::cluster(
             self.context,
             self.client.clone(),
         )
         .dump()
         .await;
 
         K8sResourceDumper::<ValidatingWebhookConfiguration>::cluster(
             self.context,
             self.client.clone(),
         )
         .dump()
         .await;
        K8sResourceDumper::<DaemonSet>::cluster(self.context, self.client.clone())
            .dump()
            .await;
        K8sResourceDumper::<CustomResourceDefinition>::cluster(self.context, self.client.clone())
            .dump()
            .await;
     }
 
     async fn _dump_k8s_pod_logs(&self, namespace: &String) -> Result<(), anyhow::Error> {
 
     /// Write namespace-level k8s resources to a yaml file per resource.
     pub async fn dump_namespaced_resources(&self, namespace: String) {
        K8sResourceDumper::<Pod>::namespaced(self.context, self.client.clone(), namespace.clone())
            .dump()
            .await;
         K8sResourceDumper::<Service>::namespaced(
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
         K8sResourceDumper::<Deployment>::namespaced(
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
         K8sResourceDumper::<StatefulSet>::namespaced(
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
         K8sResourceDumper::<ReplicaSet>::namespaced(
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
         K8sResourceDumper::<NetworkPolicy>::namespaced(
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
         K8sResourceDumper::<Event>::namespaced(
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
         K8sResourceDumper::<Materialize>::namespaced(
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
        K8sResourceDumper::<Role>::namespaced(self.context, self.client.clone(), namespace.clone())
            .dump()
            .await;
         K8sResourceDumper::<RoleBinding>::namespaced(
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
         K8sResourceDumper::<ConfigMap>::namespaced(
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
         K8sResourceDumper::<PersistentVolumeClaim>::namespaced(
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
         K8sResourceDumper::<ServiceAccount>::namespaced(
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
             self.context,
             self.client.clone(),
             namespace.clone(),
         )
         .dump()
         .await;
             futs.push(Box::pin(
                 self.dump_kubectl_describe::<ConfigMap>(Some(namespace)),
             ));
             futs.push(Box::pin(
                 self.dump_kubectl_describe::<PersistentVolumeClaim>(Some(namespace)),
             ));