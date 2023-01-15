export type StakingMinigame = {
  "version": "0.1.0",
  "name": "staking_minigame",
  "instructions": [
    {
      "name": "initAdmin",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "programData",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "admin",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "setAdmin",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "programData",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "admin",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "initPools",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeProceeds",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "closePools",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "addWhitelist",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "referenceAccount",
          "type": "publicKey"
        },
        {
          "name": "whitelistType",
          "type": "u8"
        },
        {
          "name": "faction",
          "type": "u8"
        }
      ]
    },
    {
      "name": "removeWhitelist",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "referenceAccount",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "addCharacter",
      "accounts": [
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadata",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "removeCharacter",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMetadata",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "setMineParameters",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "locked",
          "type": "bool"
        },
        {
          "name": "fee",
          "type": "u64"
        },
        {
          "name": "rate",
          "type": "u64"
        },
        {
          "name": "warmup",
          "type": "i64"
        },
        {
          "name": "cooldown",
          "type": "i64"
        },
        {
          "name": "boost",
          "type": "u16"
        },
        {
          "name": "rewardMinFraction",
          "type": "u16"
        },
        {
          "name": "rewardPrecision",
          "type": "u16"
        }
      ]
    },
    {
      "name": "updateMine",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "mintMineRewards",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "setLootParameters",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "locked",
          "type": "bool"
        },
        {
          "name": "fee",
          "type": "u64"
        },
        {
          "name": "duration",
          "type": "u64"
        },
        {
          "name": "warmup",
          "type": "i64"
        },
        {
          "name": "cooldown",
          "type": "i64"
        },
        {
          "name": "boost",
          "type": "u16"
        },
        {
          "name": "rewardProbability",
          "type": "u16"
        },
        {
          "name": "rewardPrecision",
          "type": "u16"
        }
      ]
    },
    {
      "name": "fundLoot",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateLoot",
      "accounts": [
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "stakeMine",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMetadata",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeProceeds",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimMine",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "unstakeMine",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "forceUnstakeMine",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECKED: implicity checked against the NFT holder"
          ]
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "stakeLoot",
      "accounts": [
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMetadata",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeProceeds",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimLoot",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "unstakeLoot",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "forceUnstakeLoot",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECKED: implicity checked against the NFT holder"
          ]
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "adminSettings",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "admin",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "character",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "rewardsAccrued",
            "type": "u128"
          },
          {
            "name": "rewardsSecured",
            "type": "u128"
          },
          {
            "name": "staked",
            "type": "u8"
          },
          {
            "name": "stakedPeg",
            "type": "u128"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "loot",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "feeMint",
            "type": "publicKey"
          },
          {
            "name": "feeProceeds",
            "type": "publicKey"
          },
          {
            "name": "fund",
            "type": "u128"
          },
          {
            "name": "rate",
            "type": "u64"
          },
          {
            "name": "stakedCharacters",
            "type": "u16"
          },
          {
            "name": "accruedRewards",
            "type": "u128"
          },
          {
            "name": "accruedTimestamp",
            "type": "i64"
          },
          {
            "name": "fundingTimestamp",
            "type": "i64"
          },
          {
            "name": "locked",
            "type": "bool"
          },
          {
            "name": "fee",
            "type": "u64"
          },
          {
            "name": "duration",
            "type": "u64"
          },
          {
            "name": "warmup",
            "type": "i64"
          },
          {
            "name": "cooldown",
            "type": "i64"
          },
          {
            "name": "boost",
            "type": "u16"
          },
          {
            "name": "rewardProbability",
            "type": "u16"
          },
          {
            "name": "rewardPrecision",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "mine",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "stakedCharacters",
            "type": "u16"
          },
          {
            "name": "accruedRewards",
            "type": "u128"
          },
          {
            "name": "accruedTimestamp",
            "type": "i64"
          },
          {
            "name": "locked",
            "type": "bool"
          },
          {
            "name": "feeMint",
            "type": "publicKey"
          },
          {
            "name": "feeProceeds",
            "type": "publicKey"
          },
          {
            "name": "fee",
            "type": "u64"
          },
          {
            "name": "rate",
            "type": "u64"
          },
          {
            "name": "warmup",
            "type": "i64"
          },
          {
            "name": "cooldown",
            "type": "i64"
          },
          {
            "name": "boost",
            "type": "u16"
          },
          {
            "name": "rewardMinFraction",
            "type": "u16"
          },
          {
            "name": "rewardPrecision",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "whitelist",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "whitelistType",
            "type": "u8"
          },
          {
            "name": "faction",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "ManagerOffCurve",
      "msg": "Manager account is not on-curve"
    },
    {
      "code": 6001,
      "name": "InvalidComputation",
      "msg": "Invalid computation"
    },
    {
      "code": 6002,
      "name": "InvalidMetadataAccount",
      "msg": "The supplied metadata account is not valid"
    },
    {
      "code": 6003,
      "name": "InvalidEditionAccount",
      "msg": "The supplied edition account is not valid"
    },
    {
      "code": 6004,
      "name": "InvalidWhitelistProof",
      "msg": "The supplied whitelist proof account is not valid"
    },
    {
      "code": 6005,
      "name": "InvalidWhitelistType",
      "msg": "The supplied whitelist type is invalid"
    },
    {
      "code": 6006,
      "name": "InvalidParameters",
      "msg": "Invalid parameters"
    },
    {
      "code": 6007,
      "name": "StakingPoolLocked",
      "msg": "The requested staking pool is currently locked"
    },
    {
      "code": 6008,
      "name": "NotHolder",
      "msg": "The user is not the current holder of the NFT"
    },
    {
      "code": 6009,
      "name": "AlreadyStaked",
      "msg": "The NFT is already staked"
    },
    {
      "code": 6010,
      "name": "NotStaked",
      "msg": "The NFT is not staked"
    },
    {
      "code": 6011,
      "name": "InvalidFaction",
      "msg": "The NFT does not belong to the requested faction"
    },
    {
      "code": 6012,
      "name": "Warmup",
      "msg": "The NFT warmup period has not yet elapsed"
    },
    {
      "code": 6013,
      "name": "Cooldown",
      "msg": "The NFT cooldown period has not yet elapsed"
    },
    {
      "code": 6014,
      "name": "PoolNotEmpty",
      "msg": "Can not close pool while tokens are still staked"
    }
  ]
};

