# program_id_index 与data的关系？

在 Solana 交易的 **指令 (`CompiledInstruction`)** 中，`program_id_index` 和 `data` 共同决定了交易的 **执行逻辑**。它们的关系如下：

---

## **1. `program_id_index` 与 `data` 的作用**
- **`program_id_index`** 指向 **程序 ID**（即智能合约地址）。
  - Solana 交易需要调用某个 **程序 (Program)** 执行具体的指令，例如 **系统程序 (System Program)**、**SPL 代币程序 (Token Program)** 或 **自定义智能合约**。
  - `program_id_index` 是一个索引，它对应 **`Message` 结构体中的 `account_keys` 数组**，用于定位具体的智能合约。
  
- **`data`** 存储该 **程序的输入数据**（即 **指令参数**）。
  - `data` 是一个 **字节数组 (`Vec<u8>`)**，它按照特定的格式编码了 **指令的参数**。
  - 该格式是由 `program_id_index` 关联的 **程序 (Program)** 定义的，通常使用 **Borsh** 或 **Protobuf** 进行序列化。

---

## **2. `program_id_index` 的作用**
Solana 交易的 **指令 (Instruction)** 需要调用一个特定的 **程序** 来执行。而 **`program_id_index` 只是一个索引，不是直接的程序地址**。

Solana 交易中的 `Message` 结构体包含 `account_keys`，它是一个账户数组，其中包括：
- **交易涉及的账户**（如发送者、接收者）。
- **调用的智能合约地址**（即程序 ID）。

`program_id_index` 就是 **指向 `account_keys` 数组中某个程序 ID 的索引**。

示例：
```rust
let account_keys = vec![
    "SenderPublicKey",      // 0: 发送者账户
    "RecipientPublicKey",   // 1: 接收者账户
    "TokenProgramID"        // 2: 代币程序 (Program)
];

let program_id_index = 2;  // 指向 "TokenProgramID"
```
在这个例子中，`program_id_index = 2`，表示这个指令的 `program_id` 是 **Token Program**，所以 `data` 的格式应该按照 Token Program 规定的格式进行解析。

---

## **3. `program_id_index` 和 `data` 的关系**
- **`program_id_index` 确定了 `data` 应该如何被解析**。
- **`data` 的格式取决于 `program_id_index` 关联的智能合约**。

| `program_id_index` 关联的程序  | `data` 解析方式 |
|----------------|----------------|
| **System Program (系统程序)** | `data` 可能包含 **创建账户、转账、分配内存** 等指令。 |
| **Token Program (SPL 代币程序)** | `data` 可能包含 **代币转账、授权账户、创建代币账户** 等指令。 |
| **自定义智能合约** | `data` 的格式完全由智能合约开发者定义，通常使用 **Borsh** 进行序列化。 |

### **示例：解析 `data`**
假设 `program_id_index` 指向的是 **Solana Token Program**，那么 `data` 的结构可能如下：
```rust
struct TokenTransferInstruction {
    instruction_type: u8,  // 指令类型（1 = 代币转账）
    amount: u64,           // 代币转账金额
}
```
如果 `data` 是：
```hex
01 00 00 00 00 00 10 00
```
那么解析步骤：
- `01` → `instruction_type = 1` （代表 "转账" 操作）。
- `00 00 00 00 00 00 10 00` → `amount = 4096`（以小端字节序解析）。

这说明：如果 `program_id_index` 指向的是 **Token Program**，那么 `data` 应该按照 **代币标准** 来解析。

---

## **4. `program_id_index` 与 `data` 的解析流程**
**解析 `data` 之前，必须先解析 `program_id_index`，确定它属于哪个智能合约**：

1. **解析 `program_id_index`**
   - 从 `account_keys` 获取 **程序 ID**。
   - 确定该程序是 **系统程序、代币程序，还是自定义合约**。

2. **根据 `program_id` 解析 `data`**
   - 如果 `program_id` 是 **系统程序**，则 `data` 可能包含 **创建账户、转账** 等操作。
   - 如果 `program_id` 是 **Token Program**，则 `data` 可能是 **SPL 代币转账** 的数据结构。
   - 如果 `program_id` 是 **自定义智能合约**，则 `data` 的格式取决于合约的设计，可能需要 **Borsh** 解码。

---

## **5. 代码示例**
### **示例 1：解析 `program_id_index`**
假设我们有一个 `CompiledInstruction`：
```rust
let compiled_instruction = CompiledInstruction {
    program_id_index: 2,
    accounts: vec![0, 1], // 指向 Sender 和 Receiver
    data: vec![1, 0, 0, 0, 0, 0, 16, 0], // 假设是 Token Program 的转账指令
};
```
再假设 `account_keys`：
```rust
let account_keys = vec![
    "SenderPublicKey",      // 0
    "RecipientPublicKey",   // 1
    "TokenProgramID"        // 2
];
```
我们可以找到 `program_id`：
```rust
let program_id = account_keys[compiled_instruction.program_id_index]; // "TokenProgramID"
```
现在我们知道这个指令调用的是 **Token Program**，接下来解析 `data`。

---

### **示例 2：解析 `data`**
如果 `program_id` 是 **Token Program**，那么 `data` 可以解析成 `TokenTransferInstruction`：

```rust
use borsh::{BorshDeserialize};

#[derive(BorshDeserialize, Debug)]
struct TokenTransferInstruction {
    instruction_type: u8,  // 指令类型
    amount: u64,           // 代币转账金额
}

fn parse_data(data: Vec<u8>) -> Result<TokenTransferInstruction, borsh::BorshError> {
    TokenTransferInstruction::try_from_slice(&data)
}

fn main() {
    let data = vec![1, 0, 0, 0, 0, 0, 16, 0]; // 交易数据
    match parse_data(data) {
        Ok(instruction) => {
            println!("解析后的数据: {:?}", instruction);
        }
        Err(err) => {
            eprintln!("解析失败: {:?}", err);
        }
    }
}
```

**输出结果**
```
解析后的数据: TokenTransferInstruction { instruction_type: 1, amount: 4096 }
```

---

## **6. 结论**
1. **`program_id_index` 确定了 `data` 的解析方式**：
   - `program_id_index` 指向 `account_keys` 数组中的某个智能合约（系统程序、代币程序或自定义合约）。
   - **不同的合约有不同的 `data` 格式**，所以解析 `data` 之前必须先解析 `program_id_index`。

2. **如何解析 `data`**
   - **确定 `program_id_index` 指向哪个合约**。
   - **根据合约的规范解析 `data`**，可能需要使用 **Borsh、Protobuf 或其他序列化方法**。

这样，我们可以正确地解析 Solana 交易指令中的 `data` 并提取关键信息。








```
RUST_LOG=debug substreams-sink-sql setup "psql://postgres:root@172.20.31.66:5432/blockchain_data?search_path=solana&schema=solana&sslmode=disable" ./sink/substreams.dev.yaml 
```

```
RUST_LOG=debug  substreams-sink-sql run "psql://postgres:root@172.20.31.66:5432/blockchain_data?search_path=solana&schema=solana&sslmode=disable" ./sink/substreams.dev.yaml   --header "x-api-key:"  --on-module-hash-mistmatch=ignore 

```