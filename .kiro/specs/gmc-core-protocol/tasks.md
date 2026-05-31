# Implementation Plan: GMC 核心协议（gmc-core-protocol）

## Overview

本计划将设计文档（9 个协议模块、数据模型、4 条关键流程、30 条正确性属性）拆解为增量式编码任务。实现策略：

- **纯逻辑核心优先**：先实现一个语言无关的确定性逻辑核心 crate `gmc-core`（派生树、评分数学、配额核算、衰减模型、投票者选取等），这些纯逻辑承载 Property 1–30 的属性测试。
- **基础设施后置**：Substrate L1 pallet 与 ZK Rollup L2 作为核心逻辑的外层包装（锚定、批量证明、分片、共识），在核心逻辑稳定后集成。
- **测试驱动**：每条正确性属性（Property 1–30）由**单一** proptest 属性测试实现，标签格式为 `Feature: gmc-core-protocol, Property N: ...`，每条测试至少运行 100 次随机迭代；单元/示例测试覆盖具体行为与错误条件；集成/冒烟测试覆盖跨层与一次性配置。

> 技术栈：默认采用 **Rust（Substrate 生态）+ proptest**（蓝图推荐方向）。保留 **TypeScript 参考模型 + fast-check** 作为备选方案；任务 1.1 显式记录该决策。带 `*` 的子任务为可选测试任务，可在 MVP 阶段跳过。

## Tasks

- [x] 1. 项目脚手架与测试框架
  - [x] 1.1 建立技术栈骨架与工作区
    - 创建 Rust workspace 与纯逻辑核心 crate `crates/gmc-core`（可被 Substrate pallet 与 L2 复用，不依赖链运行时）
    - 在 crate 根注释 / 文档中记录技术栈决策：默认 Substrate L1 + ZK Rollup L2 + proptest；保留 TypeScript 参考模型 + fast-check 备选方向
    - 建立 `src/lib.rs` 模块骨架占位：`registry / mechanism / quota / scoring / merit / minting / registration / recording / retroactive / antifraud / governance / carbon / gmc_base`
    - _Requirements: 5.2_
  - [x] 1.2 定义核心共享类型与统一错误码
    - 在 `src/types.rs` 定义 `Decimal` / `Ratio`（[0,1] 定点小数）、`ChainId` / `FayID` / `Timestamp`、`DimensionWeights` 等基础类型，全部金额/比例使用定点小数避免浮点误差
    - 在 `src/error.rs` 定义统一可机器辨识错误码枚举（`ParentNotFound`、`CycleConflict`、`DepthExceeded`、`MissingField`、`DomainConflict`、`MechanismConfigInvalid`、`QuotaExceeded`、`QuotaConfigInvalid`、`DimensionUnmatched`、`WeightSumInvalid`、`InflationIndexOutOfRange`、`InvalidMintAmount`、`FieldValidation`、`NotRegistered`、`EvidenceInvalid`、`RetroThresholdNotMet`、`StakeholderInsufficient`、`OperationNotAllowed`、`DoubleConversion`、`ProofVerificationFailed` 等）
    - _Requirements: 6.6, 6.7_
  - [x] 1.3 配置属性测试框架与生成器骨架
    - 接入 `proptest` 依赖，建立测试目录结构与共享生成器骨架 `tests/common/generators.rs`（派生树序列、评分占比/膨胀指数、铸造金额/影响期限/时间点、多链配额交错、干系人池与亲密度分布、碳积分重复申报序列）
    - 约定属性测试标签格式 `Feature: gmc-core-protocol, Property N: ...`，并约定每条属性由单一测试实现、迭代次数 ≥ 100
    - _Requirements: 5.2_