export const IDL: StakingMinigame = {
  "version": "0.1.0",
  "name": "staking_minigame",
  "instructions": [
    {
      "name": "initAdmin",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "programData",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "admin",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "setAdmin",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "programData",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "admin",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "initPools",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeProceeds",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "closePools",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "addWhitelist",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "referenceAccount",
          "type": "publicKey"
        },
        {
          "name": "whitelistType",
          "type": "u8"
        },
        {
          "name": "faction",
          "type": "u8"
        }
      ]
    },
    {
      "name": "removeWhitelist",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "referenceAccount",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "addCharacter",
      "accounts": [
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftMetadata",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "removeCharacter",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMetadata",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "setMineParameters",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "locked",
          "type": "bool"
        },
        {
          "name": "fee",
          "type": "u64"
        },
        {
          "name": "rate",
          "type": "u64"
        },
        {
          "name": "warmup",
          "type": "i64"
        },
        {
          "name": "cooldown",
          "type": "i64"
        },
        {
          "name": "boost",
          "type": "u16"
        },
        {
          "name": "rewardMinFraction",
          "type": "u16"
        },
        {
          "name": "rewardPrecision",
          "type": "u16"
        }
      ]
    },
    {
      "name": "updateMine",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "mintMineRewards",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "setLootParameters",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "locked",
          "type": "bool"
        },
        {
          "name": "fee",
          "type": "u64"
        },
        {
          "name": "duration",
          "type": "u64"
        },
        {
          "name": "warmup",
          "type": "i64"
        },
        {
          "name": "cooldown",
          "type": "i64"
        },
        {
          "name": "boost",
          "type": "u16"
        },
        {
          "name": "rewardProbability",
          "type": "u16"
        },
        {
          "name": "rewardPrecision",
          "type": "u16"
        }
      ]
    },
    {
      "name": "fundLoot",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateLoot",
      "accounts": [
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "stakeMine",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMetadata",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeProceeds",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimMine",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "unstakeMine",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "forceUnstakeMine",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECKED: implicity checked against the NFT holder"
          ]
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "stakeLoot",
      "accounts": [
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMetadata",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeProceeds",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feeAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "claimLoot",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "unstakeLoot",
      "accounts": [
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "forceUnstakeLoot",
      "accounts": [
        {
          "name": "adminSettings",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mine",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "loot",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "character",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nftEdition",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: custom logic checks for the validity of this account"
          ]
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rewardAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rewardMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECKED: implicity checked against the NFT holder"
          ]
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: This is not dangerous because we don't read or write from this account"
          ]
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "recentSlothashes",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECKED: account constraints checked in account trait"
          ]
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "adminSettings",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "admin",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "character",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "rewardsAccrued",
            "type": "u128"
          },
          {
            "name": "rewardsSecured",
            "type": "u128"
          },
          {
            "name": "staked",
            "type": "u8"
          },
          {
            "name": "stakedPeg",
            "type": "u128"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "loot",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "feeMint",
            "type": "publicKey"
          },
          {
            "name": "feeProceeds",
            "type": "publicKey"
          },
          {
            "name": "fund",
            "type": "u128"
          },
          {
            "name": "rate",
            "type": "u64"
          },
          {
            "name": "stakedCharacters",
            "type": "u16"
          },
          {
            "name": "accruedRewards",
            "type": "u128"
          },
          {
            "name": "accruedTimestamp",
            "type": "i64"
          },
          {
            "name": "fundingTimestamp",
            "type": "i64"
          },
          {
            "name": "locked",
            "type": "bool"
          },
          {
            "name": "fee",
            "type": "u64"
          },
          {
            "name": "duration",
            "type": "u64"
          },
          {
            "name": "warmup",
            "type": "i64"
          },
          {
            "name": "cooldown",
            "type": "i64"
          },
          {
            "name": "boost",
            "type": "u16"
          },
          {
            "name": "rewardProbability",
            "type": "u16"
          },
          {
            "name": "rewardPrecision",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "mine",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "stakedCharacters",
            "type": "u16"
          },
          {
            "name": "accruedRewards",
            "type": "u128"
          },
          {
            "name": "accruedTimestamp",
            "type": "i64"
          },
          {
            "name": "locked",
            "type": "bool"
          },
          {
            "name": "feeMint",
            "type": "publicKey"
          },
          {
            "name": "feeProceeds",
            "type": "publicKey"
          },
          {
            "name": "fee",
            "type": "u64"
          },
          {
            "name": "rate",
            "type": "u64"
          },
          {
            "name": "warmup",
            "type": "i64"
          },
          {
            "name": "cooldown",
            "type": "i64"
          },
          {
            "name": "boost",
            "type": "u16"
          },
          {
            "name": "rewardMinFraction",
            "type": "u16"
          },
          {
            "name": "rewardPrecision",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "whitelist",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "whitelistType",
            "type": "u8"
          },
          {
            "name": "faction",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "ManagerOffCurve",
      "msg": "Manager account is not on-curve"
    },
    {
      "code": 6001,
      "name": "InvalidComputation",
      "msg": "Invalid computation"
    },
    {
      "code": 6002,
      "name": "InvalidMetadataAccount",
      "msg": "The supplied metadata account is not valid"
    },
    {
      "code": 6003,
      "name": "InvalidEditionAccount",
      "msg": "The supplied edition account is not valid"
    },
    {
      "code": 6004,
      "name": "InvalidWhitelistProof",
      "msg": "The supplied whitelist proof account is not valid"
    },
    {
      "code": 6005,
      "name": "InvalidWhitelistType",
      "msg": "The supplied whitelist type is invalid"
    },
    {
      "code": 6006,
      "name": "InvalidParameters",
      "msg": "Invalid parameters"
    },
    {
      "code": 6007,
      "name": "StakingPoolLocked",
      "msg": "The requested staking pool is currently locked"
    },
    {
      "code": 6008,
      "name": "NotHolder",
      "msg": "The user is not the current holder of the NFT"
    },
    {
      "code": 6009,
      "name": "AlreadyStaked",
      "msg": "The NFT is already staked"
    },
    {
      "code": 6010,
      "name": "NotStaked",
      "msg": "The NFT is not staked"
    },
    {
      "code": 6011,
      "name": "InvalidFaction",
      "msg": "The NFT does not belong to the requested faction"
    },
    {
      "code": 6012,
      "name": "Warmup",
      "msg": "The NFT warmup period has not yet elapsed"
    },
    {
      "code": 6013,
      "name": "Cooldown",
      "msg": "The NFT cooldown period has not yet elapsed"
    },
    {
      "code": 6014,
      "name": "PoolNotEmpty",
      "msg": "Can not close pool while tokens are still staked"
    }
  ]
};
