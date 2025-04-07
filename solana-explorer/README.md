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

```json

{
    "transaction": {
        "signatures": [
            [
                14, 77, 249, 178, 2, 126, 86, 44, 172, 140, 8, 55, 79, 179, 40, 7, 99, 230, 140, 171, 162, 231, 15, 178, 4, 100, 204, 222, 138, 224, 47, 236, 216, 79, 202, 190, 64, 233, 123, 92, 59, 186, 35, 41, 95, 103, 32, 214, 88, 58, 170, 146, 221, 215, 255, 220, 130, 103, 5, 218, 152, 161, 37, 1
            ]
        ],
        "message": {
            "header": {
                "num_required_signatures": 1,
                "num_readonly_signed_accounts": 0,
                "num_readonly_unsigned_accounts": 10
            },
            "account_keys": [
                [
                    215, 42, 237, 101, 45, 211, 64, 47, 86, 99, 47, 47, 49, 64, 227, 159, 136, 14, 55, 97, 131, 224, 122, 93, 151, 176, 150, 177, 89, 132, 118, 97
                ],
                [
                    118, 104, 154, 45, 28, 210, 221, 36, 214, 59, 97, 166, 86, 52, 208, 146, 182, 173, 184, 61, 169, 121, 38, 114, 72, 128, 162, 224, 80, 134, 183, 102
                ],
                [
                    99, 131, 115, 0, 14, 162, 44, 178, 100, 211, 74, 255, 100, 160, 75, 94, 250, 191, 187, 116, 221, 205, 4, 137, 151, 177, 152, 21, 71, 215, 209, 16
                ],
                [
                    9, 64, 183, 98, 240, 9, 66, 158, 76, 109, 253, 137, 225, 198, 214, 199, 113, 82, 77, 109, 213, 255, 85, 135, 175, 57, 95, 52, 151, 1, 17, 169
                ],
                [
                    33, 220, 194, 160, 149, 131, 20, 191, 170, 89, 116, 163, 99, 194, 105, 197, 149, 24, 81, 64, 163, 166, 244, 138, 5, 31, 131, 154, 31, 131, 161, 90
                ],
                [
                    172, 245, 71, 68, 54, 21, 36, 67, 51, 116, 101, 23, 254, 193, 73, 132, 119, 93, 6, 12, 35, 36, 19, 214, 180, 121, 141, 231, 140, 207, 142, 42
                ],
                [
                    3, 6, 70, 111, 229, 33, 23, 50, 255, 236, 173, 186, 114, 195, 155, 231, 188, 140, 229, 187, 197, 247, 18, 107, 44, 67, 155, 58, 64, 0, 0, 0
                ],
                [
                    140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131, 11, 90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89
                ],
                [
                    138, 7, 47, 47, 51, 204, 1, 198, 133, 221, 116, 15, 195, 46, 164, 216, 153, 12, 207, 230, 64, 253, 203, 46, 208, 28, 242, 163, 66, 35, 202, 79
                ],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ],
                [
                    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169
                ],
                [
                    147, 255, 101, 178, 97, 182, 55, 73, 102, 240, 94, 144, 36, 11, 175, 34, 5, 40, 206, 17, 131, 161, 228, 74, 203, 17, 97, 222, 156, 23, 30, 154
                ],
                [
                    58, 134, 94, 105, 238, 15, 84, 128, 202, 188, 246, 99, 87, 228, 220, 47, 24, 213, 141, 69, 193, 234, 116, 137, 251, 55, 35, 217, 121, 60, 114, 166
                ],
                [
                    6, 167, 213, 23, 25, 44, 92, 81, 33, 140, 201, 76, 61, 74, 241, 127, 88, 218, 238, 8, 155, 161, 253, 68, 227, 219, 217, 138, 0, 0, 0, 0
                ],
                [
                    172, 241, 54, 235, 1, 252, 28, 78, 136, 61, 35, 200, 181, 132, 74, 181, 154, 55, 246, 106, 221, 87, 197, 233, 172, 59, 83, 224, 89, 211, 92, 100
                ],
                [
                    1, 86, 224, 246, 147, 102, 90, 207, 68, 219, 21, 104, 191, 23, 91, 170, 81, 137, 203, 151, 245, 210, 255, 59, 101, 93, 43, 182, 253, 109, 24, 176
                ]
            ],
            "recent_blockhash": [
                174, 164, 7, 97, 95, 242, 51, 21, 40, 93, 152, 190, 128, 140, 140, 210, 42, 17, 46, 133, 129, 48, 99, 60, 150, 240, 61, 92, 241, 254, 237, 59
            ],
            "instructions": [
                {
                    "program_id_index": 6,
                    "accounts": [],
                    "data": [
                        2, 160, 134, 1, 0
                    ]
                },
                {
                    "program_id_index": 6,
                    "accounts": [],
                    "data": [
                        3, 64, 66, 15, 0, 0, 0, 0, 0
                    ]
                },
                {
                    "program_id_index": 7,
                    "accounts": [
                        0, 1, 0, 8, 9, 10
                    ],
                    "data": []
                },
                {
                    "program_id_index": 11,
                    "accounts": [
                        12, 2, 8, 3, 4, 1, 0, 9, 10, 13, 14, 15
                    ],
                    "data": [
                        0, 64, 220, 179, 17, 0, 0, 0, 0, 119, 162, 136, 54, 61, 8, 0, 0
                    ]
                },
                {
                    "program_id_index": 9,
                    "accounts": [
                        0,5
                    ],
                    "data": [
                        2, 0, 0, 0, 192, 198, 45, 0, 0, 0, 0, 0
                    ]
                }
            ],
            "versioned": false,
            "address_table_lookups": []
        }
    },
    "meta": {
        "err": null,
        "fee": 105000,
        "pre_balances": [
            1219433153, 0, 3130553527145, 871223826, 2039280, 720655012463, 1, 731913600, 1461600, 1, 934087680, 1141440, 286914360, 1009200, 137104014, 1141440
        ],
        "post_balances": [
            917288873, 2039280, 3130556467740, 1165283231, 2039280, 720658012463, 1, 731913600, 1461600, 1, 934087680, 1141440, 286914360, 1009200, 137104014, 1141440
        ],
        "inner_instructions": [
            {
                "index": 2,
                "instructions": [
                    {
                        "program_id_index": 10,
                        "accounts": [
                            8
                        ],
                        "data": [
                            21,
                            7,
                            0
                        ],
                        "stack_height": 2
                    },
                    {
                        "program_id_index": 9,
                        "accounts": [
                            0,
                            1
                        ],
                        "data": [
                            0, 0, 0, 0, 240, 29, 31, 0, 0, 0, 0, 0, 165, 0, 0, 0, 0, 0, 0, 0, 6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169
                        ],
                        "stack_height": 2
                    },
                    {
                        "program_id_index": 10,
                        "accounts": [
                            1
                        ],
                        "data": [
                            22
                        ],
                        "stack_height": 2
                    },
                    {
                        "program_id_index": 10,
                        "accounts": [
                            1,
                            8
                        ],
                        "data": [
                            18, 215, 42, 237, 101, 45, 211, 64, 47, 86, 99, 47, 47, 49, 64, 227, 159, 136, 14, 55, 97, 131, 224, 122, 93, 151, 176, 150, 177, 89, 132, 118, 97
                        ],
                        "stack_height": 2
                    }
                ]
            },
            {
                "index": 3,
                "instructions": [
                    {
                        "program_id_index": 15,
                        "accounts": [
                            12, 2, 8, 3, 4, 1, 0, 9, 10, 13, 14, 15
                        ],
                        "data": [
                            102, 6, 61, 18, 1, 218, 235, 234, 6, 217, 230, 229, 242, 8, 0, 0, 64, 220, 179, 17, 0, 0, 0, 0
                        ],
                        "stack_height": 2
                    },
                    {
                        "program_id_index": 10,
                        "accounts": [
                            4,1,3
                        ],
                        "data": [
                            3, 6, 217, 230, 229, 242, 8, 0, 0
                        ],
                        "stack_height": 3
                    },
                    {
                        "program_id_index": 9,
                        "accounts": [
                            0,
                            3
                        ],
                        "data": [
                            2, 0, 0, 0, 141, 253, 134, 17, 0, 0, 0, 0
                        ],
                        "stack_height": 3
                    },
                    {
                        "program_id_index": 9,
                        "accounts": [
                            0,
                            2
                        ],
                        "data": [
                            2, 0, 0, 0, 179, 222, 44, 0, 0, 0, 0, 0
                        ],
                        "stack_height": 3
                    },
                    {
                        "program_id_index": 15,
                        "accounts": [
                            14
                        ],
                        "data": [
                            228, 69, 165, 46, 81, 203, 154, 29, 189, 219, 127, 211, 78, 230, 97, 238, 138, 7, 47, 47, 51, 204, 1, 198, 133, 221, 116, 15, 195, 46, 164, 216, 153, 12, 207, 230, 64, 253, 203, 46, 208, 28, 242, 163, 66, 35, 202, 79, 141, 253, 134, 17, 0, 0, 0, 0, 6, 217, 230, 229, 242, 8, 0, 0, 1, 215, 42, 237, 101, 45, 211, 64, 47, 86, 99, 47, 47, 49, 64, 227, 159, 136, 14, 55, 97, 131, 224, 122, 93, 151, 176, 150, 177, 89, 132, 118, 97, 188, 55, 213, 103, 0, 0, 0, 0, 111, 175, 133, 65, 7, 0, 0, 0, 120, 162, 119, 163, 111, 171, 3, 0, 111, 3, 98, 69, 0, 0, 0, 0, 120, 10, 101, 87, 222, 172, 2, 0
                        ],
                        "stack_height": 3
                    }
                ]
            }
        ],
        "inner_instructions_none": false,
        "log_messages": [
            "Program ComputeBudget111111111111111111111111111111 invoke [1]",
            "Program ComputeBudget111111111111111111111111111111 success",
            "Program ComputeBudget111111111111111111111111111111 invoke [1]",
            "Program ComputeBudget111111111111111111111111111111 success",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [1]",
            "Program log: Create",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
            "Program log: Instruction: GetAccountDataSize",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1569 of 92833 compute units",
            "Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA pQAAAAAAAAA=",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program 11111111111111111111111111111111 invoke [2]",
            "Program 11111111111111111111111111111111 success",
            "Program log: Initialize the associated token account",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
            "Program log: Instruction: InitializeImmutableOwner",
            "Program log: Please upgrade to SPL Token 2022 for immutable owner support",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1405 of 86246 compute units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
            "Program log: Instruction: InitializeAccount3",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4188 of 82364 compute units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 21807 of 99700 compute units",
            "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
            "Program Axiom3a2w1UbMt2SMgqSvRiuJFTPusDhwKamNgPTeNQ9 invoke [1]",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [2]",
            "Program log: Instruction: Buy",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
            "Program log: Instruction: Transfer",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 50025 compute units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program 11111111111111111111111111111111 invoke [3]",
            "Program 11111111111111111111111111111111 success",
            "Program 11111111111111111111111111111111 invoke [3]",
            "Program 11111111111111111111111111111111 success",
            "Program data: vdt/007mYe6KBy8vM8wBxoXddA/DLqTYmQzP5kD9yy7QHPKjQiPKT439hhEAAAAABtnm5fIIAAAB1yrtZS3TQC9WYy8vMUDjn4gON2GD4Hpdl7CWsVmEdmG8N9VnAAAAAG+vhUEHAAAAeKJ3o2+rAwBvA2JFAAAAAHgKZVferAIA",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [3]",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 2003 of 37689 compute units",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 39333 of 74173 compute units",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success",
            "Program Axiom3a2w1UbMt2SMgqSvRiuJFTPusDhwKamNgPTeNQ9 consumed 43129 of 77893 compute units",
            "Program Axiom3a2w1UbMt2SMgqSvRiuJFTPusDhwKamNgPTeNQ9 success",
            "Program 11111111111111111111111111111111 invoke [1]",
            "Program 11111111111111111111111111111111 success"
        ],
        "log_messages_none": false,
        "pre_token_balances": [
            {
                "account_index": 4,
                "mint": "AHocjgHi5SmF8BhHbKSgsBGpsiXf8i1kpr7HJ65upump",
                "ui_token_amount": {
                    "ui_amount": 969760234.597246,
                    "decimals": 6,
                    "amount": "969760234597246",
                    "ui_amount_string": "969760234.597246"
                },
                "owner": "d7uUaknf8t1jXPADDqyW3gbo28ceCJ4ZwTonf3eVDgt",
                "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            }
        ],
        "post_token_balances": [
            {
                "account_index": 1,
                "mint": "AHocjgHi5SmF8BhHbKSgsBGpsiXf8i1kpr7HJ65upump",
                "ui_token_amount": {
                    "ui_amount": 9839332.219142,
                    "decimals": 6,
                    "amount": "9839332219142",
                    "ui_amount_string": "9839332.219142"
                },
                "owner": "FUvcyEAnzCizErHBW9DqeRY5oFZix2vWF99vvqfMTKhz",
                "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            },
            {
                "account_index": 4,
                "mint": "AHocjgHi5SmF8BhHbKSgsBGpsiXf8i1kpr7HJ65upump",
                "ui_token_amount": {
                    "ui_amount": 959920902.378104,
                    "decimals": 6,
                    "amount": "959920902378104",
                    "ui_amount_string": "959920902.378104"
                },
                "owner": "d7uUaknf8t1jXPADDqyW3gbo28ceCJ4ZwTonf3eVDgt",
                "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            }
        ],
        "rewards": [],
        "loaded_writable_addresses": [],
        "loaded_readonly_addresses": [],
        "return_data": null,
        "return_data_none": false,
        "compute_units_consumed": 65386
    }
}


```

