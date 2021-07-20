#[allow(dead_code)]
pub mod ierc1155_upgradeable {
  # [rustfmt :: skip] use ethcontract as ethcontract ;
  #[doc = "Generated by `ethcontract`"]
  #[derive(Clone)]
  pub struct Contract {
    methods: Methods,
  }
  impl Contract {
    #[doc = r" Retrieves the truffle artifact used to generate the type safe"]
    #[doc = r" API for this contract."]
    pub fn artifact() -> &'static self::ethcontract::Artifact {
      use self::ethcontract::private::lazy_static;
      use self::ethcontract::Artifact;
      lazy_static! {
        pub static ref ARTIFACT: Artifact = {
          # [allow (unused_mut)] let mut artifact = Artifact :: from_json ("{\n  \"_format\": \"hh-sol-artifact-1\",\n  \"contractName\": \"IERC1155Upgradeable\",\n  \"sourceName\": \"contracts/token/ERC1155/IERC1155Upgradeable.sol\",\n  \"abi\": [\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"operator\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"approved\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"ApprovalForAll\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"operator\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256[]\",\n          \"name\": \"ids\",\n          \"type\": \"uint256[]\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256[]\",\n          \"name\": \"values\",\n          \"type\": \"uint256[]\"\n        }\n      ],\n      \"name\": \"TransferBatch\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"operator\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"id\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"value\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"TransferSingle\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"string\",\n          \"name\": \"value\",\n          \"type\": \"string\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"uint256\",\n          \"name\": \"id\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"URI\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"id\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"balanceOf\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address[]\",\n          \"name\": \"accounts\",\n          \"type\": \"address[]\"\n        },\n        {\n          \"internalType\": \"uint256[]\",\n          \"name\": \"ids\",\n          \"type\": \"uint256[]\"\n        }\n      ],\n      \"name\": \"balanceOfBatch\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256[]\",\n          \"name\": \"\",\n          \"type\": \"uint256[]\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"operator\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"isApprovedForAll\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256[]\",\n          \"name\": \"ids\",\n          \"type\": \"uint256[]\"\n        },\n        {\n          \"internalType\": \"uint256[]\",\n          \"name\": \"amounts\",\n          \"type\": \"uint256[]\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"data\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"safeBatchTransferFrom\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"id\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"data\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"safeTransferFrom\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"operator\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"approved\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"setApprovalForAll\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"interfaceId\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"name\": \"supportsInterface\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    }\n  ],\n  \"bytecode\": \"0x\",\n  \"deployedBytecode\": \"0x\",\n  \"linkReferences\": {},\n  \"deployedLinkReferences\": {}\n}\n") . expect ("valid artifact JSON") ;
          artifact
        };
      }
      &ARTIFACT
    }
    #[doc = r" Creates a new contract instance with the specified `web3`"]
    #[doc = r" provider at the given `Address`."]
    #[doc = r""]
    #[doc = r" Note that this does not verify that a contract with a matching"]
    #[doc = r" `Abi` is actually deployed at the given address."]
    pub fn at<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      address: self::ethcontract::Address,
    ) -> Self
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F> + Send + Sync + 'static,
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
    pub fn with_deployment_info<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      address: self::ethcontract::Address,
      deployment_information: Option<ethcontract::common::DeploymentInformation>,
    ) -> Self
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F> + Send + Sync + 'static,
    {
      use self::ethcontract::transport::DynTransport;
      use self::ethcontract::web3::api::Web3;
      use self::ethcontract::Instance;
      let transport = DynTransport::new(web3.transport().clone());
      let web3 = Web3::new(transport);
      let abi = Self::artifact().abi.clone();
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
      f.debug_tuple(stringify!(IERC1155Upgradeable))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = r" Retrieves a reference to type containing all the generated"]
    #[doc = r" contract methods. This can be used for methods where the name"]
    #[doc = r" would collide with a common method (like `at` or `deployed`)."]
    pub fn methods(&self) -> &Methods {
      &self.methods
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
    pub fn safe_batch_transfer_from(
      &self,
      from: self::ethcontract::Address,
      to: self::ethcontract::Address,
      ids: Vec<self::ethcontract::U256>,
      amounts: Vec<self::ethcontract::U256>,
      data: self::ethcontract::tokens::Bytes<Vec<u8>>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([46, 178, 194, 214], (from, to, ids, amounts, data))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn set_approval_for_all(
      &self,
      operator: self::ethcontract::Address,
      approved: bool,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([162, 44, 180, 101], (operator, approved))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn balance_of(
      &self,
      account: self::ethcontract::Address,
      id: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .view_method([0, 253, 213, 142], (account, id))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn safe_transfer_from(
      &self,
      from: self::ethcontract::Address,
      to: self::ethcontract::Address,
      id: self::ethcontract::U256,
      amount: self::ethcontract::U256,
      data: self::ethcontract::tokens::Bytes<Vec<u8>>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([242, 66, 67, 42], (from, to, id, amount, data))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn balance_of_batch(
      &self,
      accounts: Vec<self::ethcontract::Address>,
      ids: Vec<self::ethcontract::U256>,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<Vec<self::ethcontract::U256>> {
      self
        .instance
        .view_method([78, 18, 115, 244], (accounts, ids))
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
    #[doc = "Generated by `ethcontract`"]
    pub fn is_approved_for_all(
      &self,
      account: self::ethcontract::Address,
      operator: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<bool> {
      self
        .instance
        .view_method([233, 133, 233, 197], (account, operator))
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
    pub struct Uri {
      pub value: String,
      pub id: self::ethcontract::U256,
    }
    impl Uri {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          107, 183, 255, 112, 134, 25, 186, 6, 16, 203, 162, 149, 165, 133, 146, 224, 69, 29, 238,
          38, 34, 147, 140, 135, 85, 102, 118, 136, 218, 243, 82, 155,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`URI(string,uint256)`"]
      pub fn abi_signature() -> &'static str {
        "URI(string,uint256)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for Uri {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (value, id) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(Uri { value, id })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct ApprovalForAll {
      pub account: self::ethcontract::Address,
      pub operator: self::ethcontract::Address,
      pub approved: bool,
    }
    impl ApprovalForAll {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          23, 48, 126, 171, 57, 171, 97, 7, 232, 137, 152, 69, 173, 61, 89, 189, 150, 83, 242, 0,
          242, 32, 146, 4, 137, 202, 43, 89, 55, 105, 108, 49,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`ApprovalForAll(address,address,bool)`"]
      pub fn abi_signature() -> &'static str {
        "ApprovalForAll(address,address,bool)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for ApprovalForAll {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (account, operator, approved) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(ApprovalForAll {
          account,
          operator,
          approved,
        })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct TransferBatch {
      pub operator: self::ethcontract::Address,
      pub from: self::ethcontract::Address,
      pub to: self::ethcontract::Address,
      pub ids: Vec<self::ethcontract::U256>,
      pub values: Vec<self::ethcontract::U256>,
    }
    impl TransferBatch {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          74, 57, 220, 6, 212, 192, 219, 198, 75, 112, 175, 144, 253, 105, 138, 35, 58, 81, 138,
          165, 208, 126, 89, 93, 152, 59, 140, 5, 38, 200, 247, 251,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`TransferBatch(address,address,address,uint256[],uint256[])`"]
      pub fn abi_signature() -> &'static str {
        "TransferBatch(address,address,address,uint256[],uint256[])"
      }
    }
    impl self::ethcontract::tokens::Tokenize for TransferBatch {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (operator, from, to, ids, values) =
          self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(TransferBatch {
          operator,
          from,
          to,
          ids,
          values,
        })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct TransferSingle {
      pub operator: self::ethcontract::Address,
      pub from: self::ethcontract::Address,
      pub to: self::ethcontract::Address,
      pub id: self::ethcontract::U256,
      pub value: self::ethcontract::U256,
    }
    impl TransferSingle {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          195, 213, 129, 104, 197, 174, 115, 151, 115, 29, 6, 61, 91, 191, 61, 101, 120, 84, 66,
          115, 67, 244, 192, 131, 36, 15, 122, 172, 170, 45, 15, 98,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`TransferSingle(address,address,address,uint256,uint256)`"]
      pub fn abi_signature() -> &'static str {
        "TransferSingle(address,address,address,uint256,uint256)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for TransferSingle {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (operator, from, to, id, value) =
          self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(TransferSingle {
          operator,
          from,
          to,
          id,
          value,
        })
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
    pub fn uri(&self) -> self::event_builders::UriBuilder {
      self::event_builders::UriBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            107, 183, 255, 112, 134, 25, 186, 6, 16, 203, 162, 149, 165, 133, 146, 224, 69, 29,
            238, 38, 34, 147, 140, 135, 85, 102, 118, 136, 218, 243, 82, 155,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn approval_for_all(&self) -> self::event_builders::ApprovalForAllBuilder {
      self::event_builders::ApprovalForAllBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            23, 48, 126, 171, 57, 171, 97, 7, 232, 137, 152, 69, 173, 61, 89, 189, 150, 83, 242, 0,
            242, 32, 146, 4, 137, 202, 43, 89, 55, 105, 108, 49,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn transfer_batch(&self) -> self::event_builders::TransferBatchBuilder {
      self::event_builders::TransferBatchBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            74, 57, 220, 6, 212, 192, 219, 198, 75, 112, 175, 144, 253, 105, 138, 35, 58, 81, 138,
            165, 208, 126, 89, 93, 152, 59, 140, 5, 38, 200, 247, 251,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn transfer_single(&self) -> self::event_builders::TransferSingleBuilder {
      self::event_builders::TransferSingleBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            195, 213, 129, 104, 197, 174, 115, 151, 115, 29, 6, 61, 91, 191, 61, 101, 120, 84, 66,
            115, 67, 244, 192, 131, 36, 15, 122, 172, 170, 45, 15, 98,
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
    #[doc = "A builder for creating a filtered stream of `Uri` events."]
    pub struct UriBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::Uri>,
    );
    impl UriBuilder {
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
      #[doc = r" Limit the number of events that can be retrieved by this filter."]
      #[doc = r""]
      #[doc = r" Note that this parameter is non-standard."]
      pub fn limit(mut self, value: usize) -> Self {
        self.0 = (self.0).limit(value);
        self
      }
      #[doc = r" The polling interval. This is used as the interval between"]
      #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
      pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
        self.0 = (self.0).poll_interval(value);
        self
      }
      #[doc = "Adds a filter for the id event parameter."]
      pub fn id(mut self, topic: self::ethcontract::Topic<self::ethcontract::U256>) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::Uri>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::Uri>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `ApprovalForAll` events."]
    pub struct ApprovalForAllBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::ApprovalForAll>,
    );
    impl ApprovalForAllBuilder {
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
      #[doc = r" Limit the number of events that can be retrieved by this filter."]
      #[doc = r""]
      #[doc = r" Note that this parameter is non-standard."]
      pub fn limit(mut self, value: usize) -> Self {
        self.0 = (self.0).limit(value);
        self
      }
      #[doc = r" The polling interval. This is used as the interval between"]
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
      #[doc = "Adds a filter for the operator event parameter."]
      pub fn operator(
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
        std::vec::Vec<self::ethcontract::Event<self::event_data::ApprovalForAll>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::ApprovalForAll>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `TransferBatch` events."]
    pub struct TransferBatchBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::TransferBatch>,
    );
    impl TransferBatchBuilder {
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
      #[doc = r" Limit the number of events that can be retrieved by this filter."]
      #[doc = r""]
      #[doc = r" Note that this parameter is non-standard."]
      pub fn limit(mut self, value: usize) -> Self {
        self.0 = (self.0).limit(value);
        self
      }
      #[doc = r" The polling interval. This is used as the interval between"]
      #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
      pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
        self.0 = (self.0).poll_interval(value);
        self
      }
      #[doc = "Adds a filter for the operator event parameter."]
      pub fn operator(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = "Adds a filter for the from event parameter."]
      pub fn from(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic1(topic);
        self
      }
      #[doc = "Adds a filter for the to event parameter."]
      pub fn to(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic2(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::TransferBatch>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::TransferBatch>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `TransferSingle` events."]
    pub struct TransferSingleBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::TransferSingle>,
    );
    impl TransferSingleBuilder {
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
      #[doc = r" Limit the number of events that can be retrieved by this filter."]
      #[doc = r""]
      #[doc = r" Note that this parameter is non-standard."]
      pub fn limit(mut self, value: usize) -> Self {
        self.0 = (self.0).limit(value);
        self
      }
      #[doc = r" The polling interval. This is used as the interval between"]
      #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
      pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
        self.0 = (self.0).poll_interval(value);
        self
      }
      #[doc = "Adds a filter for the operator event parameter."]
      pub fn operator(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = "Adds a filter for the from event parameter."]
      pub fn from(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic1(topic);
        self
      }
      #[doc = "Adds a filter for the to event parameter."]
      pub fn to(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic2(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::TransferSingle>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::TransferSingle>,
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
    ApprovalForAll(self::event_data::ApprovalForAll),
    TransferBatch(self::event_data::TransferBatch),
    TransferSingle(self::event_data::TransferSingle),
    Uri(self::event_data::Uri),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([23 , 48 , 126 , 171 , 57 , 171 , 97 , 7 , 232 , 137 , 152 , 69 , 173 , 61 , 89 , 189 , 150 , 83 , 242 , 0 , 242 , 32 , 146 , 4 , 137 , 202 , 43 , 89 , 55 , 105 , 108 , 49]) => Ok (Event :: ApprovalForAll (log . clone () . decode (Contract :: artifact () . abi . event ("ApprovalForAll") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([74 , 57 , 220 , 6 , 212 , 192 , 219 , 198 , 75 , 112 , 175 , 144 , 253 , 105 , 138 , 35 , 58 , 81 , 138 , 165 , 208 , 126 , 89 , 93 , 152 , 59 , 140 , 5 , 38 , 200 , 247 , 251]) => Ok (Event :: TransferBatch (log . clone () . decode (Contract :: artifact () . abi . event ("TransferBatch") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([195 , 213 , 129 , 104 , 197 , 174 , 115 , 151 , 115 , 29 , 6 , 61 , 91 , 191 , 61 , 101 , 120 , 84 , 66 , 115 , 67 , 244 , 192 , 131 , 36 , 15 , 122 , 172 , 170 , 45 , 15 , 98]) => Ok (Event :: TransferSingle (log . clone () . decode (Contract :: artifact () . abi . event ("TransferSingle") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([107 , 183 , 255 , 112 , 134 , 25 , 186 , 6 , 16 , 203 , 162 , 149 , 165 , 133 , 146 , 224 , 69 , 29 , 238 , 38 , 34 , 147 , 140 , 135 , 85 , 102 , 118 , 136 , 218 , 243 , 82 , 155]) => Ok (Event :: Uri (log . clone () . decode (Contract :: artifact () . abi . event ("URI") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::ierc1155_upgradeable::Contract as IERC1155Upgradeable;
