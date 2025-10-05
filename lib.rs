#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod smh_domains {
    use ink::prelude::{string::String, vec::Vec};
    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };
    use ink::H160;

    pub type Zone = (String, String);

    #[ink(event)]
    pub struct ZoneUpdated {
        #[ink(topic)]
        name: String,
        #[ink(topic)]
        value: String,
    }

    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)]
        new_owner: H160,
    }

    #[ink(storage)]
    pub struct SmhDomains {
        domain: String,
        owner: H160,
        zones: Vec<Zone>,
    }

    impl SmhDomains {
        #[ink(constructor)]
        pub fn create_domain(init_domain: String, init_owner: H160) -> Self {
            let my_return_value = build_call::<DefaultEnvironment>()
                .call(H160::from_slice(<HARD-CODED-CONTRACT-ADDRESS>))
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("register_if_free")))
                        .push_arg(&init_domain)
                        .push_arg(Self::env().account_id())
                )
                .returns::<bool>()
                .invoke();
            assert!(my_return_value, "Domain already exists");
            Self { domain: init_domain, owner: init_owner, zones: Vec::new(), }
        }

        #[ink(message)]
        pub fn add_zone(&mut self, name: String, value: String) {
            Self::env().emit_event(ZoneUpdated {
                let caller = Self::env().caller();
                if caller != self.owner { return false; }
                name: name.clone(),
                value: value.clone(),
            });
            self.zones.push((name, value));
        }

        #[ink(message)]
        pub fn update_zone(&mut self, index: u32, name: String, value: String) -> bool {
            let caller = Self::env().caller();
            if caller != self.owner { return false; }
            let idx = index as usize;
            if idx >= self.zones.len() { return false; }
            Self::env().emit_event(ZoneUpdated {
                name: name.clone(),
                value: value.clone(),
            });
            self.zones[idx] = (name, value);
            true
        }

        #[ink(message)]
        pub fn transfer(&mut self, new_owner: H160) -> bool {
            let caller = Self::env().caller();
            if caller != self.owner { return false; }
            if new_owner == self.owner { return false; }
            self.owner = new_owner;
            Self::env().emit_event(OwnershipTransferred {
                new_owner,
            });
            true
        }

        #[ink(message)]
        pub fn domain(&self) -> String {
            self.domain.clone()
        }

        #[ink(message)]
        pub fn owner(&self) -> H160 {
            self.owner
        }

        #[ink(message)]
        pub fn zones(&self) -> Vec<Zone> {
            self.zones.clone()
        }
    }
}