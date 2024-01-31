use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log_str;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::LookupMap;
use near_sdk::near_bindgen;
use near_sdk::{
    AccountId, BorshStorageKey, PublicKey, require
};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct Manifest {
    pub version: String,
    pub cid: String,
    pub content_type: String
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Attestation {
    pub pubkey: PublicKey,
    pub cid: String
}

#[derive(BorshDeserialize, BorshStorageKey, BorshSerialize, Copy, Clone)]
enum PrefixKeys {
    Package,
    Manifest,
    Attestation
}

pub type PackageName = String;
pub type Namespace = Vec<u8>;
pub type Releases = LookupMap<PackageName, Vec<Manifest>>;
pub type Attestations = Vec<Attestation>;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub packages: LookupMap<AccountId, Releases>,
    pub attestations: LookupMap<AccountId, LookupMap<Namespace, Attestations>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            packages: LookupMap::new(PrefixKeys::Package),
            attestations: LookupMap::new(PrefixKeys::Attestation)
        }
    }
}

#[near_bindgen]
impl Contract {
    /*
        pub fn get_packages(&self, account_id: String) -> Vec<String> {
            let id: AccountId = account_id.parse().unwrap();
            return self.packages.get(&id).unwrap().iter().map(|p| p.name.clone()).collect();
        }
    */

    fn generate_key(author: AccountId, package_name: String) -> Namespace {
        let key = author.as_str().to_owned() + package_name.as_str();
        return near_sdk::env::sha256(key.as_bytes());
    }

    fn safe_package_retrieval(&self, account_id: AccountId) -> Releases {
        require!(self.packages.contains_key(&account_id), "No packages found for account_id");
        return self.packages.get(&account_id).unwrap();
    }

    fn safe_attestation_retrieval(
        &mut self,
        manifests: Releases,
        attestor: AccountId,
        author: AccountId,
        package_name: String
    ) -> Attestations {
        require!(manifests.contains_key(&package_name), "Package name not found for given author");
        require!(self.attestations.contains_key(&attestor), "Attestor not found");

        let at = self.attestations.get(&attestor).unwrap();
        let hash = Self::generate_key(author, package_name);
        return at.get(&hash).unwrap();
    }

    pub fn create_manifest(
        &mut self,
        package_name: String,
        version: String,
        content_type: String,
        cid: String
    ) {
        let manifest = Manifest {
            version,
            content_type,
            cid
        };

        if !self.packages.contains_key(&near_sdk::env::signer_account_id()) {
            self.packages.insert(
                &near_sdk::env::signer_account_id(),
                &LookupMap::new(PrefixKeys::Manifest)
            );

            log_str(&format!("Creating storage..."));
            let mut manifests = self.packages.get(&near_sdk::env::signer_account_id()).unwrap();
            manifests.insert(&package_name, &mut Vec::new());
        }

        log_str(&format!("Writing manifest for {package_name}..."));
        let mut manifests = self.packages.get(&near_sdk::env::signer_account_id()).unwrap();
        let mut versions = manifests.get(&package_name)
            .unwrap();

        versions.push(manifest);
        manifests.insert(&package_name, &versions);
    }

    pub fn get_latest_manifest(
        &self,
        account_id: AccountId,
        package_name: String
    ) -> String {
        let manifests = self.safe_package_retrieval(account_id);
        require!(manifests.contains_key(&package_name), "Package name not found for given account_id");

        return manifests.get(&package_name)
            .unwrap()
            .last()
            .unwrap()
            .cid.clone();
    }

    pub fn get_manifest(
        &self,
        account_id: AccountId,
        package_name: String,
        version: String
    ) -> String {
        let manifests = self.safe_package_retrieval(account_id);
        require!(manifests.contains_key(&package_name), "Package name not found for given account_id");

        let versions = manifests.get(&package_name).unwrap();
        for v in versions {
            if v.version == version {
                return v.cid
            }
        }

        return "None".to_string();
    }

    pub fn update_manifest(
        &mut self,
        package_name: String,
        version: String,
        content_type: String,
        cid: String
    ) {
        let mut manifests = self.safe_package_retrieval(near_sdk::env::signer_account_id());
        let versions = manifests.get(&package_name).unwrap().clone();

        log_str(&format!("Updating existing manifest for {package_name} and {version}..."));

        let v = versions.clone().into_iter().map(|mut m| {
                if m.version == version {
                    m.cid = cid.clone();
                    m.content_type = content_type.clone();
                }

                return m;
            }
        ).collect::<Vec<Manifest>>();

        manifests.insert(&package_name, &v);
    }