- [x] 2. Chain_Registry 派生树与生命周期
  - [x] 2.1 实现派生树数据模型与注册表索引
    - 在 `src/registry.rs` 实现 `NestedMeritChain`（`id / parentId / domain / path / depth / stewards / originType / createdAt / lifecycle / evaluationMechanism / config`）与 `(parentId, domain)` 唯一性索引；GMC_Base 为 depth=0 根节点
    - _Requirements: 1.1, 1.4, 2.4_
  - [x] 2.2 实现 derive 派生校验算法
    - 在 `src/registry.rs` 实现 `derive`：按"父链存在 → 不成环 → 深度 ≤ 16 → (父链, 领域) 唯一"顺序校验，任一失败不写入任何记录并返回对应错误；实现 `detectCycle` 守卫（目标父链等于自身或位于自身子树则拒绝）
    - _Requirements: 1.2, 1.5, 1.6, 1.7, 2.5, 2.9_
  - [x] 2.3 实现派生路径查询与评判机制继承
    - 在 `src/registry.rs` 实现 `getPath`（返回从 GMC_Base 起的有序路径，`len(path) == depth + 1`）、`setLifecycle`、`resolveEvaluationMechanism`（本链未定义则沿 path 上溯继承最近一个已定义祖先配置）
    - _Requirements: 1.3, 3.2_
  - [x]* 2.4 编写 Property 1 属性测试
    - **Property 1: 派生路径与元数据完整性**
    - **Validates: Requirements 1.2, 1.3, 1.4**
    - 单一 proptest，标签 `Feature: gmc-core-protocol, Property 1: ...`，≥100 次迭代
  - [x]* 2.5 编写 Property 2 属性测试
    - **Property 2: 派生树永不成环**
    - **Validates: Requirements 1.5**
    - 生成含派生与重挂接(re-parent)的请求序列，验证成环请求被拒且注册表状态不变
  - [x]* 2.6 编写 Property 3 属性测试
    - **Property 3: 层级深度上界**
    - **Validates: Requirements 1.7**
  - [x]* 2.7 编写 Property 4 属性测试
    - **Property 4: (父链, 领域) 全局唯一**
    - **Validates: Requirements 2.9**
  - [x]* 2.8 编写 Property 5 属性测试
    - **Property 5: 每条链至少一个 Steward**
    - **Validates: Requirements 2.4**
  - [x]* 2.9 编写 Property 6 属性测试
    - **Property 6: 评判机制沿派生路径继承**
    - **Validates: Requirements 3.2**

- [x] 3. 检查点 - 确保派生树相关测试通过
  - Ensure all tests pass, ask the user if questions arise.

- [x] 4. Governance_Module 加权投票与阈值判定
  - [x] 4.1 实现加权投票与阈值判定
    - 在 `src/governance.rs` 实现 `openVote / castVote / tally`：按 `voter.curMerit / Σ(voters.curMerit)` 计算投票权重，所有权重之和为 1；通过条件为加权赞成比例 ≥ threshold；预留 `anchorOutcome` 锚定接口与 ZK 身份保护占位
    - _Requirements: 11.5_
  - [x]* 4.2 编写 Property 26 属性测试
    - **Property 26: 投票按 curMerit 占比加权**
    - **Validates: Requirements 11.5**

- [x] 5. Evaluation_Mechanism 配置与变更治理
  - [x] 5.1 实现评判机制模型与配置校验
    - 在 `src/mechanism.rs` 实现 `EvaluationMechanism`（`acquisitionModes` 至少声明一种、`consensusThreshold ∈ (0,1]`、`excludeHighIntimacy=true`）；当且仅当声明 ≥1 种获取方式且阈值落在 (0,1] 时接受，否则拒绝并保留先前有效配置
    - _Requirements: 3.1, 3.3, 3.5, 3.6_
    - _Property: 7_
  - [x] 5.2 实现评判机制变更的治理阈值守卫
    - 在 `src/mechanism.rs` 接入 `Governance_Module`：变更须达本链既定治理阈值方可生效，未达则拒绝并保留现有配置；预留生效变更锚定 L1 的接口
    - _Requirements: 3.4, 3.7, 3.8_
    - _Property: 8_
  - [x]* 5.3 编写 Property 7 属性测试
    - **Property 7: 评判机制配置校验**
    - **Validates: Requirements 3.3, 3.6**
  - [x]* 5.4 编写 Property 8 属性测试
    - **Property 8: 未达治理阈值则配置不变**
    - **Validates: Requirements 3.7**

