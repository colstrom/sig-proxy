#[allow(dead_code)]
pub mod ierc2612_upgradeable {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"IERC2612Upgradeable\",\"abi\":[{\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"nonces\",\"inputs\":[{\"name\":\"owner\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"permit\",\"inputs\":[{\"name\":\"owner\",\"type\":\"address\"},{\"name\":\"spender\",\"type\":\"address\"},{\"name\":\"value\",\"type\":\"uint256\"},{\"name\":\"deadline\",\"type\":\"uint256\"},{\"name\":\"v\",\"type\":\"uint8\"},{\"name\":\"r\",\"type\":\"bytes32\"},{\"name\":\"s\",\"type\":\"bytes32\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"}],\"bytecode\":\"\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(IERC2612Upgradeable))
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
    #[doc = "Returns signature for method `DOMAIN_SEPARATOR():(bytes32)`."]
    #[allow(clippy::type_complexity)]
    pub fn domain_separator(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::tokens::Bytes<[u8; 32]>>
    {
      self::ethcontract::contract::Signature::new([54, 68, 229, 21])
    }
    #[doc = "Returns signature for method `nonces(address):(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn nonces(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::Address,),
      self::ethcontract::U256,
    > {
      self::ethcontract::contract::Signature::new([126, 206, 190, 0])
    }
    #[doc = "Returns signature for method `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)`."]
    #[allow(clippy::type_complexity)]
    pub fn permit(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        self::ethcontract::Address,
        self::ethcontract::Address,
        self::ethcontract::U256,
        self::ethcontract::U256,
        u8,
        self::ethcontract::tokens::Bytes<[u8; 32]>,
        self::ethcontract::tokens::Bytes<[u8; 32]>,
      ),
      (),
    > {
      self::ethcontract::contract::Signature::new([213, 5, 172, 207])
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
    pub fn domain_separator(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::tokens::Bytes<[u8; 32]>>
    {
      self
        .instance
        .view_method([54, 68, 229, 21], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn nonces(
      &self,
      owner: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([126, 206, 190, 0], (owner,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn permit(
      &self,
      owner: self::ethcontract::Address,
      spender: self::ethcontract::Address,
      value: self::ethcontract::U256,
      deadline: self::ethcontract::U256,
      v: u8,
      r: self::ethcontract::tokens::Bytes<[u8; 32]>,
      s: self::ethcontract::tokens::Bytes<[u8; 32]>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method(
          [213, 5, 172, 207],
          (owner, spender, value, deadline, v, r, s),
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
pub use self::ierc2612_upgradeable::Contract as IERC2612Upgradeable;
