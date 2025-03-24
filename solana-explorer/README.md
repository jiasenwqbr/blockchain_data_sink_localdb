# Solana Explorer

The Solana Explorer consists of several Substreams modules showcasing the most basic operations that you can perform with Substreams on the Solana blockchain.

## Before You Begin

Make sure you have the [Substreams CLI installed](https://substreams.streamingfast.io/getting-started/installing-the-cli), and you know the [basic structure of a Substreams module](https://substreams.streamingfast.io/documentation/intro-getting-started/intro-solana). You must also provide an [authentication token](https://substreams.streamingfast.io/documentation/consume/authentication).

## Modules

## Running the Substreams

First, generate the Protobuf code, which are the outputs of the Substreams:

```
> make protogen
```

Then, build the Rust code using the `cargo` command-line tool:

```
> make build
```

### Running the "map_block_meta" Module

In the following command, you retrieve the metadata of the block with slot number `153000000`. You specify the starting block by using the `--start-block` parameter.

```bash
> substreams gui -e mainnet.sol.streamingfast.io:443 ./solana-explorer-v0.1.0.spkg map_block_meta --start-block 153000000 --stop-block +1
Connected (trace ID 0c93ceb65b98b138f0b8caf8a0e81c77)
Progress messages received: 0 (0/sec)
Backprocessing history up to requested target block 153000000:
(hit 'm' to switch mode)

----------- BLOCK #153,000,000 (ENZNjBKCxXQqgFxX63T6kn7DJwc7fQiwy3W3vyehtEGA) ---------------
{
  "@module": "map_block_meta",
  "@block": 153000000,
  "@type": "sol.block_meta.v1.BlockMeta",
  "@data": {
    "slot": "153000000",
    "hash": "ENZNjBKCxXQqgFxX63T6kn7DJwc7fQiwy3W3vyehtEGA",
    "parentHash": "7Eric2Heq94zoz5zBf5xg3K7opnWDaRVoS6DVa4Xxtcn"
  }
}

all done
```

### Running the "map_filter_instructions" Module

To run this module, you must provide a program id value in the parameters in your substreams.yaml manifest.

This example will use the default `Stake11111111111111111111111111111111111111` program.

```yaml
params:
  map_filter_instructions: "program_id=Stake11111111111111111111111111111111111111"
```

```bash
> substreams run -e mainnet.sol.streamingfast.io:443 ./solana-explorer-v0.1.0.spkg map_filter_instructions -s 153000000 -t +50
----------- BLOCK #153,000,000 (ENZNjBKCxXQqgFxX63T6kn7DJwc7fQiwy3W3vyehtEGA) ---------------
----------- BLOCK #153,000,001 (8q2n7y3ozKWUWrAGVbNc3wrkxdmT6iSdgf8CHgqrvSv1) ---------------
----------- BLOCK #153,000,002 (AfE4vBBsvcCXzdexP4sX9byHYrdZsUpd2GTTTTvkShxu) ---------------
----------- BLOCK #153,000,003 (H5RZGAShGm7sMW6NvaPY3TsX36WEvugZ64Y1gSBRPyLV) ---------------
----------- BLOCK #153,000,004 (Egf4PFv5636rycdntsBfi26Bou2Mt3Z5UAgH4hBbkZat) ---------------
----------- BLOCK #153,000,005 (DmkkhPG2RDWkJnLbSiG1XTUotJVpLbjqBeRa9FoQB3Tg) ---------------
----------- BLOCK #153,000,006 (Dy9YQQrTed69j7fsy7z3F3sa1x34T4CGmCVQVFoWobRE) ---------------
----------- BLOCK #153,000,007 (3bHscyCgsvzLo6u4CpvCzJe7ELkuf9k8MSwHyLUtif9L) ---------------
----------- BLOCK #153,000,008 (CkDjN7c4hjYqtuYXH9Vw8MTmk29XjgMnbEtAzrjbyjnU) ---------------
----------- BLOCK #153,000,009 (8SVnGrwbwpAWCPdRXKgyNCvJRedEqmYnLazB3mB7Rehg) ---------------
----------- BLOCK #153,000,010 (BTVDWyLK26P2CBjGjCvsHwNqaUcn15xUauwCJmkCSAx1) ---------------
----------- BLOCK #153,000,011 (DVzw63xEAHmWTdo9iA9ca7yTiaondmGGvr4VczQnFhbn) ---------------
----------- BLOCK #153,000,012 (EtMvzAbrkdWpawfP7ozcL86yocFNYxvWvtS4f8pMrajD) ---------------
{
  "@module": "map_filter_instructions",
  "@block": 153000012,
  "@type": "sol.programs.v1.Instructions",
  "@data": {
    "instructions": [
      {
        "programId": "Stake11111111111111111111111111111111111111",
        "accounts": [
          "4nicrkdB15JqjMViy8guLFi8x1RLqTQud37XGX5LZVkN",
          "SysvarRent111111111111111111111111111111111"
        ],
        "data": "1111jgvBfvfPuc2GLifVk9vJpXFTL5urtkjvyJHP6p7TqNR9G47jkYe3RGab9rya96yeGycaY8SVbcpaBkRLZonTJL7y4GyDWwyEmYmvNz74HWew5cPsky18ppTT1WyqmjxvmznzUWXfRogakvUscqJ8C54tP",
      },
      {
        "programId": "Stake11111111111111111111111111111111111111",
        "accounts": [
          "4nicrkdB15JqjMViy8guLFi8x1RLqTQud37XGX5LZVkN",
          "6F5xdRXh2W3B2vhte12VG79JVUkUSLYrHydGX1SAadfZ",
          "SysvarC1ock11111111111111111111111111111111",
          "SysvarStakeHistory1111111111111111111111111",
          "StakeConfig11111111111111111111111111111111",
          "GJYanGsBPyRUgAYkXBTyBYdZiH5wppXoJys9HaJwitzk"
        ],
        "data": "3xyZh",
      }
    ]
  }
}

----------- BLOCK #153,000,013 (7NVSAZZefTrNkK7JSg227yoponB6Tig27PXnEm7kQFJf) ---------------
----------- BLOCK #153,000,014 (3nfE81F85ggdWxjRZUENBPhvXStAJsVxnwrQ99VcTMME) ---------------
----------- BLOCK #153,000,015 (4N3sqbbJEZobfu1pL6rZEZf6Kpx2WYB8fHm1hbqASa6S) ---------------
----------- BLOCK #153,000,016 (812uX2UUSvvep5APemSmTDk4bDnMkZnJtMBSa6enBmFN) ---------------
----------- BLOCK #153,000,017 (CXsKxq7LmJVW6P3YjBVbrfgDiyMyzYDX5jNhgdsryMfv) ---------------
----------- BLOCK #153,000,018 (FnVqabMm8VEKCr9pchMFxipxSssNeioxx7bCaqyGbzgX) ---------------
----------- BLOCK #153,000,019 (J49iZE979hGtK4hnxUigpxQiNsU3oNq9UMMJqknGLJVT) ---------------
----------- BLOCK #153,000,020 (GJ1BNqsBtyxucvqkipkZDaHW2KGdsU3sR41ebmCxth24) ---------------
----------- BLOCK #153,000,021 (Bxc99wYm1heyzfY1ViXNnJFmf81BFHzLKrwizW5TPemn) ---------------
----------- BLOCK #153,000,022 (2iLweM1VHWnkmXpF7G2fHr52zGuwsEjFXd8ZtpwwUcDw) ---------------
----------- BLOCK #153,000,023 (6JpfNdC2qTErHpXd8e74E5aAkm11USA2rJJbCvQdtHyg) ---------------
----------- BLOCK #153,000,024 (AjN8rk6c8ya5TNYRBUWKUgEYEJceqtJsPMMDdweg4MPR) ---------------
----------- BLOCK #153,000,025 (F7dvT2mwGusrLmRwdJrGZ7cAXsxq43kHsrKpzeseywwg) ---------------
----------- BLOCK #153,000,026 (aGdXha9hhhaETuNywV2XY39Y6B719NBZ6xbUzAu3wah) ---------------
----------- BLOCK #153,000,027 (GfoPvdzekjLV1QHvvHzyVG7xbjajDKA4Ev7gXum2Ry1S) ---------------
----------- BLOCK #153,000,028 (5xj9iYo6nuAiXvPFDUieUKBcRLbg9rz5qiBQg2NYjyBv) ---------------
{
  "@module": "map_filter_instructions",
  "@block": 153000028,
  "@type": "sol.programs.v1.Instructions",
  "@data": {
    "instructions": [
      {
        "programId": "Stake11111111111111111111111111111111111111",
        "accounts": [
          "EQFsbzpgBufUVPV65tt3awmnv1E8anaWfNSBCfnf4bUh",
          "SysvarC1ock11111111111111111111111111111111",
          "4sUsH6ZzqWU2g1xzM8udFErB4qN3GSL1QbNjiSHc3QxN"
        ],
        "data": "8QwQj",
      }
    ]
  }
}

----------- BLOCK #153,000,029 (3LBYvzhWfdtcAwuCvoBzTCkUbpVbckyBFAcJezdScGhP) ---------------
----------- BLOCK #153,000,030 (B97KXhFadGBEkMjZ6FTyK6dP6Lyk79p2CqgQBYmjhSdf) ---------------
----------- BLOCK #153,000,031 (C2SgWr1LvVrG3N37DtvN3rd5mQisswU87PfSD6pyLdae) ---------------
----------- BLOCK #153,000,032 (4buy43g7PDLmR196MtRDDid1ZfWbqxJbTi2RWt7gvjqn) ---------------
----------- BLOCK #153,000,033 (C7RAFX27t2cpPyVWXbCACccQAjyHxoyyP2GbKmy7to3y) ---------------
----------- BLOCK #153,000,034 (CyigpnTa5q9j6vLMFpBCNyEUjViojVQcwicb89DAntq6) ---------------
----------- BLOCK #153,000,035 (8npojbiPbBFau8GuXtbYnn6JfBa2tkzfmv6uKEwAKhJ) ---------------
----------- BLOCK #153,000,036 (AAhLqu8Z8REBTMGh5FcMuWyWANBiVpf65XP6PWXG11rx) ---------------
----------- BLOCK #153,000,037 (8AYAF1uiVtnC94c99wmzZP7gRAs7oMFcvE99dtXaNYFL) ---------------
----------- BLOCK #153,000,038 (3aopsHdWuQLT4X11x4bo1e6Vcyyou9HB8wVV7hbMphvi) ---------------
----------- BLOCK #153,000,039 (8HpRgRVePiSpTUqRJBsnvSF3XMFQCyfy2STrTCqTCt2J) ---------------
----------- BLOCK #153,000,040 (AhfUVwbX1SinS8hXzfgSM1ESkrU9CV6aMvTQgch9nZhY) ---------------
----------- BLOCK #153,000,041 (A6JVr9HVkqQL1mvqGLzDuKvbsf9PFQffvS4PnaDNa44F) ---------------
----------- BLOCK #153,000,042 (FMqcoisPGNatsBt3eWmiWJuX1cLGAu3qLAy9696RNp4s) ---------------
----------- BLOCK #153,000,043 (47ARP75kLw3m6s3yWcUWmjsGB4avyNmspgFq6UkxYQde) ---------------
----------- BLOCK #153,000,044 (DzDjDvU1vXCHBeoYwEPZKy3VGByVAM6DkQEKZop3Z8Wn) ---------------
----------- BLOCK #153,000,045 (CrgFgZFxVCSqXUfysNe7GXQ71oTK8eGUVaM9FvsPG81C) ---------------
----------- BLOCK #153,000,046 (4bNCMpe2E3W1zMxHCNmKYooGe2ptTo1EMe92qRKQYMP5) ---------------
----------- BLOCK #153,000,047 (6MqMSHpKjwavQp4FRs5wAGEbKqkp8D82EH8KFkF2YXyH) ---------------
----------- BLOCK #153,000,048 (2E8ncPZMAqa5RKPvEUAMt6abLNcDZM7J6nawyaiSboGD) ---------------
----------- BLOCK #153,000,049 (GyG9Rw4Mo8sYb9WLzz8oqHgV3k4sYa3UWeveGG2uLGWX) ---------------
all done
```

### Running the "map_filter_transactions" Module

To run this module, you must provide a transaction signature value in the parameters in your substreams.yaml manifest.

This example will use the default `21ED2HBGuLUwgbaBb77cGwFR8MkVQfjR9KszzCb7jZkeSysJkHAVew6RaaBh3r1zTefpdq9Kf5geFp19P3nUXB3t` program.

```yaml
params:
  map_filter_transactions: "signature=21ED2HBGuLUwgbaBb77cGwFR8MkVQfjR9KszzCb7jZkeSysJkHAVew6RaaBh3r1zTefpdq9Kf5geFp19P3nUXB3t"
```

```bash
substreams run -e mainnet.sol.streamingfast.io:443 ./solana-explorer-v0.1.0.spkg map_filter_transactions -s 153000000 -t +30
Connected (trace ID 27d4c57714280d824e0cd37f7cd8dc17)
Progress messages received: 0 (0/sec)
Backprocessing history up to requested target block 153000028:
(hit 'm' to switch mode)
----------- BLOCK #153,000,000 (ENZNjBKCxXQqgFxX63T6kn7DJwc7fQiwy3W3vyehtEGA) ---------------
----------- BLOCK #153,000,001 (8q2n7y3ozKWUWrAGVbNc3wrkxdmT6iSdgf8CHgqrvSv1) ---------------
----------- BLOCK #153,000,002 (AfE4vBBsvcCXzdexP4sX9byHYrdZsUpd2GTTTTvkShxu) ---------------
----------- BLOCK #153,000,003 (H5RZGAShGm7sMW6NvaPY3TsX36WEvugZ64Y1gSBRPyLV) ---------------
----------- BLOCK #153,000,004 (Egf4PFv5636rycdntsBfi26Bou2Mt3Z5UAgH4hBbkZat) ---------------
----------- BLOCK #153,000,005 (DmkkhPG2RDWkJnLbSiG1XTUotJVpLbjqBeRa9FoQB3Tg) ---------------
----------- BLOCK #153,000,006 (Dy9YQQrTed69j7fsy7z3F3sa1x34T4CGmCVQVFoWobRE) ---------------
----------- BLOCK #153,000,007 (3bHscyCgsvzLo6u4CpvCzJe7ELkuf9k8MSwHyLUtif9L) ---------------
----------- BLOCK #153,000,008 (CkDjN7c4hjYqtuYXH9Vw8MTmk29XjgMnbEtAzrjbyjnU) ---------------
----------- BLOCK #153,000,009 (8SVnGrwbwpAWCPdRXKgyNCvJRedEqmYnLazB3mB7Rehg) ---------------
----------- BLOCK #153,000,010 (BTVDWyLK26P2CBjGjCvsHwNqaUcn15xUauwCJmkCSAx1) ---------------
----------- BLOCK #153,000,011 (DVzw63xEAHmWTdo9iA9ca7yTiaondmGGvr4VczQnFhbn) ---------------
----------- BLOCK #153,000,012 (EtMvzAbrkdWpawfP7ozcL86yocFNYxvWvtS4f8pMrajD) ---------------
----------- BLOCK #153,000,013 (7NVSAZZefTrNkK7JSg227yoponB6Tig27PXnEm7kQFJf) ---------------
----------- BLOCK #153,000,014 (3nfE81F85ggdWxjRZUENBPhvXStAJsVxnwrQ99VcTMME) ---------------
----------- BLOCK #153,000,015 (4N3sqbbJEZobfu1pL6rZEZf6Kpx2WYB8fHm1hbqASa6S) ---------------
----------- BLOCK #153,000,016 (812uX2UUSvvep5APemSmTDk4bDnMkZnJtMBSa6enBmFN) ---------------
----------- BLOCK #153,000,017 (CXsKxq7LmJVW6P3YjBVbrfgDiyMyzYDX5jNhgdsryMfv) ---------------
----------- BLOCK #153,000,018 (FnVqabMm8VEKCr9pchMFxipxSssNeioxx7bCaqyGbzgX) ---------------
----------- BLOCK #153,000,019 (J49iZE979hGtK4hnxUigpxQiNsU3oNq9UMMJqknGLJVT) ---------------
----------- BLOCK #153,000,020 (GJ1BNqsBtyxucvqkipkZDaHW2KGdsU3sR41ebmCxth24) ---------------
----------- BLOCK #153,000,021 (Bxc99wYm1heyzfY1ViXNnJFmf81BFHzLKrwizW5TPemn) ---------------
----------- BLOCK #153,000,022 (2iLweM1VHWnkmXpF7G2fHr52zGuwsEjFXd8ZtpwwUcDw) ---------------
----------- BLOCK #153,000,023 (6JpfNdC2qTErHpXd8e74E5aAkm11USA2rJJbCvQdtHyg) ---------------
----------- BLOCK #153,000,024 (AjN8rk6c8ya5TNYRBUWKUgEYEJceqtJsPMMDdweg4MPR) ---------------
----------- BLOCK #153,000,025 (F7dvT2mwGusrLmRwdJrGZ7cAXsxq43kHsrKpzeseywwg) ---------------
----------- BLOCK #153,000,026 (aGdXha9hhhaETuNywV2XY39Y6B719NBZ6xbUzAu3wah) ---------------
----------- BLOCK #153,000,027 (GfoPvdzekjLV1QHvvHzyVG7xbjajDKA4Ev7gXum2Ry1S) ---------------
----------- BLOCK #153,000,028 (5xj9iYo6nuAiXvPFDUieUKBcRLbg9rz5qiBQg2NYjyBv) ---------------
{
  "@module": "map_filter_transactions",
  "@block": 153000028,
  "@type": "sol.transactions.v1.Transactions",
  "@data": {
    "transactions": [
      {
        "signatures": [
          "21ED2HBGuLUwgbaBb77cGwFR8MkVQfjR9KszzCb7jZkeSysJkHAVew6RaaBh3r1zTefpdq9Kf5geFp19P3nUXB3t"
        ],
        "instructions": [
          {
            "programId": "Vote111111111111111111111111111111111111111",
            "accounts": [
              "HRfK8kbqCaKYsHk3R8HCtLNDp4aTneq1eSQ9ZrA2Kb2q",
              "SysvarS1otHashes111111111111111111111111111",
              "SysvarC1ock11111111111111111111111111111111",
              "HEJzPiLSg9ty5CGAyJm5y7ef1NPzr1u1aHLYaAMXsgH9"
            ],
            "data": "29z5mr1JoRmJYQ6yybpMZZDfJB6dWDGQsB8jimJo3eTNYA7qEJvjo3dwvmP25dQtpxkKPQdEy37P5hDAMoAtGnaE1vGpXq"
          }
        ]
      }
    ]
  }
}

----------- BLOCK #153,000,029 (3LBYvzhWfdtcAwuCvoBzTCkUbpVbckyBFAcJezdScGhP) ---------------
all done
```

## block data structure

### header

#### 

```json

{
    "previous_blockhash": "H71qFzUREou1PU8vnU89EDFciouGnxfjesCGvApKSpow",
    "blockhash": "2YxKFHjcr2hL8FfZKXp5ehSRFRbH56wxtZCoa6u29dyx",
    "parent_slot": 326880937,
    "transactions":[]
    "rewards": [
        {
            "pubkey": "7QGeaLDAhDdrLHZxFb27xL6GMoVGZ5oJTk7ULpgici1M",
            "lamports": 10746330,
            "post_balance": 8992919977,
            "reward_type": 1,
            "commission": ""
        }
    ],
    "block_time": {
        "timestamp": 1742026684
    },
    "block_height": {
        "block_height": 305139705
    },
    "slot": 326880938
}

```



#### 1.previous_blockhash

```json
"previous_blockhash": "H71qFzUREou1PU8vnU89EDFciouGnxfjesCGvApKSpow"
```



类型：字符串（String）

含义：当前区块的前一个区块的哈希值。这个字段用于确保区块链的连贯性，每个区块都会包含前一个区块的哈希，从而形成区块链。

#### 2.blockhash

```json
"blockhash": "2YxKFHjcr2hL8FfZKXp5ehSRFRbH56wxtZCoa6u29dyx"
```
类型：字符串（String）

含义：当前区块的哈希值。这个哈希值是根据区块内容计算得到的，唯一标识当前区块。

#### 3.parent_slot

```json
"parent_slot": 326880937
```

**类型**：整数（u64）

**含义**：表示当前区块的父槽（slot）。在区块链中，槽（slot）通常用于表示区块的时间或顺序标识。`parent_slot` 表示这个区块的父级区块所在的槽。

#### **4. `transactions`**

```json

"transactions": []
复制编辑
```

**类型**：数组（Array）

**含义**：当前区块中的交易列表。这个字段列出了所有在该区块中处理的交易。

#### 5.`rewards`

```json
"rewards": [
    {
        "pubkey": "7QGeaLDAhDdrLHZxFb27xL6GMoVGZ5oJTk7ULpgici1M",
        "lamports": 10746330,
        "post_balance": 8992919977,
        "reward_type": 1,
        "commission": ""
    }
]
```

**类型**：数组（Array）

**含义**：奖励信息。每个奖励项表示给一个账户分配的奖励。包括以下字段：

- `pubkey`：奖励的接收账户的公钥，表示该账户在区块链中的唯一标识符。
- `lamports`：奖励的数量，以 Lamports 为单位。Lamport 是 Solana 网络中的最小单位，类似于以太坊中的 Wei。
- `post_balance`：奖励发放后，账户的余额。
- `reward_type`：奖励类型。不同的数字可能代表不同类型的奖励，通常 `1` 代表一种特定类型的奖励（例如费用奖励、租金奖励等）。
- `commission`：奖励的佣金，这里为空字符串，可能是一个可选字段。

**常见的 `reward_type` 值**

**`1` - Fee Reward (费用奖励)**

- 这是最常见的奖励类型，表示用户或验证者因区块链上的交易手续费而获得的奖励。

**`2` - Rent Reward (租金奖励)**

- 这个奖励类型通常表示账户因持有资金而获得的租金奖励。Solana 使用这种奖励类型来激励账户持有者保留一定数量的资金。

**`3` - Staking Reward (质押奖励)**

- 该奖励类型表示用户参与了质押（staking）并因此获得的奖励。Solana 网络允许用户通过将其代币质押到验证者节点来获得奖励。

**`4` - Voting Reward (投票奖励)**

- 验证者参与投票并且投票正确时，可能会获得的奖励。Solana 验证者通过参与网络共识和投票，帮助确保网络的稳定性和安全性。

**`5` - Inflation Reward (通胀奖励)**

- 这个奖励类型表示随着通货膨胀而产生的奖励，通常与 Solana 的通货膨胀机制相关。网络中的新生成的代币会被分发给那些参与质押和验证的账户。

#### **6. `block_time`**

```json

"block_time": {
    "timestamp": 1742026684
}
```

- **类型**：对象（Object）
- **含义**：区块的时间戳。时间戳是一个整数，表示自 Unix 纪元以来的秒数。这里的时间戳 `1742026684` 代表了区块的创建时间。

#### **7. `block_height`**

```json
"block_height": {
    "block_height": 305139705
}
```

- **类型**：对象（Object）
- **含义**：区块的高度，表示区块链中该区块的顺序编号。`block_height` 字段的值是当前区块在区块链中的编号。

#### **8. `slot`**

```json
"slot": 326880938
```

- **类型**：整数（u64）
- **含义**：区块的槽位。Solana 网络中使用槽（slot）来标识区块。每个槽位通常表示一个时间段或某个区块的位置。



### Transactions