- [x] 6. 配额与刷新周期核算
  - [x] 6.1 实现 QuotaLedger 模型与配置校验
    - 在 `src/quota.rs` 实现 `QuotaLedger`（`mintedThisPeriod / periodStart / exhausted`）、`RefreshPeriod`（`OneTime` 或带显式单位且 value>0 的 `Periodic`）；当且仅当 `quota` 为大于零有限数值且 `refreshPeriod` 合法时接受配置，否则拒绝
    - _Requirements: 4.1, 4.8_
    - _Property: 12_
  - [x] 6.2 实现配额检查、消耗与逐链隔离
    - 在 `src/quota.rs` 实现 `checkQuota`（`mintedThisPeriod + amount ≤ quota`）、`consumeQuota`（累计本周期铸造量）；超限拒绝且计数不变；一次性链耗尽后置 `exhausted=true` 并对后续请求恒拒绝、不恢复配额；每条链独立计量互不影响
    - _Requirements: 4.2, 4.3, 4.4, 4.6, 4.7_
    - _Property: 9, 11_
  - [x] 6.3 实现刷新周期到期重置
    - 在 `src/quota.rs` 实现 `resetQuota`：非一次性链在 `Refresh_Period` 到期时将 `mintedThisPeriod` 重置为 0 并刷新 `periodStart`
    - _Requirements: 4.5_
    - _Property: 10_
  - [x]* 6.4 编写 Property 9 属性测试
    - **Property 9: 配额永不超限（含一次性耗尽不恢复）**
    - **Validates: Requirements 4.2, 4.3, 4.4, 4.7**
  - [x]* 6.5 编写 Property 10 属性测试
    - **Property 10: 刷新周期到期重置**
    - **Validates: Requirements 4.5**
  - [x]* 6.6 编写 Property 11 属性测试
    - **Property 11: 配额逐链隔离**
    - **Validates: Requirements 4.6**
  - [x]* 6.7 编写 Property 12 属性测试
    - **Property 12: 配额与刷新周期配置校验**
    - **Validates: Requirements 4.1, 4.8**

- [x] 7. 检查点 - 确保配额与治理相关测试通过
  - Ensure all tests pass, ask the user if questions arise.

- [x] 8. Scoring_Engine 三维评分
  - [x] 8.1 实现维度分类与占比校验
    - 在 `src/scoring.rs` 实现 `classify`：依据所属链的 `Evaluation_Mechanism` 将贡献归入 Thought / Training / Technique 中的 1~3 个维度；输出 `DimensionWeights`，每个适用维度占比 ∈ (0,1] 且总和 = 1；无法归类返回 `DimensionUnmatched`，占比和 ≠ 1 返回 `WeightSumInvalid`，均不铸造
    - _Requirements: 6.1, 6.5, 6.6, 6.7_
    - _Property: 13_
  - [x] 8.2 实现膨胀指数配置与区间校验
    - 在 `src/scoring.rs` 实现 `setInflationIndex`：Thought ∈ (1.00,10.00]、Training ∈ [0.95,1.05]、Technique ∈ [0.01,1.00]，精确到两位小数；越界拒绝并保留该维度原值；预留经治理阈值通过后生效并锚定 L1 的接口
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.7, 7.8, 7.9_
    - _Property: 14_
  - [x] 8.3 实现铸造数量加权求和
    - 在 `src/scoring.rs` 实现 `computeMintAmount`：`amount = Σ_dim 占比_dim × 基础分_dim × 膨胀指数_dim`；当所有基础分为 0 或结果不大于 0 时返回错误，保证有效输出严格大于零
    - _Requirements: 7.5, 7.6, 8.3_
    - _Property: 15_
  - [x]* 8.4 编写 Property 13 属性测试
    - **Property 13: 维度占比和为 100%**
    - **Validates: Requirements 6.1, 6.5**
  - [x]* 8.5 编写 Property 14 属性测试
    - **Property 14: 膨胀指数区间校验**
    - **Validates: Requirements 7.1, 7.2, 7.3, 7.4, 7.8**
  - [x]* 8.6 编写 Property 15 属性测试
    - **Property 15: 铸造数量按维度加权求和且为正**
    - **Validates: Requirements 7.5, 7.6, 8.3**
  - [x]* 8.7 编写维度分类示例单元测试
    - 科研→Thought（6.2）、AI 训练→Training（6.3）、手艺→Technique（6.4）；维度未匹配（6.6）与占比校验（6.7）错误路径
    - _Requirements: 6.2, 6.3, 6.4, 6.6, 6.7_

