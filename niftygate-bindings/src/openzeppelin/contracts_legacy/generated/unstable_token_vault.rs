#[allow(dead_code)]
pub mod unstable_token_vault {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"__unstable__TokenVault\",\"abi\":[{\"type\":\"function\",\"name\":\"primary\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transfer\",\"inputs\":[{\"name\":\"token\",\"type\":\"address\"},{\"name\":\"to\",\"type\":\"address\"},{\"name\":\"amount\",\"type\":\"uint256\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"transferPrimary\",\"inputs\":[{\"name\":\"recipient\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"PrimaryTransferred\",\"inputs\":[{\"name\":\"recipient\",\"type\":\"address\",\"indexed\":false}],\"anonymous\":false}],\"bytecode\":\"608060405260006100146100bf60201b60201c565b9050806000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4101e71e974f68df5e9730cc223280b41654676bbb052cdcc735c3337e64d2d981604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a1506100c7565b600033905090565b61054a806100d66000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80632348238c14610046578063beabacc81461008a578063c6dbdf61146100f8575b600080fd5b6100886004803603602081101561005c57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190505050610142565b005b6100f6600480360360608110156100a057600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803573ffffffffffffffffffffffffffffffffffffffff1690602001909291908035906020019092919050505061031a565b005b61010061048e565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166101826104b7565b73ffffffffffffffffffffffffffffffffffffffff16146101ee576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602c8152602001806104ea602c913960400191505060405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161415610274576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602a8152602001806104c0602a913960400191505060405180910390fd5b806000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4101e71e974f68df5e9730cc223280b41654676bbb052cdcc735c3337e64d2d981604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a150565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1661035a6104b7565b73ffffffffffffffffffffffffffffffffffffffff16146103c6576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602c8152602001806104ea602c913960400191505060405180910390fd5b8273ffffffffffffffffffffffffffffffffffffffff1663a9059cbb83836040518363ffffffff1660e01b8152600401808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200182815260200192505050602060405180830381600087803b15801561044d57600080fd5b505af1158015610461573d6000803e3d6000fd5b505050506040513d602081101561047757600080fd5b810190808051906020019092919050505050505050565b60008060009054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b60003390509056fe5365636f6e646172793a206e6577207072696d61727920697320746865207a65726f20616464726573735365636f6e646172793a2063616c6c6572206973206e6f7420746865207072696d617279206163636f756e74a265627a7a723158200814f9815547e5df2957c23308b9b337c731c16ad59cdfa3683e348e8f4b50fa64736f6c63430005110032\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(__unstable__TokenVault))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = "Generated by `ethcontract`"]
    #[allow(clippy::too_many_arguments)]
    pub fn builder<F, B, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
    ) -> self::ethcontract::dyns::DynDeployBuilder<Self>
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
      use self::ethcontract::contract::DeployBuilder;
      use self::ethcontract::dyns::DynTransport;
      use self::ethcontract::web3::api::Web3;
      let transport = DynTransport::new(web3.transport().clone());
      let web3 = Web3::new(transport);
      let bytecode = Self::raw_contract().bytecode.clone();
      DeployBuilder::new(web3, bytecode, ()).expect("valid deployment args")
    }
  }
  impl self::ethcontract::contract::Deploy<self::ethcontract::dyns::DynTransport> for Contract {
    type Context = self::ethcontract::common::Bytecode;
    fn bytecode(cx: &Self::Context) -> &self::ethcontract::common::Bytecode {
      cx
    }
    fn abi(_: &Self::Context) -> &self::ethcontract::common::Abi {
      &Self::raw_contract().abi
    }
    fn from_deployment(
      web3: self::ethcontract::dyns::DynWeb3,
      address: self::ethcontract::Address,
      transaction_hash: self::ethcontract::H256,
      _: Self::Context,
    ) -> Self {
      Self::with_deployment_info(&web3, address, Some(transaction_hash.into()))
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
    #[doc = "Returns signature for method `primary():(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn primary(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::Address> {
      self::ethcontract::contract::Signature::new([198, 219, 223, 97])
    }
    #[doc = "Returns signature for method `transfer(address,address,uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn transfer(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (
        self::ethcontract::Address,
        self::ethcontract::Address,
        self::ethcontract::U256,
      ),
      (),
    > {
      self::ethcontract::contract::Signature::new([190, 171, 172, 200])
    }
    #[doc = "Returns signature for method `transferPrimary(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn transfer_primary(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([35, 72, 35, 140])
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
    pub fn primary(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([198, 219, 223, 97], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn transfer(
      &self,
      token: self::ethcontract::Address,
      to: self::ethcontract::Address,
      amount: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([190, 171, 172, 200], (token, to, amount))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn transfer_primary(
      &self,
      recipient: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([35, 72, 35, 140], (recipient,))
        .expect("generated call")
    }
  }
  impl std::ops::Deref for Contract {
    type Target = Methods;
    fn deref(&self) -> &Self::Target {
      &self.methods
    }
  }
  #[doc = r" Module containing all generated data models for this contract's"]
  #[doc = r" events."]
  pub mod event_data {
    use super::ethcontract;
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct PrimaryTransferred {
      pub recipient: self::ethcontract::Address,
    }
    impl PrimaryTransferred {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          65, 1, 231, 30, 151, 79, 104, 223, 94, 151, 48, 204, 34, 50, 128, 180, 22, 84, 103, 107,
          187, 5, 44, 220, 199, 53, 195, 51, 126, 100, 210, 217,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`PrimaryTransferred(address)`"]
      pub fn abi_signature() -> &'static str {
        "PrimaryTransferred(address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for PrimaryTransferred {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (recipient,) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(PrimaryTransferred { recipient })
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
    pub fn primary_transferred(&self) -> self::event_builders::PrimaryTransferredBuilder {
      self::event_builders::PrimaryTransferredBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            65, 1, 231, 30, 151, 79, 104, 223, 94, 151, 48, 204, 34, 50, 128, 180, 22, 84, 103,
            107, 187, 5, 44, 220, 199, 53, 195, 51, 126, 100, 210, 217,
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
    #[doc = "A builder for creating a filtered stream of `PrimaryTransferred` events."]
    pub struct PrimaryTransferredBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::PrimaryTransferred>,
    );
    impl PrimaryTransferredBuilder {
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
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::PrimaryTransferred>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::PrimaryTransferred>,
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
    PrimaryTransferred(self::event_data::PrimaryTransferred),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([65 , 1 , 231 , 30 , 151 , 79 , 104 , 223 , 94 , 151 , 48 , 204 , 34 , 50 , 128 , 180 , 22 , 84 , 103 , 107 , 187 , 5 , 44 , 220 , 199 , 53 , 195 , 51 , 126 , 100 , 210 , 217]) => Ok (Event :: PrimaryTransferred (log . clone () . decode (Contract :: raw_contract () . abi . event ("PrimaryTransferred") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::unstable_token_vault::Contract as __unstable__TokenVault;
