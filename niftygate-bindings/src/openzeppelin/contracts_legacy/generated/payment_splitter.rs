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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"PaymentSplitter\",\"abi\":[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"payees\",\"type\":\"address[]\"},{\"name\":\"shares\",\"type\":\"uint256[]\"}]},{\"type\":\"function\",\"name\":\"release\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\"}],\"outputs\":[],\"constant\":false,\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"totalReleased\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"totalShares\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"released\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"payee\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"shares\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"constant\":true,\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"PaymentReceived\",\"inputs\":[{\"name\":\"from\",\"type\":\"address\",\"indexed\":false},{\"name\":\"amount\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PayeeAdded\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":false},{\"name\":\"shares\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PaymentReleased\",\"inputs\":[{\"name\":\"to\",\"type\":\"address\",\"indexed\":false},{\"name\":\"amount\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"fallback\"}],\"bytecode\":\"60806040526040516200113e3803806200113e833981810160405260408110156200002957600080fd5b81019080805160405193929190846401000000008211156200004a57600080fd5b838201915060208201858111156200006157600080fd5b82518660208202830111640100000000821117156200007f57600080fd5b8083526020830192505050908051906020019060200280838360005b83811015620000b85780820151818401526020810190506200009b565b5050505090500160405260200180516040519392919084640100000000821115620000e257600080fd5b83820191506020820185811115620000f957600080fd5b82518660208202830111640100000000821117156200011757600080fd5b8083526020830192505050908051906020019060200280838360005b838110156200015057808201518184015260208101905062000133565b505050509050016040525050508051825114620001b9576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526032815260200180620010e16032913960400191505060405180910390fd5b600082511162000231576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601a8152602001807f5061796d656e7453706c69747465723a206e6f2070617965657300000000000081525060200191505060405180910390fd5b60008090505b825181101562000289576200027b8382815181106200025257fe5b60200260200101518383815181106200026757fe5b60200260200101516200029260201b60201c565b808060010191505062000237565b505050620005ef565b600073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1614156200031a576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602c815260200180620010b5602c913960400191505060405180910390fd5b6000811162000391576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601d8152602001807f5061796d656e7453706c69747465723a2073686172657320617265203000000081525060200191505060405180910390fd5b6000600260008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054146200042b576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602b81526020018062001113602b913960400191505060405180910390fd5b60048290806001815401808255809150509060018203906000526020600020016000909192909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505080600260008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550620004f1816000546200056660201b620006e71790919060201c565b6000819055507f40c340f65e17194d14ddddb073d3c9f888e3cb52b5aae0c6c7706b4fbc905fac8282604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15050565b600080828401905083811015620005e5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601b8152602001807f536166654d6174683a206164646974696f6e206f766572666c6f77000000000081525060200191505060405180910390fd5b8091505092915050565b610ab680620005ff6000396000f3fe6080604052600436106100555760003560e01c806319165587146100c95780633a98ef391461011a5780638b83209b146101455780639852595c146101c0578063ce7c2ac214610225578063e33b7de31461028a575b7f6ef95f06320e7a25a04a175ca677b7052bdd97131872c2192525a629f51be77061007e6102b5565b34604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a1005b3480156100d557600080fd5b50610118600480360360208110156100ec57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff1690602001909291905050506102bd565b005b34801561012657600080fd5b5061012f610601565b6040518082815260200191505060405180910390f35b34801561015157600080fd5b5061017e6004803603602081101561016857600080fd5b810190808035906020019092919050505061060a565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b3480156101cc57600080fd5b5061020f600480360360208110156101e357600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff16906020019092919050505061064b565b6040518082815260200191505060405180910390f35b34801561023157600080fd5b506102746004803603602081101561024857600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190505050610694565b6040518082815260200191505060405180910390f35b34801561029657600080fd5b5061029f6106dd565b6040518082815260200191505060405180910390f35b600033905090565b6000600260008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205411610355576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526026815260200180610a106026913960400191505060405180910390fd5b60006103836001543073ffffffffffffffffffffffffffffffffffffffff16316106e790919063ffffffff16565b9050600061043e600360008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054610430600054610422600260008973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020548761076f90919063ffffffff16565b6107f590919063ffffffff16565b61083f90919063ffffffff16565b9050600081141561049a576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602b815260200180610a36602b913960400191505060405180910390fd5b6104ec81600360008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546106e790919063ffffffff16565b600360008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550610544816001546106e790919063ffffffff16565b6001819055508273ffffffffffffffffffffffffffffffffffffffff166108fc829081150290604051600060405180830381858888f19350505050158015610590573d6000803e3d6000fd5b507fdf20fd1e76bc69d672e4814fafb2c449bba3a5369d8359adf9e05e6fde87b0568382604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a1505050565b60008054905090565b60006004828154811061061957fe5b9060005260206000200160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050919050565b6000600360008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020549050919050565b6000600260008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020549050919050565b6000600154905090565b600080828401905083811015610765576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601b8152602001807f536166654d6174683a206164646974696f6e206f766572666c6f77000000000081525060200191505060405180910390fd5b8091505092915050565b60008083141561078257600090506107ef565b600082840290508284828161079357fe5b04146107ea576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526021815260200180610a616021913960400191505060405180910390fd5b809150505b92915050565b600061083783836040518060400160405280601a81526020017f536166654d6174683a206469766973696f6e206279207a65726f000000000000815250610889565b905092915050565b600061088183836040518060400160405280601e81526020017f536166654d6174683a207375627472616374696f6e206f766572666c6f77000081525061094f565b905092915050565b60008083118290610935576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825283818151815260200191508051906020019080838360005b838110156108fa5780820151818401526020810190506108df565b50505050905090810190601f1680156109275780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b50600083858161094157fe5b049050809150509392505050565b60008383111582906109fc576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825283818151815260200191508051906020019080838360005b838110156109c15780820151818401526020810190506109a6565b50505050905090810190601f1680156109ee5780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b506000838503905080915050939250505056fe5061796d656e7453706c69747465723a206163636f756e7420686173206e6f207368617265735061796d656e7453706c69747465723a206163636f756e74206973206e6f7420647565207061796d656e74536166654d6174683a206d756c7469706c69636174696f6e206f766572666c6f77a265627a7a72315820ff02ab66fabd418505e584aff862d3307419280eb228522ed63f9feb41a405f464736f6c634300051100325061796d656e7453706c69747465723a206163636f756e7420697320746865207a65726f20616464726573735061796d656e7453706c69747465723a2070617965657320616e6420736861726573206c656e677468206d69736d617463685061796d656e7453706c69747465723a206163636f756e7420616c72656164792068617320736861726573\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
    #[doc = "Returns signature for method `release(address)`."]
    #[allow(clippy::type_complexity)]
    pub fn release(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::Address,), ()> {
      self::ethcontract::contract::Signature::new([25, 22, 85, 135])
    }
    #[doc = "Returns signature for method `totalReleased():(uint256)`."]
    #[allow(clippy::type_complexity)]
    pub fn total_released(
      &self,
    ) -> self::ethcontract::contract::Signature<(), self::ethcontract::U256> {
      self::ethcontract::contract::Signature::new([227, 59, 125, 227])
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
  }
  #[doc = r" Type containing all contract methods for generated contract type."]
  #[derive(Clone)]
  pub struct Methods {
    instance: self::ethcontract::dyns::DynInstance,
  }
  #[allow(clippy::too_many_arguments, clippy::type_complexity)]
  impl Methods {
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
    pub fn total_released(
      &self,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([227, 59, 125, 227], ())
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
    pub fn shares(
      &self,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([206, 124, 42, 194], (account,))
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
