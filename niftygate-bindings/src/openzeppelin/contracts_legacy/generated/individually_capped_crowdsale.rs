#[allow(dead_code)]
pub mod individually_capped_crowdsale {
  # [rustfmt :: skip] use ethcontract as ethcontract ;
  #[doc = "Generated by `ethcontract`"]
  #[derive(Clone)]
  pub struct Contract {
    methods: Methods,
  }
  impl Contract {
    #[doc = r" Retrieves the raw contract instance used to generate the type safe"]
    #[doc = r" API for this contract."]
    pub fn raw_contract() -> &'static self::ethcontract::Contract {
      use self::ethcontract::common::artifact::truffle::TruffleLoader;
      use self::ethcontract::private::lazy_static;
      use self::ethcontract::Contract;
      lazy_static! {
        pub static ref CONTRACT: Contract = {
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"IndividuallyCappedCrowdsale\",\"abi\":[{\"type\":\"function\",\"name\":\"renounceCapper\",\"inputs\":[],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"weiRaised\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"buyTokens\",\"inputs\":[{\"name\":\"beneficiary\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"isCapper\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"addCapper\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"wallet\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setCap\",\"inputs\":[{\"name\":\"beneficiary\",\"type\":\"address\"},{\"name\":\"cap\",\"type\":\"uint256\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"token\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getContribution\",\"inputs\":[{\"name\":\"beneficiary\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getCap\",\"inputs\":[{\"name\":\"beneficiary\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"rate\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"TokensPurchased\",\"inputs\":[{\"name\":\"purchaser\",\"type\":\"address\",\"indexed\":true},{\"name\":\"beneficiary\",\"type\":\"address\",\"indexed\":true},{\"name\":\"value\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"amount\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"CapperAdded\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"CapperRemoved\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true}],\"anonymous\":false},{\"type\":\"fallback\"}],\"bytecode\":\"\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
          contract
        };
      }
      &CONTRACT
    }
    #[doc = r" Creates a new contract instance with the specified `web3`"]
    #[doc = r" provider at the given `Address`."]
    #[doc = r""]
    #[doc = r" Note that this does not verify that a contract with a matching"]
    #[doc = r" `Abi` is actually deployed at the given address."]
    pub fn at<F, B, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      address: self::ethcontract::Address,
    ) -> Self
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + 'static,
      B: std::future::Future<
          Output = Result<
            Vec<Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>>,
            self::ethcontract::web3::Error,
          >,
        > + Send
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F>
        + self::ethcontract::web3::BatchTransport<Batch = B>
        + Send
        + Sync
        + 'static,
    {
      Contract::with_deployment_info(web3, address, None)
    }
    #[doc = r" Creates a new contract instance with the specified `web3` provider with"]
    #[doc = r" the given `Abi` at the given `Address` and an optional transaction hash."]
    #[doc = r" This hash is used to retrieve contract related information such as the"]
    #[doc = r" creation block (which is useful for fetching all historic events)."]
    #[doc = r""]
    #[doc = r" Note that this does not verify that a contract with a matching `Abi` is"]
    #[doc = r" actually deployed at the given address nor that the transaction hash,"]
    #[doc = r" when provided, is actually for this contract deployment."]
    pub fn with_deployment_info<F, B, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      address: self::ethcontract::Address,
      deployment_information: Option<ethcontract::common::DeploymentInformation>,
    ) -> Self
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + 'static,
      B: std::future::Future<
          Output = Result<
            Vec<Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>>,
            self::ethcontract::web3::Error,
          >,
        > + Send
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F>
        + self::ethcontract::web3::BatchTransport<Batch = B>
        + Send
        + Sync
        + 'static,
    {
      use self::ethcontract::transport::DynTransport;
      use self::ethcontract::web3::api::Web3;
      use self::ethcontract::Instance;
      let transport = DynTransport::new(web3.transport().clone());
      let web3 = Web3::new(transport);
      let abi = Self::raw_contract().abi.clone();
      let instance = Instance::with_deployment_info(web3, abi, address, deployment_information);
      Contract::from_raw(instance)
    }
    #[doc = r" Creates a contract from a raw instance."]
    fn from_raw(instance: self::ethcontract::dyns::DynInstance) -> Self {
      let methods = Methods { instance };
      Contract { methods }
    }
    #[doc = r" Returns the contract address being used by this instance."]
    pub fn address(&self) -> self::ethcontract::Address {
      self.raw_instance().address()
    }
    #[doc = r" Returns the deployment information of the contract"]
    #[doc = r" if it is known, `None` otherwise."]
    pub fn deployment_information(&self) -> Option<ethcontract::common::DeploymentInformation> {
      self.raw_instance().deployment_information()
    }
    #[doc = r" Returns a reference to the default method options used by this"]
    #[doc = r" contract."]
    pub fn defaults(&self) -> &self::ethcontract::contract::MethodDefaults {
      &self.raw_instance().defaults
    }
    #[doc = r" Returns a mutable reference to the default method options used"]
    #[doc = r" by this contract."]
    pub fn defaults_mut(&mut self) -> &mut self::ethcontract::contract::MethodDefaults {
      &mut self.raw_instance_mut().defaults
    }
    #[doc = r" Returns a reference to the raw runtime instance used by this"]
    #[doc = r" contract."]
    pub fn raw_instance(&self) -> &self::ethcontract::dyns::DynInstance {
      &self.methods.instance
    }
    #[doc = r" Returns a mutable reference to the raw runtime instance used by"]
    #[doc = r" this contract."]
    fn raw_instance_mut(&mut self) -> &mut self::ethcontract::dyns::DynInstance {
      &mut self.methods.instance
    }
  }
  impl std::fmt::Debug for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      f.debug_tuple(stringify!(IndividuallyCappedCrowdsale))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = r" Returns an object that allows accessing typed method signatures."]
    pub fn signatures() -> Signatures {
      Signatures
    }
    #[doc = r" Retrieves a reference to type containing all the generated"]
    #[doc = r" contract methods. This can be used for methods where the name"]
    #[doc = r" would collide with a common method (like `at` or `deployed`)."]
    pub fn methods(&self) -> &Methods {
      &self.methods
    }
  }
  #[doc = r" Type containing signatures for all methods for generated contract type."]
  #[derive(Clone, Copy)]
  pub struct Signatures;
  impl Signatures {
    #[doc = "Returns signature for method `renounceCapper()`."]
    #[allow(clippy::type_complexity)]
    pub fn renounce_capper(&self) -> self::ethcontract::contract::Signature<(), ()> {
      self::ethcontract::contract::Signature::new([93, 85, 118, 248])
    }
    #[doc = "Returns signature for method `weiRaised():(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn wei_raised(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::U256> {
      self::ethcontract::contract::Signature::new([64, 66, 182, 111])
    }
    #[doc = "Returns signature for method `buyTokens(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn buy_tokens(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([236, 138, 196, 216])
    }
    #[doc = "Returns signature for method `isCapper(address):(bool)`."]
    #[allow(clippy::type_complexity)]
    pub fn is_capper(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), bool> {
      self::ethcontract::contract::Signature::new([57, 86, 69, 97])
    }
    #[doc = "Returns signature for method `addCapper(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn add_capper(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([141, 251, 207, 54])
    }
    #[doc = "Returns signature for method `wallet():(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn wallet(&self) -> self::ethcontract::contract::Signature<(), self::ethcontract::Address> {
      self::ethcontract::contract::Signature::new([82, 30, 178, 115])
    }
    #[doc = "Returns signature for method `setCap(address,uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn set_cap(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::Address, self::ethcontract::U256),
      (),
    > {
      self::ethcontract::contract::Signature::new([128, 173, 44, 243])
    }
    #[doc = "Returns signature for method `token():(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn token(&self) -> self::ethcontract::contract::Signature<(), self::ethcontract::Address> {
      self::ethcontract::contract::Signature::new([252, 12, 84, 106])
    }
    #[doc = "Returns signature for method `getContribution(address):(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn get_contribution(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::Address,),
      self::ethcontract::U256,
    > {
      self::ethcontract::contract::Signature::new([33, 239, 247, 252])
    }
    #[doc = "Returns signature for method `getCap(address):(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn get_cap(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::Address,),
      self::ethcontract::U256,
    > {
      self::ethcontract::contract::Signature::new([179, 174, 251, 117])
    }
    #[doc = "Returns signature for method `rate():(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn rate(&self) -> self::ethcontract::contract::Signature<(), self::ethcontract::U256> {
      self::ethcontract::contract::Signature::new([44, 78, 114, 46])
    }
  }
  #[doc = r" Type containing all contract methods for generated contract type."]
  #[derive(Clone)]
  pub struct Methods {
    instance: self::ethcontract::dyns::DynInstance,
  }
  #[allow(clippy::too_many_arguments, clippy::type_complexity)]
  impl Methods {
    #[doc = "Generated by `ethcontract`"]
    pub fn renounce_capper(&self) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([93, 85, 118, 248], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn wei_raised(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([64, 66, 182, 111], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn buy_tokens(
      &self,
      beneficiary: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([236, 138, 196, 216], (beneficiary,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn is_capper(
      &self,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<bool> {
      self
        .instance
        .view_method([57, 86, 69, 97], (account,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn add_capper(
      &self,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([141, 251, 207, 54], (account,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn wallet(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([82, 30, 178, 115], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn set_cap(
      &self,
      beneficiary: self::ethcontract::Address,
      cap: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([128, 173, 44, 243], (beneficiary, cap))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn token(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([252, 12, 84, 106], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get_contribution(
      &self,
      beneficiary: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([33, 239, 247, 252], (beneficiary,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get_cap(
      &self,
      beneficiary: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([179, 174, 251, 117], (beneficiary,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn rate(&self) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([44, 78, 114, 46], ())
        .expect("generated call")
    }
  }
  impl std::ops::Deref for Contract {
    type Target = Methods;
    fn deref(&self) -> &Self::Target {
      &self.methods
    }
  }
  impl Contract {
    #[doc = r" Returns a method builder to setup a call to a smart"]
    #[doc = r" contract's fallback function."]
    pub fn fallback<D>(&self, data: D) -> self::ethcontract::dyns::DynMethodBuilder<()>
    where
      D: Into<Vec<u8>>,
    {
      self
        .raw_instance()
        .fallback(data)
        .expect("generated fallback method")
    }
  }
  #[doc = r" Module containing all generated data models for this contract's"]
  #[doc = r" events."]
  pub mod event_data {
    use super::ethcontract;
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct TokensPurchased {
      pub purchaser: self::ethcontract::Address,
      pub beneficiary: self::ethcontract::Address,
      pub value: self::ethcontract::U256,
      pub amount: self::ethcontract::U256,
    }
    impl TokensPurchased {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          111, 175, 147, 35, 26, 69, 110, 85, 45, 188, 153, 97, 245, 141, 151, 19, 238, 79, 46,
          105, 209, 95, 25, 117, 176, 80, 239, 9, 17, 5, 58, 123,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`TokensPurchased(address,address,uint256,uint256)`"]
      pub fn abi_signature() -> &'static str {
        "TokensPurchased(address,address,uint256,uint256)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for TokensPurchased {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (purchaser, beneficiary, value, amount) =
          self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(TokensPurchased {
          purchaser,
          beneficiary,
          value,
          amount,
        })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct CapperAdded {
      pub account: self::ethcontract::Address,
    }
    impl CapperAdded {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          167, 85, 92, 149, 182, 157, 79, 92, 200, 71, 136, 31, 235, 74, 178, 136, 58, 25, 33, 49,
          158, 52, 250, 32, 67, 116, 123, 121, 61, 101, 179, 110,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`CapperAdded(address)`"]
      pub fn abi_signature() -> &'static str {
        "CapperAdded(address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for CapperAdded {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (account,) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(CapperAdded { account })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct CapperRemoved {
      pub account: self::ethcontract::Address,
    }
    impl CapperRemoved {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          66, 116, 0, 210, 121, 197, 6, 223, 97, 2, 36, 178, 46, 204, 232, 155, 105, 63, 193, 134,
          88, 100, 17, 63, 33, 200, 209, 156, 31, 12, 42, 59,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`CapperRemoved(address)`"]
      pub fn abi_signature() -> &'static str {
        "CapperRemoved(address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for CapperRemoved {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (account,) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(CapperRemoved { account })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
  }
  impl Contract {
    #[doc = r" Retrieves a handle to a type containing for creating event"]
    #[doc = r" streams for all the contract events."]
    pub fn events(&self) -> Events<'_> {
      Events {
        instance: self.raw_instance(),
      }
    }
  }
  pub struct Events<'a> {
    instance: &'a self::ethcontract::dyns::DynInstance,
  }
  impl Events<'_> {
    #[doc = r" Generated by `ethcontract`."]
    pub fn tokens_purchased(&self) -> self::event_builders::TokensPurchasedBuilder {
      self::event_builders::TokensPurchasedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            111, 175, 147, 35, 26, 69, 110, 85, 45, 188, 153, 97, 245, 141, 151, 19, 238, 79, 46,
            105, 209, 95, 25, 117, 176, 80, 239, 9, 17, 5, 58, 123,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn capper_added(&self) -> self::event_builders::CapperAddedBuilder {
      self::event_builders::CapperAddedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            167, 85, 92, 149, 182, 157, 79, 92, 200, 71, 136, 31, 235, 74, 178, 136, 58, 25, 33,
            49, 158, 52, 250, 32, 67, 116, 123, 121, 61, 101, 179, 110,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn capper_removed(&self) -> self::event_builders::CapperRemovedBuilder {
      self::event_builders::CapperRemovedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            66, 116, 0, 210, 121, 197, 6, 223, 97, 2, 36, 178, 46, 204, 232, 155, 105, 63, 193,
            134, 88, 100, 17, 63, 33, 200, 209, 156, 31, 12, 42, 59,
          ]))
          .expect("generated event filter"),
      )
    }
  }
  #[doc = r" Module containing the generated event stream builders with type safe"]
  #[doc = r" filter methods for this contract's events."]
  pub mod event_builders {
    use super::ethcontract;
    use super::event_data;
    #[doc = "A builder for creating a filtered stream of `TokensPurchased` events."]
    pub struct TokensPurchasedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::TokensPurchased>,
    );
    impl TokensPurchasedBuilder {
      #[doc = r" Sets the starting block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the latest block."]
      #[allow(clippy::wrong_self_convention)]
      pub fn from_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).from_block(block);
        self
      }
      #[doc = r" Sets the last block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the streaming until the end of days."]
      #[allow(clippy::wrong_self_convention)]
      pub fn to_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).to_block(block);
        self
      }
      #[doc = r" Limits the number of events that can be retrieved by this filter."]
      #[doc = r""]
      #[doc = r" Note that this parameter is non-standard."]
      pub fn limit(mut self, value: usize) -> Self {
        self.0 = (self.0).limit(value);
        self
      }
      #[doc = r" Sets the polling interval. This is used as the interval between"]
      #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
      pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
        self.0 = (self.0).poll_interval(value);
        self
      }
      #[doc = "Adds a filter for the purchaser event parameter."]
      pub fn purchaser(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = "Adds a filter for the beneficiary event parameter."]
      pub fn beneficiary(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic1(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::TokensPurchased>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::TokensPurchased>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `CapperAdded` events."]
    pub struct CapperAddedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::CapperAdded>,
    );
    impl CapperAddedBuilder {
      #[doc = r" Sets the starting block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the latest block."]
      #[allow(clippy::wrong_self_convention)]
      pub fn from_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).from_block(block);
        self
      }
      #[doc = r" Sets the last block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the streaming until the end of days."]
      #[allow(clippy::wrong_self_convention)]
      pub fn to_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).to_block(block);
        self
      }
      #[doc = r" Limits the number of events that can be retrieved by this filter."]
      #[doc = r""]
      #[doc = r" Note that this parameter is non-standard."]
      pub fn limit(mut self, value: usize) -> Self {
        self.0 = (self.0).limit(value);
        self
      }
      #[doc = r" Sets the polling interval. This is used as the interval between"]
      #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
      pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
        self.0 = (self.0).poll_interval(value);
        self
      }
      #[doc = "Adds a filter for the account event parameter."]
      pub fn account(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::CapperAdded>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::CapperAdded>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `CapperRemoved` events."]
    pub struct CapperRemovedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::CapperRemoved>,
    );
    impl CapperRemovedBuilder {
      #[doc = r" Sets the starting block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the latest block."]
      #[allow(clippy::wrong_self_convention)]
      pub fn from_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).from_block(block);
        self
      }
      #[doc = r" Sets the last block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the streaming until the end of days."]
      #[allow(clippy::wrong_self_convention)]
      pub fn to_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).to_block(block);
        self
      }
      #[doc = r" Limits the number of events that can be retrieved by this filter."]
      #[doc = r""]
      #[doc = r" Note that this parameter is non-standard."]
      pub fn limit(mut self, value: usize) -> Self {
        self.0 = (self.0).limit(value);
        self
      }
      #[doc = r" Sets the polling interval. This is used as the interval between"]
      #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
      pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
        self.0 = (self.0).poll_interval(value);
        self
      }
      #[doc = "Adds a filter for the account event parameter."]
      pub fn account(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::CapperRemoved>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::CapperRemoved>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
  }
  impl Contract {
    #[doc = r" Returns a log stream with all events."]
    pub fn all_events(&self) -> self::ethcontract::dyns::DynAllEventsBuilder<Event> {
      self::ethcontract::dyns::DynAllEventsBuilder::new(
        self.raw_instance().web3(),
        self.address(),
        self.deployment_information(),
      )
    }
  }
  #[doc = r" A contract event."]
  #[derive(Clone, Debug, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
  pub enum Event {
    CapperAdded(self::event_data::CapperAdded),
    CapperRemoved(self::event_data::CapperRemoved),
    TokensPurchased(self::event_data::TokensPurchased),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([167 , 85 , 92 , 149 , 182 , 157 , 79 , 92 , 200 , 71 , 136 , 31 , 235 , 74 , 178 , 136 , 58 , 25 , 33 , 49 , 158 , 52 , 250 , 32 , 67 , 116 , 123 , 121 , 61 , 101 , 179 , 110]) => Ok (Event :: CapperAdded (log . clone () . decode (Contract :: raw_contract () . abi . event ("CapperAdded") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([66 , 116 , 0 , 210 , 121 , 197 , 6 , 223 , 97 , 2 , 36 , 178 , 46 , 204 , 232 , 155 , 105 , 63 , 193 , 134 , 88 , 100 , 17 , 63 , 33 , 200 , 209 , 156 , 31 , 12 , 42 , 59]) => Ok (Event :: CapperRemoved (log . clone () . decode (Contract :: raw_contract () . abi . event ("CapperRemoved") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([111 , 175 , 147 , 35 , 26 , 69 , 110 , 85 , 45 , 188 , 153 , 97 , 245 , 141 , 151 , 19 , 238 , 79 , 46 , 105 , 209 , 95 , 25 , 117 , 176 , 80 , 239 , 9 , 17 , 5 , 58 , 123]) => Ok (Event :: TokensPurchased (log . clone () . decode (Contract :: raw_contract () . abi . event ("TokensPurchased") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::individually_capped_crowdsale::Contract as IndividuallyCappedCrowdsale;
