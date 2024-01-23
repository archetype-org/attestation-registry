use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log_str;
use near_sdk::collections::LookupMap;
use near_sdk::near_bindgen;
use near_sdk::{
    AccountId, BorshStorageKey
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Manifest {
    pub version: String,
    pub cid: String,
    pub content_type: String
}

#[derive(BorshDeserialize, BorshStorageKey, BorshSerialize, Copy, Clone)]
enum PrefixKeys {
    Package,
    Content,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub packages: LookupMap<AccountId, LookupMap<String, Vec<Manifest>>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self { packages: LookupMap::new(PrefixKeys::Package), }
    }
}

#[near_bindgen]
impl Contract {

    /*
    pub fn get_packages(&self, account_id: String) -> Vec<String> {
        let id: AccountId = account_id.parse().unwrap();

        return self.packages.get(&id).unwrap().iter().map(|p| p.name.clone()).collect();
    }*/ 

    pub fn create_manifest(&mut self, package_name: String, version: String, content_type: String, cid: String) {
        let manifest = Manifest {
            version,
            content_type,
            cid
        };

        if !self.packages.contains_key(&near_sdk::env::signer_account_id()) {
            self.packages.insert(&near_sdk::env::signer_account_id(), &LookupMap::new(b"m"));
            log_str(&format!("Creating storage..."));
        }

        log_str(&format!("Writing manifest for {package_name}..."));
        let manifests = self.packages.get(&near_sdk::env::signer_account_id()).unwrap();
        manifests.get(&package_name).unwrap().push(manifest);
    }

    pub fn get_latest_manifest(&self, account_id: AccountId, package_name: String) -> String {
        let manifests = self.packages.get(&account_id).unwrap();
        return manifests.get(&package_name).unwrap().last().unwrap().cid.clone();
    }

    pub fn get_manifest(&self, account_id: AccountId, package_name: String, version: String) -> String {
        let manifests = self.packages.get(&account_id).unwrap();
        let versions = manifests.get(&package_name).unwrap();

        for v in versions {
            if v.version == version {
                return v.cid
            }
        }

        return "None".to_string();
    }

    pub fn update_manifest(&mut self, package_name: String, version: String, content_type: String, cid: String) {
        let manifests = self.packages.get(&near_sdk::env::signer_account_id()).unwrap();
        log_str(&format!("Updating existing manifest for {package_name} and {version}..."));
        for mut m in manifests.get(&package_name).unwrap() {
            if m.version == version {
                m.cid = cid.clone();
                m.content_type = content_type.clone();
            }
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getters() {
    }

    #[test]
    fn setters() {
    }
}
*/
