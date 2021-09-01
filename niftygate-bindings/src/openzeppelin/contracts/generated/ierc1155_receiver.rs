#[allow(dead_code)]
pub mod ierc1155_receiver {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"IERC1155Receiver\",\"abi\":[{\"type\":\"function\",\"name\":\"onERC1155Received\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\"},{\"name\":\"from\",\"type\":\"address\"},{\"name\":\"id\",\"type\":\"uint256\"},{\"name\":\"value\",\"type\":\"uint256\"},{\"name\":\"data\",\"type\":\"bytes\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes4\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"onERC1155BatchReceived\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\"},{\"name\":\"from\",\"type\":\"address\"},{\"name\":\"ids\",\"type\":\"uint256[]\"},{\"name\":\"values\",\"type\":\"uint256[]\"},{\"name\":\"data\",\"type\":\"bytes\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes4\"}],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"supportsInterface\",\"inputs\":[{\"name\":\"interfaceId\",\"type\":\"bytes4\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"constant\":false,\"stateMutability\":\"view\"}],\"bytecode\":\"\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(IERC1155Receiver))
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
    #[doc = "Returns signature for method `onERC1155Received(address,address,uint256,uint256,bytes):(bytes4)`."]
    #[allow(clippy::type_complexity)]
    pub fn on_erc1155_received(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        self::ethcontract::Address,
        self::ethcontract::Address,
        self::ethcontract::U256,
        self::ethcontract::U256,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
      ),
      self::ethcontract::tokens::Bytes<[u8; 4]>,
    > {
      self::ethcontract::contract::Signature::new([242, 58, 110, 97])
    }
    #[doc = "Returns signature for method `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes):(bytes4)`."]
    #[allow(clippy::type_complexity)]
    pub fn on_erc1155_batch_received(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        self::ethcontract::Address,
        self::ethcontract::Address,
        Vec<self::ethcontract::U256>,
        Vec<self::ethcontract::U256>,
        self::ethcontract::tokens::Bytes<Vec<u8>>,
      ),
      self::ethcontract::tokens::Bytes<[u8; 4]>,
    > {
      self::ethcontract::contract::Signature::new([188, 25, 124, 129])
    }
    #[doc = "Returns signature for method `supportsInterface(bytes4):(bool)`."]
    #[allow(clippy::type_complexity)]
    pub fn supports_interface(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::tokens::Bytes<[u8; 4]>,), bool>
    {
      self::ethcontract::contract::Signature::new([1, 255, 201, 167])
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
    pub fn on_erc1155_received(
      &self,
      operator: self::ethcontract::Address,
      from: self::ethcontract::Address,
      id: self::ethcontract::U256,
      value: self::ethcontract::U256,
      data: self::ethcontract::tokens::Bytes<Vec<u8>>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::tokens::Bytes<[u8; 4]>> {
      self
        .instance
        .method([242, 58, 110, 97], (operator, from, id, value, data))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn on_erc1155_batch_received(
      &self,
      operator: self::ethcontract::Address,
      from: self::ethcontract::Address,
      ids: Vec<self::ethcontract::U256>,
      values: Vec<self::ethcontract::U256>,
      data: self::ethcontract::tokens::Bytes<Vec<u8>>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::tokens::Bytes<[u8; 4]>> {
      self
        .instance
        .method([188, 25, 124, 129], (operator, from, ids, values, data))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn supports_interface(
      &self,
      interface_id: self::ethcontract::tokens::Bytes<[u8; 4]>,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<bool> {
      self
        .instance
        .view_method([1, 255, 201, 167], (interface_id,))
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
pub use self::ierc1155_receiver::Contract as IERC1155Receiver;