- [x] 9. MeriToken 衰减/底部值模型与 Minting_Service
  - [x] 9.1 实现 MeritPocket 与 Merit 批次模型及衰减计算
    - 在 `src/merit.rs` 实现 `MeritPocket`（`minMerit` 初始 = e ≈ 2.718）、`MeritBatch`（`V / B / lambda / influenceDuration / acquiredAt / sourceChainId`）；实现单批次衰减 `MeriToken_i(t) = (V_i − B_i)·e^(−λ_i·t) + B_i` 与 `curMerit(t) = Σ_i MeriToken_i(t)`
    - _Requirements: 8.1, 8.4_
  - [x] 9.2 实现 minMerit 更新与 curMerit≥minMerit 不变式
    - 在 `src/merit.rs` 实现底部值更新 `B' = (x + M) × B / M`（`x>0` 且 `M≥B` 时 `B'≥B`，只增不减，惩罚除外）；保证任意时刻 `curMerit ≥ minMerit`
    - _Requirements: 8.2, 8.5_
  - [x] 9.3 实现 Minting_Service 铸造流水线
    - 在 `src/minting.rs` 实现 `mint`：按"校验 amount>0 → 检查配额 → 创建批次 → 更新 minMerit → 累计配额消耗"顺序执行，任一前置失败不进入后续步骤、不部分修改 `MeritPocket` 与 `QuotaLedger`；`amount ≤ 0` 返回 `InvalidMintAmount` 且不改 curMerit/minMerit；预留 L2 计算 + L1 状态根锚定接口
    - _Requirements: 8.6, 8.7_
  - [x]* 9.4 编写 Property 16 属性测试
    - **Property 16: minMerit 单调非减**
    - **Validates: Requirements 8.2**
  - [x]* 9.5 编写 Property 17 属性测试
    - **Property 17: curMerit 永不低于 minMerit（含批次独立衰减）**
    - **Validates: Requirements 8.1, 8.4, 8.5**
  - [x]* 9.6 编写无效铸造单元测试
    - 验证 `amount ≤ 0` 时不创建批次、不修改 curMerit/minMerit 并返回 `InvalidMintAmount`
    - _Requirements: 8.7_

- [x] 10. 检查点 - 确保评分与铸造相关测试通过
  - Ensure all tests pass, ask the user if questions arise.

- [x] 11. 登记 → 记录 → 授予流程
  - [x] 11.1 实现 Registration_Service 登记与字段校验
    - 在 `src/registration.rs` 实现 `register`：必填 `contributorId / chainId / registeredAt`、描述 ≤ 2000 字，校验通过创建初始状态为 `Valid` 的登记记录，否则返回 `FieldValidation` 且不创建记录；实现 `findValidRegistration`；预留登记状态根锚定 L1 接口
    - _Requirements: 9.1, 9.2_
    - _Property: 18_
  - [x] 11.2 实现 Recording_Service 贡献记录与登记匹配
    - 在 `src/recording.rs` 实现 `record`：须存在匹配的有效登记（`contributorId + chainId + status=Valid`）方可创建贡献记录并关联登记，否则且非事后申报返回 `NotRegistered`；实现 `markEvaluationResult`（`passed=false` 保留记录并标记"认定未通过"，不铸造）
    - _Requirements: 9.3, 9.4, 9.6_
    - _Property: 19_
  - [x] 11.3 实现授予三条件守卫
    - 在 `src/registration.rs` 实现 `canGrant`：当且仅当"存在匹配有效登记 ∧ 存在关联贡献记录 ∧ 该记录已通过认定"三者同时为真时触发铸造；衔接 `Recording_Service` 认定通过后调用 `Minting_Service`
    - _Requirements: 9.5, 9.8_
    - _Property: 20_
  - [x]* 11.4 编写 Property 18 属性测试
    - **Property 18: 登记申请字段校验**
    - **Validates: Requirements 9.1, 9.2**
  - [x]* 11.5 编写 Property 19 属性测试
    - **Property 19: 贡献记录须匹配有效登记**
    - **Validates: Requirements 9.3, 9.4**
  - [x]* 11.6 编写 Property 20 属性测试
    - **Property 20: 授予三条件守卫**
    - **Validates: Requirements 9.5, 9.6, 9.8**

