#[allow(dead_code)]
pub mod i_relay_recipient {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"IRelayRecipient\",\"abi\":[{\"type\":\"function\",\"name\":\"preRelayedCall\",\"inputs\":[{\"name\":\"context\",\"type\":\"bytes\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"postRelayedCall\",\"inputs\":[{\"name\":\"context\",\"type\":\"bytes\"},{\"name\":\"success\",\"type\":\"bool\"},{\"name\":\"actualCharge\",\"type\":\"uint256\"},{\"name\":\"preRetVal\",\"type\":\"bytes32\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"getHubAddr\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"acceptRelayedCall\",\"inputs\":[{\"name\":\"relay\",\"type\":\"address\"},{\"name\":\"from\",\"type\":\"address\"},{\"name\":\"encodedFunction\",\"type\":\"bytes\"},{\"name\":\"transactionFee\",\"type\":\"uint256\"},{\"name\":\"gasPrice\",\"type\":\"uint256\"},{\"name\":\"gasLimit\",\"type\":\"uint256\"},{\"name\":\"nonce\",\"type\":\"uint256\"},{\"name\":\"approvalData\",\"type\":\"bytes\"},{\"name\":\"maxPossibleCharge\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"},{\"name\":\"\",\"type\":\"bytes\"}],\"constant\":true,\"stateMutability\":\"view\"}],\"bytecode\":\"\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(IRelayRecipient))
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
    #[doc = "Returns signature for method `preRelayedCall(bytes):(bytes32)`."]
    #[allow(clippy::type_complexity)]
    pub fn pre_relayed_call(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::tokens::Bytes<Vec<u8>>,),
      self::ethcontract::tokens::Bytes<[u8; 32]>,
    > {
      self::ethcontract::contract::Signature::new([128, 39, 77, 183])
    }
    #[doc = "Returns signature for method `postRelayedCall(bytes,bool,uint256,bytes32)`."]
    #[allow(clippy::type_complexity)]
    pub fn post_relayed_call(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        self::ethcontract::tokens::Bytes<Vec<u8>>,
        bool,
        self::ethcontract::U256,
        self::ethcontract::tokens::Bytes<[u8; 32]>,
      ),
      (),
    > {
      self::ethcontract::contract::Signature::new([224, 110, 14, 34])
    }
    #[doc = "Returns signature for method `getHubAddr():(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn get_hub_addr(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::Address> {
      self::ethcontract::contract::Signature::new([116, 232, 97, 214])
    }
    #[doc = "Returns signature for method `acceptRelayedCall(address,address,bytes,uint256,uint256,uint256,uint256,bytes,uint256):(uint256,bytes)`."]
    #[allow(clippy::type_complexity)]
    pub fn accept_relayed_call(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        self::ethcontract::Address,
        self::ethcontract::Address,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
        self::ethcontract::U256,
        self::ethcontract::U256,
        self::ethcontract::U256,
        self::ethcontract::U256,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
        self::ethcontract::U256,
      ),
      (
        self::ethcontract::U256,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
      ),
    > {
      self::ethcontract::contract::Signature::new([131, 148, 126, 160])
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
    pub fn pre_relayed_call(
      &self,
      context: self::ethcontract::tokens::Bytes<Vec<u8>>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::tokens::Bytes<[u8; 32]>> {
      self
        .instance
        .method([128, 39, 77, 183], (context,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn post_relayed_call(
      &self,
      context: self::ethcontract::tokens::Bytes<Vec<u8>>,
      success: bool,
      actual_charge: self::ethcontract::U256,
      pre_ret_val: self::ethcontract::tokens::Bytes<[u8; 32]>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method(
          [224, 110, 14, 34],
          (context, success, actual_charge, pre_ret_val),
        )
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get_hub_addr(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([116, 232, 97, 214], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn accept_relayed_call(
      &self,
      relay: self::ethcontract::Address,
      from: self::ethcontract::Address,
      encoded_function: self::ethcontract::tokens::Bytes<Vec<u8>>,
      transaction_fee: self::ethcontract::U256,
      gas_price: self::ethcontract::U256,
      gas_limit: self::ethcontract::U256,
      nonce: self::ethcontract::U256,
      approval_data: self::ethcontract::tokens::Bytes<Vec<u8>>,
      max_possible_charge: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<(
      self::ethcontract::U256,
      self::ethcontract::tokens::Bytes<Vec<u8>>,
    )> {
      self
        .instance
        .view_method(
          [131, 148, 126, 160],
          (
            relay,
            from,
            encoded_function,
            transaction_fee,
            gas_price,
            gas_limit,
            nonce,
            approval_data,
            max_possible_charge,
          ),
        )
        .expect("generated call")
    }
  }
  impl std::ops::Deref for Contract {
    type Target = Methods;
    fn deref(&self) -> &Self::Target {
      &self.methods
    }
  }
}
pub use self::i_relay_recipient::Contract as IRelayRecipient;
