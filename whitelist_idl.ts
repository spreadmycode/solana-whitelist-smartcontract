export const WHITELIST_IDL = 
{
  "version": "0.0.0",
  "name": "whitelist",
  "instructions": [
    {
      "name": "initializeContract",
      "accounts": [
        {
          "name": "data",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "whitelistData",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
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
      "name": "addWhitelists",
      "accounts": [
        {
          "name": "data",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelistData",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minter",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "addresses",
          "type": {
            "vec": "publicKey"
          }
        }
      ]
    },
    {
      "name": "clearWhitelist",
      "accounts": [
        {
          "name": "data",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelistData",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minter",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "setPending",
      "accounts": [
        {
          "name": "data",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelistData",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minter",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "togglePeriod",
      "accounts": [
        {
          "name": "data",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelistData",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "minter",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "Data",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "periodStatus",
            "type": "u8"
          },
          {
            "name": "counter",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "WhitelistData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "data",
            "type": {
              "array": [
                "publicKey",
                3000
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "PeriodStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "PendingSale"
          },
          {
            "name": "PreSale"
          },
          {
            "name": "PostSale"
          }
        ]
      }
    }
  ]
};