- [x] 12. AntiFraud_Engine 反刷票与投票者选取
  - [x] 12.1 实现高亲密度排除与随机抽样
    - 在 `src/antifraud.rs` 实现投票者选取：在归一化亲密度 [0,1] 内排除与贡献者亲密度 > 0.9 的全部实体；从剩余干系人中随机抽样产生规模 ≥7 且 ≤剩余总数的投票者集合；剩余 < 7 返回 `StakeholderInsufficient` 且不铸造
    - _Requirements: 11.1, 11.2, 11.3_
    - _Property: 24_
  - [x] 12.2 实现异常投票行为检测
    - 在 `src/antifraud.rs` 实现异常检测：当某投票者在最近 30 天评估窗口内对同一对象赞成投票次数 ≥5 次且赞成票占比 > 80% 时，标记为异常并记入待审计条目
    - _Requirements: 11.4_
    - _Property: 25_
  - [x]* 12.3 编写 Property 24 属性测试
    - **Property 24: 投票者选取排除高亲密度且规模合规**
    - **Validates: Requirements 11.1, 11.2**
  - [x]* 12.4 编写 Property 25 属性测试
    - **Property 25: 异常投票行为检测**
    - **Validates: Requirements 11.4**
  - [x]* 12.5 编写串通追溯回收单元测试
    - 构造串通刷票场景，验证回收因该次认定铸造的 MeriToken、撤销认定结果并锚定处理结果（铸造逆操作的一致回退）
    - _Requirements: 11.6_

- [x] 13. GMC_Base 货币兑换拦截
  - [x] 13.1 实现根节点与货币兑换拒绝
    - 在 `src/gmc_base.rs` 实现 `rootChainId`、`recordTopLevelCategory`（顶层贡献行为类别）与 `rejectMonetaryRequest`：任何以货币注资兑换 MeriToken 或购买贡献认定的请求一律拒绝，不铸造、不变更认定结果，返回 `OperationNotAllowed`
    - _Requirements: 1.1, 11.8_
  - [x]* 13.2 编写货币兑换拒绝单元测试
    - 验证注资兑换/购买认定请求被拒绝且无任何状态变更
    - _Requirements: 11.8_

- [x] 14. 检查点 - 确保登记授予与反作弊相关测试通过
  - Ensure all tests pass, ask the user if questions arise.

- [x] 15. 事后申报审核投票
  - [x] 15.1 实现 Retroactive_Review_Module 受理与证据校验
    - 在 `src/retroactive.rs` 实现 `RetroactiveDeclaration` 模型与受理逻辑：必填 `contributorId / chainId / description / occurredAt` 且至少一条可被审核者独立访问与核验（可复盘）的证据引用，校验通过标记 `Pending`，否则返回 `EvidenceInvalid` 且不入投票
    - _Requirements: 10.1, 10.2, 10.8_
    - _Property: 21_
  - [x] 15.2 实现事后申报阈值与投票组织
    - 在 `src/retroactive.rs` 实现 `retroThreshold = max(链常规阈值, 2/3)` 且严格高于常规阈值；衔接 `AntiFraud_Engine` 选取投票者与 `Governance_Module` 加权投票（ZK 隐私）；未达阈值标记 `Rejected` 且不铸造；通过则按三维模型铸造；预留审核状态与投票结果锚定 L1 接口
    - _Requirements: 10.3, 10.4, 10.5, 10.6, 10.7_
    - _Property: 22, 23_
  - [x]* 15.3 编写 Property 21 属性测试
    - **Property 21: 事后申报受理证据校验**
    - **Validates: Requirements 10.1, 10.2, 10.8**
  - [x]* 15.4 编写 Property 22 属性测试
    - **Property 22: 事后申报阈值严格更高**
    - **Validates: Requirements 10.3**
  - [x]* 15.5 编写 Property 23 属性测试
    - **Property 23: 事后申报未达阈值则驳回**
    - **Validates: Requirements 10.5**

