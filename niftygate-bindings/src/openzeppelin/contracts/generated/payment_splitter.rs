#[allow(dead_code)]
pub mod payment_splitter {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"PaymentSplitter\",\"abi\":[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"payees\",\"type\":\"address[]\"},{\"name\":\"shares_\",\"type\":\"uint256[]\"}]},{\"type\":\"function\",\"name\":\"payee\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"release\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"shares\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"totalShares\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"released\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"totalReleased\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":false,\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"PaymentReceived\",\"inputs\":[{\"name\":\"from\",\"type\":\"address\",\"indexed\":false},{\"name\":\"amount\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PayeeAdded\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":false},{\"name\":\"shares\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PaymentReleased\",\"inputs\":[{\"name\":\"to\",\"type\":\"address\",\"indexed\":false},{\"name\":\"amount\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"receive\"}],\"bytecode\":\"608060405260405162000b6a38038062000b6a8339810160408190526200002691620003db565b8051825114620000985760405162461bcd60e51b815260206004820152603260248201527f5061796d656e7453706c69747465723a2070617965657320616e6420736861726044820152710cae640d8cadccee8d040dad2e6dac2e8c6d60731b60648201526084015b60405180910390fd5b6000825111620000eb5760405162461bcd60e51b815260206004820152601a60248201527f5061796d656e7453706c69747465723a206e6f2070617965657300000000000060448201526064016200008f565b60005b82518110156200016f576200015a8382815181106200011d57634e487b7160e01b600052603260045260246000fd5b60200260200101518383815181106200014657634e487b7160e01b600052603260045260246000fd5b60200260200101516200017860201b60201c565b8062000166816200052c565b915050620000ee565b50505062000576565b6001600160a01b038216620001e55760405162461bcd60e51b815260206004820152602c60248201527f5061796d656e7453706c69747465723a206163636f756e74206973207468652060448201526b7a65726f206164647265737360a01b60648201526084016200008f565b60008111620002375760405162461bcd60e51b815260206004820152601d60248201527f5061796d656e7453706c69747465723a2073686172657320617265203000000060448201526064016200008f565b6001600160a01b03821660009081526002602052604090205415620002b35760405162461bcd60e51b815260206004820152602b60248201527f5061796d656e7453706c69747465723a206163636f756e7420616c726561647960448201526a206861732073686172657360a81b60648201526084016200008f565b60048054600181019091557f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b0180546001600160a01b0319166001600160a01b0384169081179091556000908152600260205260408120829055546200031b90829062000511565b600055604080516001600160a01b0384168152602081018390527f40c340f65e17194d14ddddb073d3c9f888e3cb52b5aae0c6c7706b4fbc905fac910160405180910390a15050565b600082601f83011262000375578081fd5b815160206200038e6200038883620004eb565b620004b8565b80838252828201915082860187848660051b8901011115620003ae578586fd5b855b85811015620003ce57815184529284019290840190600101620003b0565b5090979650505050505050565b60008060408385031215620003ee578182fd5b82516001600160401b038082111562000405578384fd5b818501915085601f83011262000419578384fd5b815160206200042c6200038883620004eb565b8083825282820191508286018a848660051b89010111156200044c578889fd5b8896505b84871015620004855780516001600160a01b03811681146200047057898afd5b83526001969096019591830191830162000450565b50918801519196509093505050808211156200049f578283fd5b50620004ae8582860162000364565b9150509250929050565b604051601f8201601f191681016001600160401b0381118282101715620004e357620004e362000560565b604052919050565b60006001600160401b0382111562000507576200050762000560565b5060051b60200190565b600082198211156200052757620005276200054a565b500190565b60006000198214156200054357620005436200054a565b5060010190565b634e487b7160e01b600052601160045260246000fd5b634e487b7160e01b600052604160045260246000fd5b6105e480620005866000396000f3fe6080604052600436106100595760003560e01c806319165587146100a75780633a98ef39146100c95780638b83209b146100ed5780639852595c14610125578063ce7c2ac21461015b578063e33b7de314610191576100a2565b366100a2577f6ef95f06320e7a25a04a175ca677b7052bdd97131872c2192525a629f51be77033604080516001600160a01b0390921682523460208301520160405180910390a1005b600080fd5b3480156100b357600080fd5b506100c76100c23660046104d7565b6101a6565b005b3480156100d557600080fd5b506000545b6040519081526020015b60405180910390f35b3480156100f957600080fd5b5061010d6101083660046104fa565b61037b565b6040516001600160a01b0390911681526020016100e4565b34801561013157600080fd5b506100da6101403660046104d7565b6001600160a01b031660009081526003602052604090205490565b34801561016757600080fd5b506100da6101763660046104d7565b6001600160a01b031660009081526002602052604090205490565b34801561019d57600080fd5b506001546100da565b6001600160a01b03811660009081526002602052604090205461021f5760405162461bcd60e51b815260206004820152602660248201527f5061796d656e7453706c69747465723a206163636f756e7420686173206e6f2060448201526573686172657360d01b60648201526084015b60405180910390fd5b60006001544761022f9190610512565b6001600160a01b03831660009081526003602090815260408083205483546002909352908320549394509192610265908561054a565b61026f919061052a565b6102799190610569565b9050806102dc5760405162461bcd60e51b815260206004820152602b60248201527f5061796d656e7453706c69747465723a206163636f756e74206973206e6f742060448201526a191d59481c185e5b595b9d60aa1b6064820152608401610216565b6001600160a01b038316600090815260036020526040902054610300908290610512565b6001600160a01b038416600090815260036020526040902055600154610327908290610512565b60015561033483826103b9565b604080516001600160a01b0385168152602081018390527fdf20fd1e76bc69d672e4814fafb2c449bba3a5369d8359adf9e05e6fde87b056910160405180910390a1505050565b60006004828154811061039e57634e487b7160e01b600052603260045260246000fd5b6000918252602090912001546001600160a01b031692915050565b804710156104095760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e63650000006044820152606401610216565b6000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114610456576040519150601f19603f3d011682016040523d82523d6000602084013e61045b565b606091505b50509050806104d25760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d617920686176652072657665727465640000000000006064820152608401610216565b505050565b6000602082840312156104e8578081fd5b81356104f381610596565b9392505050565b60006020828403121561050b578081fd5b5035919050565b6000821982111561052557610525610580565b500190565b60008261054557634e487b7160e01b81526012600452602481fd5b500490565b600081600019048311821515161561056457610564610580565b500290565b60008282101561057b5761057b610580565b500390565b634e487b7160e01b600052601160045260246000fd5b6001600160a01b03811681146105ab57600080fd5b5056fea2646970667358221220fd47b4c67cba81d391f2a7510b725d33aab03c0d4da3265f86bffb2017e1ec8c64736f6c63430008030033\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(PaymentSplitter))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = "Generated by `ethcontract`"]
    #[allow(clippy::too_many_arguments)]
    pub fn builder<F, B, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      payees: Vec<self::ethcontract::Address>,
      shares: Vec<self::ethcontract::U256>,
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
      DeployBuilder::new(web3, bytecode, (payees, shares)).expect("valid deployment args")
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
    #[doc = "Returns signature for method `payee(uint256):(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn payee(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::U256,),
      self::ethcontract::Address,
    > {
      self::ethcontract::contract::Signature::new([139, 131, 32, 155])
    }
    #[doc = "Returns signature for method `release(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn release(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([25, 22, 85, 135])
    }
    #[doc = "Returns signature for method `shares(address):(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn shares(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::Address,),
      self::ethcontract::U256,
    > {
      self::ethcontract::contract::Signature::new([206, 124, 42, 194])
    }
    #[doc = "Returns signature for method `totalShares():(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn total_shares(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::U256> {
      self::ethcontract::contract::Signature::new([58, 152, 239, 57])
    }
    #[doc = "Returns signature for method `released(address):(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn released(
      &self,
    ) -> self::ethcontract::contract::Signature<
      (self::ethcontract::Address,),
      self::ethcontract::U256,
    > {
      self::ethcontract::contract::Signature::new([152, 82, 89, 92])
    }
    #[doc = "Returns signature for method `totalReleased():(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn total_released(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::U256> {
      self::ethcontract::contract::Signature::new([227, 59, 125, 227])
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
    pub fn payee(
      &self,
      index: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([139, 131, 32, 155], (index,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn release(
      &self,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([25, 22, 85, 135], (account,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn shares(
      &self,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([206, 124, 42, 194], (account,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn total_shares(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([58, 152, 239, 57], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn released(
      &self,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([152, 82, 89, 92], (account,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn total_released(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([227, 59, 125, 227], ())
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
    pub struct PaymentReceived {
      pub from: self::ethcontract::Address,
      pub amount: self::ethcontract::U256,
    }
    impl PaymentReceived {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          110, 249, 95, 6, 50, 14, 122, 37, 160, 74, 23, 92, 166, 119, 183, 5, 43, 221, 151, 19,
          24, 114, 194, 25, 37, 37, 166, 41, 245, 27, 231, 112,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`PaymentReceived(address,uint256)`"]
      pub fn abi_signature() -> &'static str {
        "PaymentReceived(address,uint256)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for PaymentReceived {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (from, amount) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(PaymentReceived { from, amount })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct PayeeAdded {
      pub account: self::ethcontract::Address,
      pub shares: self::ethcontract::U256,
    }
    impl PayeeAdded {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          64, 195, 64, 246, 94, 23, 25, 77, 20, 221, 221, 176, 115, 211, 201, 248, 136, 227, 203,
          82, 181, 170, 224, 198, 199, 112, 107, 79, 188, 144, 95, 172,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`PayeeAdded(address,uint256)`"]
      pub fn abi_signature() -> &'static str {
        "PayeeAdded(address,uint256)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for PayeeAdded {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (account, shares) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(PayeeAdded { account, shares })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct PaymentReleased {
      pub to: self::ethcontract::Address,
      pub amount: self::ethcontract::U256,
    }
    impl PaymentReleased {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          223, 32, 253, 30, 118, 188, 105, 214, 114, 228, 129, 79, 175, 178, 196, 73, 187, 163,
          165, 54, 157, 131, 89, 173, 249, 224, 94, 111, 222, 135, 176, 86,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`PaymentReleased(address,uint256)`"]
      pub fn abi_signature() -> &'static str {
        "PaymentReleased(address,uint256)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for PaymentReleased {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (to, amount) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(PaymentReleased { to, amount })
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
    pub fn payment_received(&self) -> self::event_builders::PaymentReceivedBuilder {
      self::event_builders::PaymentReceivedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            110, 249, 95, 6, 50, 14, 122, 37, 160, 74, 23, 92, 166, 119, 183, 5, 43, 221, 151, 19,
            24, 114, 194, 25, 37, 37, 166, 41, 245, 27, 231, 112,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn payee_added(&self) -> self::event_builders::PayeeAddedBuilder {
      self::event_builders::PayeeAddedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            64, 195, 64, 246, 94, 23, 25, 77, 20, 221, 221, 176, 115, 211, 201, 248, 136, 227, 203,
            82, 181, 170, 224, 198, 199, 112, 107, 79, 188, 144, 95, 172,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn payment_released(&self) -> self::event_builders::PaymentReleasedBuilder {
      self::event_builders::PaymentReleasedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            223, 32, 253, 30, 118, 188, 105, 214, 114, 228, 129, 79, 175, 178, 196, 73, 187, 163,
            165, 54, 157, 131, 89, 173, 249, 224, 94, 111, 222, 135, 176, 86,
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
    #[doc = "A builder for creating a filtered stream of `PaymentReceived` events."]
    pub struct PaymentReceivedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::PaymentReceived>,
    );
    impl PaymentReceivedBuilder {
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
        std::vec::Vec<self::ethcontract::Event<self::event_data::PaymentReceived>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::PaymentReceived>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `PayeeAdded` events."]
    pub struct PayeeAddedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::PayeeAdded>,
    );
    impl PayeeAddedBuilder {
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
        std::vec::Vec<self::ethcontract::Event<self::event_data::PayeeAdded>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::PayeeAdded>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `PaymentReleased` events."]
    pub struct PaymentReleasedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::PaymentReleased>,
    );
    impl PaymentReleasedBuilder {
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
        std::vec::Vec<self::ethcontract::Event<self::event_data::PaymentReleased>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::PaymentReleased>,
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
    PayeeAdded(self::event_data::PayeeAdded),
    PaymentReceived(self::event_data::PaymentReceived),
    PaymentReleased(self::event_data::PaymentReleased),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([64 , 195 , 64 , 246 , 94 , 23 , 25 , 77 , 20 , 221 , 221 , 176 , 115 , 211 , 201 , 248 , 136 , 227 , 203 , 82 , 181 , 170 , 224 , 198 , 199 , 112 , 107 , 79 , 188 , 144 , 95 , 172]) => Ok (Event :: PayeeAdded (log . clone () . decode (Contract :: raw_contract () . abi . event ("PayeeAdded") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([110 , 249 , 95 , 6 , 50 , 14 , 122 , 37 , 160 , 74 , 23 , 92 , 166 , 119 , 183 , 5 , 43 , 221 , 151 , 19 , 24 , 114 , 194 , 25 , 37 , 37 , 166 , 41 , 245 , 27 , 231 , 112]) => Ok (Event :: PaymentReceived (log . clone () . decode (Contract :: raw_contract () . abi . event ("PaymentReceived") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([223 , 32 , 253 , 30 , 118 , 188 , 105 , 214 , 114 , 228 , 129 , 79 , 175 , 178 , 196 , 73 , 187 , 163 , 165 , 54 , 157 , 131 , 89 , 173 , 249 , 224 , 94 , 111 , 222 , 135 , 176 , 86]) => Ok (Event :: PaymentReleased (log . clone () . decode (Contract :: raw_contract () . abi . event ("PaymentReleased") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::payment_splitter::Contract as PaymentSplitter;