This Solana transaction is a detailed JSON structure that encapsulates various aspects of the transaction, such as signatures, message contents, accounts, instructions, and logs. Let's break down the main components of this JSON:

#### 1.Transaction Structure

**`signatures`**: The first element represents the digital signatures required to authorize the transaction. The array includes a list of bytes which signify the cryptographic signatures of involved accounts. In this case, there is one signature.

**`message`**: This contains the main information about the transaction, including:

- **`header`**: This section defines the number of required signatures and accounts involved.
  - `num_required_signatures`: The number of signatures needed to authorize the transaction (1 in this case).
  - `num_readonly_signed_accounts`: The number of accounts that are read-only and signed.
  - `num_readonly_unsigned_accounts`: The number of accounts that are read-only and unsigned.
- **`account_keys`**: This is a list of account public keys involved in the transaction. Each public key is represented as a byte array.
- **`recent_blockhash`**: A recent blockhash to prevent double-spending and ensure the transaction is included in the current block.
- **`instructions`**: The instructions in the transaction that define actions to be performed by the Solana programs.

#### 2.**Program Execution**

- **`instructions`**: This array contains the individual program instructions that are executed as part of the transaction. Each instruction includes:
  - **`program_id_index`**: An index to the list of programs in the transaction.
  - **`accounts`**: An array of account indices that are required for the instruction.
  - **`data`**: The actual payload of the instruction, typically encoded data for the program.