- [x] 16. 碳积分转 MeriToken 应用场景
  - [x] 16.1 实现碳积分凭证模型与申报导入
    - 在 `src/carbon.rs` 实现 `CarbonCreditVoucher`（`voucherId / evidenceRef / converted / convertedDeclarationId`）；环保链启用碳积分场景时，以可验证碳积分凭证引用为证据的贡献申报导入事后申报流程；凭证无效返回 `EvidenceInvalid` 且不铸造、不消耗配额
    - _Requirements: 12.1, 12.2, 12.3, 12.4_
  - [x] 16.2 实现至多一次转化与配额计入
    - 在 `src/carbon.rs` 实现转化守卫：凭证已标记"已转化"再申报返回 `DoubleConversion` 且不铸造、不消耗配额；成功铸造时将凭证标记 `converted=true` 并将铸造量计入环保链当前 `Refresh_Period` 配额消耗（计入后不超 `Quota`）
    - _Requirements: 12.5, 12.6, 12.7_
    - _Property: 27, 28_
  - [x]* 16.3 编写 Property 27 属性测试
    - **Property 27: 碳积分凭证至多转化一次**
    - **Validates: Requirements 12.6, 12.7**
  - [x]* 16.4 编写 Property 28 属性测试
    - **Property 28: 碳积分铸造计入当前周期配额**
    - **Validates: Requirements 12.5**

- [x] 17. 检查点 - 确保事后申报与碳积分相关测试通过
  - Ensure all tests pass, ask the user if questions arise.

- [x] 18. L1_Settlement（Substrate）集成
  - [x] 18.1 实现 L1 存储与注册/治理/惩罚锚定
    - 实现 Substrate pallet 包装 `gmc-core` 逻辑：存储功勋链注册记录、身份注册记录、治理投票结果、惩罚记录与状态根；接入 `Chain_Registry` 派生关系状态根、创建记录锚定、机制/膨胀指数变更锚定、事后申报结果锚定；配置免手续费与 GRANDPA/BABE 共识
    - _Requirements: 2.6, 3.8, 5.1, 7.7, 8.6, 10.7, 13.1, 13.4, 13.6_
  - [x] 18.2 实现 ZK 证明验证与状态根更新守卫
    - 在 L1 pallet 实现批次证明验证：验证通过则更新状态根为该批次状态根，验证失败返回 `ProofVerificationFailed`、拒绝该批次状态更新并保留上一已确认状态根
    - _Requirements: 13.8_
    - _Property: 30_
  - [x]* 18.3 编写 Property 30 属性测试
    - **Property 30: 证明验证失败保留前一状态根**
    - **Validates: Requirements 13.8**
  - [x]* 18.4 编写 L1 锚定与免手续费集成测试
    - 验证 L1 存储职责（13.1）、各模块锚定（2.6/3.8/5.1/7.7/8.6/10.7）与每条交易不收费（13.4）
    - _Requirements: 2.6, 3.8, 5.1, 7.7, 8.6, 10.7, 13.1, 13.4_

- [x] 19. L2_Rollup（ZK Rollup）集成
  - [x] 19.1 实现 L2 高频处理与批量证明提交
    - 实现 L2 Rollup 包装：处理贡献记录创建、MeriToken 计算与亲密度更新，记录提交后 5 秒内返回计算结果；累计 1,000 条或距上批满 60 秒（先到者为准）即向 L1 提交批次 ZK 证明；接入 `Recording_Service.submitRollupBatch`；配置 BFT 类共识（区块最终确认 ≤ 3 秒）
    - _Requirements: 9.7, 13.2, 13.3, 13.7_
    - _Property: 29_
  - [x] 19.2 实现分片扩容与 ZK 投票隐私
    - 实现分片扩容决策：全网提交速率持续 > 在用实例额定吞吐之和（默认 1,000 条/秒/实例）超 60 秒则新增并行 Rollup 实例，直至总额定吞吐 ≥ 提交速率；接入 `AntiFraud_Engine` / `Governance_Module` 的 ZK 投票隐私，仅公开投票结果不泄露身份
    - _Requirements: 11.7, 13.5_
  - [x]* 19.3 编写 Property 29 属性测试
    - **Property 29: 批量证明触发条件**
    - **Validates: Requirements 13.3**
  - [x]* 19.4 编写 L2 处理与跨层集成测试
    - 验证 L2 时延（13.2）、BFT 最终确认 ≤3s（13.7）、分片扩容触发（13.5）、ZK 投票隐私仅含结果不泄露身份（11.7）
    - _Requirements: 11.7, 13.2, 13.5, 13.7_