    pub fn create_attestation(
        &mut self,
        package_name: String,
        author: AccountId,
        cid: String
    ) {
        let manifests = self.safe_package_retrieval(author.clone());
        let attest = Attestation {
            pubkey: near_sdk::env::signer_account_pk(),
            cid
        };

        let hash = Self::generate_key(author.clone(), package_name.clone());

        if !self.attestations.contains_key(&near_sdk::env::signer_account_id()) {
            self.attestations.insert(
                &near_sdk::env::signer_account_id(),
                &LookupMap::new(PrefixKeys::Attestation)
            );

            log_str(&format!("Creating attestation storage..."));
            let mut at = self.attestations.get(&near_sdk::env::signer_account_id()).unwrap();
            at.insert(&hash, &mut Vec::new());
        }

        let mut user_atts = self.safe_attestation_retrieval(
            manifests,
            near_sdk::env::signer_account_id(),
            author,
            package_name
        );

        user_atts.push(attest);
        let mut at = self.attestations.get(&near_sdk::env::signer_account_id()).unwrap();
        at.insert(&hash, &user_atts);
    }

    pub fn get_attestations(
        &mut self,
        attestor: AccountId,
        package_name: String,
        author: AccountId
    ) -> Attestations {
        let manifests = self.safe_package_retrieval(author.clone());

        return self.safe_attestation_retrieval(
            manifests,
            attestor,
            author,
            package_name
        );
    }

    pub fn get_attestation(
        &mut self,
        attestor: AccountId,
        package_name: String,
        author: AccountId,
        index: usize
    ) -> Attestation {
        let at = self.get_attestations(attestor, package_name, author);

        return at[index].clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn set_package_manifest() {
        let context = get_context(false);
        testing_env!(context.clone());
        let cid = "QmPK1s3pNYLi9ERiq3BDxKa4XosgWwFRQUydHUtz4YgpqB".to_string();
        let name = "test-package".to_string();
        let version = "0.0.1".to_string();
        let content_type = "ipfs".to_string();

        let mut contract = Contract::default();
        contract.create_manifest(
            name.clone(),
            version.clone(),
            content_type.clone(),
            cid.clone()
        );
        assert_eq!(
            contract.get_manifest(context.signer_account_id.clone(), name.clone(), version.clone()),
            cid.clone()
        );
    }

    #[test]
    fn update_existing_manifest() {
        let context = get_context(false);
        testing_env!(context.clone());
        let cid = "QmPK1s3pNYLi9ERiq3BDxKa4XosgWwFRQUydHUtz4YgpqB".to_string();
        let name = "test-package".to_string();
        let version = "0.0.1".to_string();
        let content_type = "ipfs".to_string();

        let mut contract = Contract::default();
        contract.create_manifest(
            name.clone(),
            version.clone(),
            content_type.clone(),
            cid.clone()
        );

        let new_cid = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n".to_string();

        contract.update_manifest(
            name.to_string(),
            version.to_string(),
            "hyperfiles".to_string(),
            new_cid.clone()
        );

        assert_eq!(
            contract.get_manifest(context.signer_account_id.clone(), name.clone(), version.to_string()),
            new_cid.clone()
        );
    }


    #[test]
    fn set_attestation() {
        let context = get_context(false);
        testing_env!(context.clone());
        let cid = "QmPK1s3pNYLi9ERiq3BDxKa4XosgWwFRQUydHUtz4YgpqB".to_string();
        let name = "test-package".to_string();
        let version = "0.0.1".to_string();
        let content_type = "ipfs".to_string();

        let mut contract = Contract::default();
        contract.create_manifest(
            name.clone(),
            version.clone(),
            content_type.clone(),
            cid.clone()
        );


        let attestation = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n".to_string();

        contract.create_attestation(name.clone(), context.signer_account_id.clone(), attestation.clone());

        assert_eq!(
            contract.get_attestation(context.signer_account_id.clone(), name.clone(), context.signer_account_id.clone(), 0).cid,
            attestation.clone()
        );
    }
}