For example, one of the instructions has:

```json
{
    "program_id_index": 6,
    "accounts": [],
    "data": [2, 160, 134, 1, 0]
}
```

This instruction is referring to a specific program and sending a particular set of data.

#### 3. **Meta Data**

- **`meta`**: This object includes metadata related to the transaction execution, including:

  - **`err`**: This would show any error encountered during the execution. In this case, it's `null`, indicating no error.

  - **`fee`**: The transaction fee paid for executing the transaction, which is `105000` lamports.

  - **`pre_balances`** and **`post_balances`**: These represent the state of accounts before and after the transaction. Each balance corresponds to an account from the `account_keys` array.

  - **`inner_instructions`**: These represent any inner transactions that may have been invoked as part of the execution. Each entry here includes a series of instructions executed during this inner invocation.

  - **`log_messages`**: Logs generated during the execution of the transaction, which can provide insight into what happened during the transaction.

For example:

```json
"Program log: Instruction: GetAccountDataSize"
```

#### 4. **Token Balances**

- **`pre_token_balances`** and **`post_token_balances`**: These show the token balances of accounts involved in the transaction before and after execution. These are token-specific balances, typically for SPL tokens (Solana's standard for fungible tokens).
- For instance:

```

{
    "account_index": 4,
    "mint": "AHocjgHi5SmF8BhHbKSgsBGpsiXf8i1kpr7HJ65upump",
    "ui_token_amount": {
        "ui_amount": 969760234.597246,
        "decimals": 6,
        "amount": "969760234597246"
    },
    "owner": "d7uUaknf8t1jXPADDqyW3gbo28ceCJ4ZwTonf3eVDgt",
    "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
}
```

#### 5.**Compute Units**

- **`compute_units_consumed`**: This indicates the total compute units consumed by the transaction. Compute units are a resource used to limit the amount of processing time a transaction can take on Solana. In this case, the transaction consumed `65386` compute units.

#### 6. **Instructions Breakdown**

Each instruction in the transaction might involve different Solana programs. For example, the instructions involve different programs like:

- **`TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`**: Likely related to SPL Token actions.
- **`ComputeBudget111111111111111111111111111111111111`**: A program for managing compute resources.
- **`ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`**: Another custom program involved in the transaction.