- [x] 20. 端到端流程接线与发起通道
  - [x] 20.1 接线四条关键流程
    - 串联流程 1（功勋链派生）、流程 2（登记→记录→授予）、流程 3（事后申报审核投票）、流程 4（碳积分转 MeriToken），将 L1/L2 模块与核心逻辑贯通，消除孤立未集成代码
    - _Requirements: 9.5, 9.8, 10.6, 12.2_
  - [x] 20.2 实现嵌套功勋链三种发起通道
    - 实现"投票发起"（达治理阈值，2.1）、"主理人发起"（资格校验，2.2/2.7）、"机构申请"（审核通过，2.3/2.8）三种创建通道，记录对应 `originType`；缺父链/领域标识返回 `MissingField`（2.5）
    - _Requirements: 2.1, 2.2, 2.3, 2.5, 2.7, 2.8_
  - [x]* 20.3 编写发起通道与端到端集成测试
    - 验证三种发起通道的成功与拒绝路径，以及四条关键流程的端到端贯通
    - _Requirements: 2.1, 2.2, 2.3, 2.7, 2.8, 9.5, 10.6, 12.2_

- [x] 21. 技术选型评估校验与冒烟测试
  - [x]* 21.1 编写选型判定单元测试
    - 验证收费方案被标记为不满足"免费或极低成本记录"约束、免费方案满足约束（5.5）
    - _Requirements: 5.5_
  - [x]* 21.2 编写交付物与一次性配置冒烟测试
    - 校验设计文档"技术选型评估"含基准候选、对照候选、三维对比表与附依据建议（5.2/5.3/5.6）；校验根节点配置（1.1）、L1 免手续费（13.4）、L1 共识 GRANDPA/BABE（13.6）
    - _Requirements: 1.1, 5.2, 5.3, 5.6, 13.4, 13.6_

- [x] 22. 最终检查点 - 确保全部测试通过
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- 标记 `*` 的子任务为可选测试任务（单元/属性/集成/冒烟），可为更快的 MVP 跳过；核心实现任务不可标记可选。
- 每个任务引用其覆盖的具体需求子条款（`_Requirements:_`），承载属性测试的实现任务额外标注其实现的属性编号（`_Property:_`）以便追溯。
- 正确性属性 Property 1–30 各由**单一** proptest 属性测试实现，标签格式 `Feature: gmc-core-protocol, Property N: ...`，每条至少运行 100 次随机迭代；属性测试任务紧邻其对应实现任务，以尽早捕获逻辑错误。
- 纯逻辑核心（派生树、评分数学、配额核算、衰减模型、投票者选取）先于基础设施/集成实现，因其承载全部属性测试。
- 检查点用于增量验证；基础设施类、配置类与文档交付物类验收标准由集成/冒烟/示例测试覆盖（不适用 PBT）。
- 默认技术栈 Rust（Substrate）+ proptest；如改用 TypeScript 参考模型则相应替换为 fast-check，属性映射与任务结构保持不变。

## Task Dependency Graph

```json
{
  "waves": [
    { "id": 0, "tasks": ["1.1"] },
    { "id": 1, "tasks": ["1.2", "1.3"] },
    { "id": 2, "tasks": ["2.1", "4.1", "5.1", "6.1", "8.1", "9.1", "13.1"] },
    { "id": 3, "tasks": ["2.2", "5.2", "6.2", "8.2", "9.2", "11.1", "11.2", "12.1", "4.2", "13.2"] },
    { "id": 4, "tasks": ["2.3", "6.3", "8.3", "12.2", "15.1", "5.3", "5.4", "9.4", "9.5", "11.4", "11.5"] },
    { "id": 5, "tasks": ["9.3", "16.1", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "6.4", "6.5", "6.6", "6.7", "8.4", "8.5", "8.6", "8.7", "12.3", "12.4", "15.3"] },
    { "id": 6, "tasks": ["11.3", "15.2", "16.2", "9.6", "12.5"] },
    { "id": 7, "tasks": ["18.1", "19.1", "11.6", "15.4", "15.5", "16.3", "16.4"] },
    { "id": 8, "tasks": ["18.2", "19.2"] },
    { "id": 9, "tasks": ["20.1", "18.3", "18.4", "19.3", "19.4"] },
    { "id": 10, "tasks": ["20.2"] },
    { "id": 11, "tasks": ["20.3", "21.1", "21.2"] }
  ]
}
```
