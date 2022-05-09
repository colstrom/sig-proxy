#[allow(dead_code)]
pub mod safe_cast_mock {
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
          # [allow (unused_mut)] let mut contract = TruffleLoader :: new () . load_contract_from_str ("{\"contractName\":\"SafeCastMock\",\"abi\":[{\"type\":\"function\",\"name\":\"toUint16\",\"inputs\":[{\"name\":\"a\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint16\"}],\"constant\":true,\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"toUint128\",\"inputs\":[{\"name\":\"a\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint128\"}],\"constant\":true,\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"toUint32\",\"inputs\":[{\"name\":\"a\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\"}],\"constant\":true,\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"toUint8\",\"inputs\":[{\"name\":\"a\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\"}],\"constant\":true,\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"toUint64\",\"inputs\":[{\"name\":\"a\",\"type\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint64\"}],\"constant\":true,\"stateMutability\":\"pure\"}],\"bytecode\":\"608060405234801561001057600080fd5b50610553806100206000396000f3fe608060405234801561001057600080fd5b50600436106100575760003560e01c80630cc4681e1461005c5780632665fad0146100a4578063809fdd33146100fa5780639374068f14610160578063c8193255146101aa575b600080fd5b6100886004803603602081101561007257600080fd5b81019080803590602001909291905050506101f8565b604051808260ff1660ff16815260200191505060405180910390f35b6100d0600480360360208110156100ba57600080fd5b810190808035906020019092919050505061020a565b604051808267ffffffffffffffff1667ffffffffffffffff16815260200191505060405180910390f35b6101266004803603602081101561011057600080fd5b810190808035906020019092919050505061021c565b60405180826fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b61018c6004803603602081101561017657600080fd5b810190808035906020019092919050505061022e565b604051808261ffff1661ffff16815260200191505060405180910390f35b6101d6600480360360208110156101c057600080fd5b8101908080359060200190929190505050610240565b604051808263ffffffff1663ffffffff16815260200191505060405180910390f35b600061020382610252565b9050919050565b6000610215826102b6565b9050919050565b600061022782610321565b9050919050565b600061023982610394565b9050919050565b600061024b826103f9565b9050919050565b600061010082106102ae576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260258152602001806104876025913960400191505060405180910390fd5b819050919050565b6000680100000000000000008210610319576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260268152602001806104d36026913960400191505060405180910390fd5b819050919050565b6000700100000000000000000000000000000000821061038c576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260278152602001806104ac6027913960400191505060405180910390fd5b819050919050565b60006201000082106103f1576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260268152602001806104616026913960400191505060405180910390fd5b819050919050565b60006401000000008210610458576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260268152602001806104f96026913960400191505060405180910390fd5b81905091905056fe53616665436173743a2076616c756520646f65736e27742066697420696e203136206269747353616665436173743a2076616c756520646f65736e27742066697420696e2038206269747353616665436173743a2076616c756520646f65736e27742066697420696e20313238206269747353616665436173743a2076616c756520646f65736e27742066697420696e203634206269747353616665436173743a2076616c756520646f65736e27742066697420696e2033322062697473a265627a7a723158204ebc26ebd5f14823ea9db1acf95700cf51d75d36a7d0b4149c518f4c6414a67d64736f6c63430005110032\",\"networks\":{},\"devdoc\":{\"details\":null,\"methods\":{}},\"userdoc\":{\"details\":null,\"methods\":{}}}") . expect ("valid contract JSON") ;
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
      f.debug_tuple(stringify!(SafeCastMock))
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
    #[doc = "Returns signature for method `toUint16(uint256):(uint16)`."]
    #[allow(clippy::type_complexity)]
    pub fn to_uint_16(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::U256,), u16> {
      self::ethcontract::contract::Signature::new([147, 116, 6, 143])
    }
    #[doc = "Returns signature for method `toUint128(uint256):(uint128)`."]
    #[allow(clippy::type_complexity)]
    pub fn to_uint_128(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::U256,), u128> {
      self::ethcontract::contract::Signature::new([128, 159, 221, 51])
    }
    #[doc = "Returns signature for method `toUint32(uint256):(uint32)`."]
    #[allow(clippy::type_complexity)]
    pub fn to_uint_32(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::U256,), u32> {
      self::ethcontract::contract::Signature::new([200, 25, 50, 85])
    }
    #[doc = "Returns signature for method `toUint8(uint256):(uint8)`."]
    #[allow(clippy::type_complexity)]
    pub fn to_uint_8(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::U256,), u8> {
      self::ethcontract::contract::Signature::new([12, 196, 104, 30])
    }
    #[doc = "Returns signature for method `toUint64(uint256):(uint64)`."]
    #[allow(clippy::type_complexity)]
    pub fn to_uint_64(
      &self,
    ) -> self::ethcontract::contract::Signature<(self::ethcontract::U256,), u64> {
      self::ethcontract::contract::Signature::new([38, 101, 250, 208])
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
    pub fn to_uint_16(
      &self,
      a: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<u16> {
      self
        .instance
        .view_method([147, 116, 6, 143], (a,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn to_uint_128(
      &self,
      a: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<u128> {
      self
        .instance
        .view_method([128, 159, 221, 51], (a,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn to_uint_32(
      &self,
      a: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<u32> {
      self
        .instance
        .view_method([200, 25, 50, 85], (a,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn to_uint_8(
      &self,
      a: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<u8> {
      self
        .instance
        .view_method([12, 196, 104, 30], (a,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn to_uint_64(
      &self,
      a: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<u64> {
      self
        .instance
        .view_method([38, 101, 250, 208], (a,))
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
pub use self::safe_cast_mock::Contract as SafeCastMock;
