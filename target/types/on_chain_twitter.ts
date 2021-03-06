export type OnChainTwitter = {
  "version": "0.1.0",
  "name": "on_chain_twitter",
  "instructions": [
    {
      "name": "createTwitterAccount",
      "accounts": [
        {
          "name": "author",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "username",
          "type": "string"
        }
      ]
    },
    {
      "name": "changeUserName",
      "accounts": [
        {
          "name": "author",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "username",
          "type": "string"
        }
      ]
    },
    {
      "name": "transferOwnershipUserAccount",
      "accounts": [
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "author",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "newOwner",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "getNumberOfTweetsByUser",
      "accounts": [
        {
          "name": "author",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "twitterUser",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": "u8"
    },
    {
      "name": "deleteTwitterAccount",
      "accounts": [
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "author",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "sendTweet",
      "accounts": [
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tweet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "author",
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
          "name": "topic",
          "type": "string"
        },
        {
          "name": "content",
          "type": "string"
        }
      ]
    },
    {
      "name": "updateTweet",
      "accounts": [
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tweet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "author",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "topic",
          "type": "string"
        },
        {
          "name": "content",
          "type": "string"
        }
      ]
    },
    {
      "name": "deleteTweet",
      "accounts": [
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tweet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "author",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "updateNextAddress",
      "accounts": [
        {
          "name": "author",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nextAddress",
          "type": "publicKey"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "tweet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "publicKey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          },
          {
            "name": "topic",
            "type": "string"
          },
          {
            "name": "content",
            "type": "string"
          },
          {
            "name": "author",
            "type": "publicKey"
          },
          {
            "name": "tweetNumber",
            "type": "u8"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                10
              ]
            }
          }
        ]
      }
    },
    {
      "name": "twitterUser",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "author",
            "type": "publicKey"
          },
          {
            "name": "lastInteractionTimestamp",
            "type": "i64"
          },
          {
            "name": "nextAddress",
            "type": "publicKey"
          },
          {
            "name": "username",
            "type": "string"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "tweetCount",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                10
              ]
            }
          }
        ]
      }
    }
  ]
};

export const IDL: OnChainTwitter = {
  "version": "0.1.0",
  "name": "on_chain_twitter",
  "instructions": [
    {
      "name": "createTwitterAccount",
      "accounts": [
        {
          "name": "author",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "username",
          "type": "string"
        }
      ]
    },
    {
      "name": "changeUserName",
      "accounts": [
        {
          "name": "author",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "username",
          "type": "string"
        }
      ]
    },
    {
      "name": "transferOwnershipUserAccount",
      "accounts": [
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "author",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "newOwner",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "getNumberOfTweetsByUser",
      "accounts": [
        {
          "name": "author",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "twitterUser",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": "u8"
    },
    {
      "name": "deleteTwitterAccount",
      "accounts": [
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "author",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "sendTweet",
      "accounts": [
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tweet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "author",
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
          "name": "topic",
          "type": "string"
        },
        {
          "name": "content",
          "type": "string"
        }
      ]
    },
    {
      "name": "updateTweet",
      "accounts": [
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tweet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "author",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "topic",
          "type": "string"
        },
        {
          "name": "content",
          "type": "string"
        }
      ]
    },
    {
      "name": "deleteTweet",
      "accounts": [
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tweet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "author",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "updateNextAddress",
      "accounts": [
        {
          "name": "author",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "twitterUser",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nextAddress",
          "type": "publicKey"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "tweet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "publicKey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          },
          {
            "name": "topic",
            "type": "string"
          },
          {
            "name": "content",
            "type": "string"
          },
          {
            "name": "author",
            "type": "publicKey"
          },
          {
            "name": "tweetNumber",
            "type": "u8"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                10
              ]
            }
          }
        ]
      }
    },
    {
      "name": "twitterUser",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "author",
            "type": "publicKey"
          },
          {
            "name": "lastInteractionTimestamp",
            "type": "i64"
          },
          {
            "name": "nextAddress",
            "type": "publicKey"
          },
          {
            "name": "username",
            "type": "string"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "tweetCount",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                10
              ]
            }
          }
        ]
      }
    }
  ]
};
