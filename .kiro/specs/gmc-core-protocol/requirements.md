# Requirements Document

## Introduction

本文档定义 GMC（Global Merit Chain，全球功勋链）核心协议的技术设计需求。GMC 的定位是"后货币时代的贡献共识基础设施"：贡献被记录与承认，认可成为主要回报；话语权来自贡献占比而非资本或权力；MeriToken 不可交易、不可兑换，随时间指数衰减并保有非零底部值。

本协议在既有蓝图（blueprint 第 03/04/06/08/09/11/12/13 章）的基础上，新增并细化三条基础原则：

- **原则一（递归嵌套功勋链）**：GMC 作为基座，可递归派生领域分支，每个嵌套功勋链拥有自定义评判机制、独立配额与刷新周期。
- **原则二（三维功勋评分）**：贡献按"思想 / 训练 / 技艺"三个维度评分，每个维度配置独立的膨胀指数，并与既有 MeriToken 衰减 / 底部值经济模型集成。
- **原则三（登记 → 记录 → 授予流程）**：功勋一般须先在链上登记、再记录行为、再授予 MeriToken；允许事后申报，但事后申报需经严格审核与干系人投票，并配套严格的反刷票策略。

本文档使用 EARS 模式描述验收标准，并遵循 INCOSE 质量规则。本文档为讨论稿阶段的需求设想，递归嵌套关系的技术选型需在设计阶段对以太坊及同类产品进行评估，与既有蓝图推荐的 Substrate L1 + ZK Rollup L2 架构共同权衡。

## Glossary

- **GMC_Base（GMC 基座）**：顶层功勋链基座，记录贡献行为，可递归派生嵌套功勋链。
- **Nested_Merit_Chain（嵌套功勋链）**：从 GMC_Base 或另一个嵌套功勋链派生出的领域分支链，例如学术、社会公益、知识普及、环境保护、艺术等领域。
- **Chain_Registry（功勋链注册表）**：记录所有功勋链派生关系、归属层级与生命周期状态的链上注册表。
- **Steward（主理人）**：负责发起与维护某个嵌套功勋链的实体。
- **Stakeholder（干系人）**：与某次贡献或某条功勋链相关的利益相关方，负责共识投票。
- **Governance_Module（治理投票模块）**：执行加权投票、共识判定与提案处理的协议模块。
- **Quota（配额）**：某个嵌套功勋链在一个刷新周期内可铸造的 MeriToken 上限。
- **Refresh_Period（刷新周期）**：配额重置的时间间隔；可设为"一次性"（无刷新周期）。
- **Evaluation_Mechanism（评判机制）**：嵌套功勋链自定义的贡献评判规则集合。
- **Scoring_Engine（功勋评分引擎）**：依据三维模型计算贡献评分的协议模块。
- **Thought（思想维度）**：引领人类认知突破的贡献维度，例如科学研究与发明创造。
- **Training（训练维度）**：快速普及既有事物并提升效率的贡献维度，例如训练领域专用 AI 模型。
- **Technique（技艺维度）**：人类通过技能 / 手艺提供价值的贡献维度，例如服务员、演员、匠人。
- **Inflation_Index（膨胀指数）**：各评分维度的膨胀参数，作用于该维度贡献的铸造数量。
- **Registration_Service（功勋登记服务）**：受理功勋登记申请并在链上创建登记记录的协议模块。
- **Recording_Service（贡献记录服务）**：在已登记功勋之后记录具体贡献行为的协议模块。
- **Minting_Service（铸造服务）**：在记录通过后铸造 MeriToken 并写入 MeritPocket 的协议模块。
- **Retroactive_Declaration（事后申报）**：对已发生但未事先登记的贡献提交的补充申报。
- **Retroactive_Review_Module（事后申报审核器）**：受理并审核事后申报、组织干系人投票的协议模块。
- **AntiFraud_Engine（反作弊引擎）**：执行亲密度排除、行为审计、随机抽样、异常检测的协议模块。
- **MeritPocket**：MeriToken 的容器，绑定一个 FayID（沿用既有蓝图定义）。
- **MeriToken**：贡献度量单位，不可交易、不可转让，按批次指数衰减并保有底部值（沿用既有蓝图定义）。
- **curMerit**：MeritPocket 的当前 MeriToken 值，随时间衰减、随贡献增长（沿用既有蓝图定义）。
- **minMerit**：MeriToken 的底部值，只增不减（惩罚除外），初始值 = e ≈ 2.718（沿用既有蓝图定义）。
- **L1_Settlement（L1 结算层）**：基于 Substrate 的专用链，负责状态根锚定、身份管理、治理投票与惩罚记录。
- **L2_Rollup（L2 Rollup 层）**：基于 ZK Rollup 的高频处理层，负责贡献记录、MeriToken 实时计算与亲密度更新。

---

## Requirements

### Requirement 1: GMC 基座与递归派生结构

**User Story:** 作为协议设计者，我希望 GMC 作为可递归派生的基座存在，以便顶层记录贡献行为、下层按领域分化形成嵌套功勋链。

#### Acceptance Criteria

1. THE GMC_Base SHALL 作为整个派生层级的根节点（层级深度为 0）在 Chain_Registry 中记录顶层贡献行为类别。
2. WHEN 某个领域分支被创建，THE GMC_Base SHALL 将该分支登记为一个 Nested_Merit_Chain，并在 Chain_Registry 中记录该分支与其父链的派生关系。
3. WHEN 一个 Nested_Merit_Chain 从另一个 Nested_Merit_Chain 派生，THE Chain_Registry SHALL 记录一条从根节点 GMC_Base 起始、按派生顺序逐级排列直至该链的完整有序派生层级路径。
4. THE Chain_Registry SHALL 为每个 Nested_Merit_Chain 记录归属领域标识、父链标识与创建时间。
5. IF 一个派生请求会使某个 Nested_Merit_Chain 成为自身的祖先链（形成环路），THEN THE Chain_Registry SHALL 拒绝该派生请求、不创建任何派生记录（保持 Chain_Registry 状态不变）并返回环路冲突错误。
6. IF 一个派生请求所指定的父链在 Chain_Registry 中不存在，THEN THE Chain_Registry SHALL 拒绝该派生请求、不创建任何派生记录并返回父链不存在错误。
7. IF 一个派生请求会使 Nested_Merit_Chain 的层级深度超过 16 层（以 GMC_Base 为深度 0 的根节点计），THEN THE Chain_Registry SHALL 拒绝该派生请求、不创建任何派生记录并返回层级深度超限错误。

### Requirement 2: 嵌套功勋链的创建与发起

**User Story:** 作为社区成员或机构，我希望能通过投票、主理人发起或机构申请三种方式创建嵌套功勋链，以便不同来源的领域需求都能被纳入协议。

#### Acceptance Criteria

1. WHEN 一项创建嵌套功勋链的投票提案在 Governance_Module 中达到本链既定的治理通过阈值，THE Chain_Registry SHALL 创建对应的 Nested_Merit_Chain 并记录发起方式为"投票发起"。
2. WHEN 一个具备主理人资格的实体提交创建请求，THE Chain_Registry SHALL 创建对应的 Nested_Merit_Chain，记录该实体为 Steward，并记录发起方式为"主理人发起"。
3. WHEN 一个机构提交创建申请且申请通过审核，THE Chain_Registry SHALL 创建对应的 Nested_Merit_Chain 并记录发起方式为"机构申请"。
4. THE Chain_Registry SHALL 为每个 Nested_Merit_Chain 记录至少一个 Steward 标识。
5. IF 创建请求缺少所属父链标识或领域标识，THEN THE Chain_Registry SHALL 拒绝该创建请求、不创建任何 Nested_Merit_Chain，并返回指明所缺失字段（父链标识或领域标识）的字段缺失错误。
6. WHEN 一个 Nested_Merit_Chain 被成功创建，THE Chain_Registry SHALL 将包含该链标识、父链标识、领域标识、Steward 标识、发起方式与创建时间的创建记录锚定到 L1_Settlement。
7. IF 一个以"主理人发起"方式提交创建请求的实体不具备主理人资格，THEN THE Chain_Registry SHALL 拒绝该创建请求、不创建任何 Nested_Merit_Chain，并返回主理人资格不满足错误。
8. IF 一个机构提交的创建申请未通过审核，THEN THE Chain_Registry SHALL 拒绝该创建申请、不创建任何 Nested_Merit_Chain，并返回申请审核未通过错误。
9. IF 一个创建请求的父链标识与领域标识组合已存在于 Chain_Registry，THEN THE Chain_Registry SHALL 拒绝该创建请求、保留既有 Nested_Merit_Chain 记录不变，并返回领域重复冲突错误。

### Requirement 3: 嵌套功勋链的自定义评判机制

**User Story:** 作为嵌套功勋链的主理人，我希望为本链定义自定义的功勋评判机制，以便不同领域采用各自合适的贡献认定规则。

#### Acceptance Criteria

1. THE Nested_Merit_Chain SHALL 保存一份独立的 Evaluation_Mechanism 配置。
2. WHERE 某个 Nested_Merit_Chain 未定义自定义 Evaluation_Mechanism，THE Nested_Merit_Chain SHALL 沿派生层级向上，继承其上溯路径中最近一个已定义 Evaluation_Mechanism 的祖先链配置。
3. THE Evaluation_Mechanism SHALL 至少声明两项内容：贡献认定的获取方式（客观计量、任务悬赏或两者组合）与一个共识通过阈值，且该共识通过阈值为大于 0%（即大于 0）且不超过 100%（即不超过 1）的比例值。
4. WHEN Steward 提交对 Evaluation_Mechanism 的变更，THE Governance_Module SHALL 要求该变更的加权赞成比例达到本链既定治理阈值后方可使其生效。
5. THE Evaluation_Mechanism SHALL 沿用既有贡献认定机制中的干系人投票与高亲密度者排除规则。
6. IF 某个 Evaluation_Mechanism 配置缺少贡献认定获取方式声明、缺少共识通过阈值，或其共识通过阈值不落在大于 0% 且不超过 100% 的区间内，THEN THE Nested_Merit_Chain SHALL 拒绝该配置、保留先前有效配置不变，并返回配置校验错误。
7. IF 一项 Evaluation_Mechanism 变更的加权赞成比例未达到本链既定治理阈值，THEN THE Governance_Module SHALL 拒绝该变更、保留现有 Evaluation_Mechanism 配置不变，并返回变更未通过提示。
8. WHEN 一项 Evaluation_Mechanism 变更通过本链既定治理阈值并生效，THE Governance_Module SHALL 将该次变更记录锚定到 L1_Settlement。

### Requirement 4: 配额与刷新周期

**User Story:** 作为协议设计者，我希望每个嵌套功勋链拥有独立的 MeriToken 配额与刷新周期（或一次性无刷新），以便控制各领域的铸造节奏并防止单链无限增发。

#### Acceptance Criteria

1. THE Nested_Merit_Chain SHALL 记录一个以 MeriToken 计量、取值为大于零的有限数值的 Quota，以及一个 Refresh_Period 设置；该 Refresh_Period 取值为"一次性"（无刷新周期）或一个带显式时间单位（秒、小时或天）且取值大于零的有限时间间隔。
2. WHERE 某个 Nested_Merit_Chain 被配置为一次性，THE Nested_Merit_Chain SHALL 将 Refresh_Period 标记为"无刷新周期"，且在 Quota 耗尽后停止新增铸造。
3. IF 一个铸造请求会使本周期内已铸造的 MeriToken 累计总量超过 Quota，THEN THE Minting_Service SHALL 拒绝该铸造请求、返回配额超限错误，并保持本周期已铸造计数不变（被拒绝的请求不计入累计量）。
4. WHILE 一个 Refresh_Period 处于进行中，THE Minting_Service SHALL 累计本周期内已铸造的 MeriToken 数量。
5. WHEN 一个非一次性 Nested_Merit_Chain 的 Refresh_Period 到期，THE Minting_Service SHALL 将本链本周期已铸造计数重置为零。
6. THE Minting_Service SHALL 为每个 Nested_Merit_Chain 独立计量 Quota 消耗，互不影响其它链的可用配额。
7. WHEN 一个被配置为一次性的 Nested_Merit_Chain 的 Quota 已耗尽后再收到铸造请求，THE Minting_Service SHALL 拒绝该请求并返回配额超限错误，且不为该链恢复任何可用 Quota。
8. IF 一个 Nested_Merit_Chain 的配置中 Quota 不是大于零的有限数值，或 Refresh_Period 既非"一次性"也非带显式时间单位且大于零的有限时间间隔，THEN THE Nested_Merit_Chain SHALL 拒绝该配置并返回校验错误。

### Requirement 5: 递归嵌套关系的技术选型评估

**User Story:** 作为协议架构师，我希望递归嵌套功勋链的实现技术选型有明确的评估范围，以便在 Substrate L1 + ZK Rollup L2 既定方向与以太坊等同类产品之间做出有据可依的取舍。

#### Acceptance Criteria

1. THE Chain_Registry SHALL 通过 L1_Settlement 维护功勋链的派生关系状态根。
2. WHERE 嵌套功勋链需要可组合的合约化派生能力，THE 技术选型评估 SHALL 将既定的 Substrate L1 + ZK Rollup L2 架构纳入为基准候选，并将以太坊及同类产品的派生 / 子链方案纳入为对照候选。
3. THE 技术选型评估 SHALL 为每个候选方案记录其相对于基准候选的可比较对比结论，且该对比结论 SHALL 覆盖以下三个维度：单条贡献记录的交易成本、单位时间可处理的贡献记录数（吞吐量），以及对自定义评判机制、独立配额与刷新周期的支持程度（可定制治理）。
4. THE Nested_Merit_Chain 的贡献记录 SHALL 在 L2_Rollup 上处理。
5. IF 某个候选技术方案对每条贡献记录向贡献者收取交易手续费，或其单条贡献记录成本超过协议规定的单条记录成本上限，THEN THE 技术选型评估 SHALL 将该方案标记为不满足"免费或极低成本记录"约束。
6. WHEN 三个维度的对比结论全部记录完成，THE 技术选型评估 SHALL 输出一份在基准候选与对照候选之间的技术选型建议，并附带支撑该建议的三维对比依据。

### Requirement 6: 三维功勋分类

**User Story:** 作为贡献者，我希望我的贡献被归入"思想 / 训练 / 技艺"三个维度之一，以便不同性质的贡献被区别度量。

#### Acceptance Criteria

1. WHEN 一次贡献被提交认定，THE Scoring_Engine SHALL 依据该贡献所属功勋链的 Evaluation_Mechanism 将该贡献归类到 Thought、Training、Technique 中的 1 至 3 个维度。
2. THE Scoring_Engine SHALL 将引领人类认知突破类贡献（例如科学研究、发明创造）归入 Thought 维度。
3. THE Scoring_Engine SHALL 将快速普及既有事物并提升效率类贡献（例如训练领域专用 AI 模型）归入 Training 维度。
4. THE Scoring_Engine SHALL 将人类通过技能或手艺提供价值类贡献（例如服务、表演、手工艺）归入 Technique 维度。
5. WHERE 一次贡献同时具备多个维度的属性，THE Scoring_Engine SHALL 记录每个适用维度及其对应的贡献占比，且每个适用维度的占比大于 0% 且不超过 100%，所有适用维度的占比之和等于 100%。
6. IF 一次贡献无法归入任一维度，THEN THE Scoring_Engine SHALL 拒绝该评分请求、不铸造任何 MeriToken，并返回维度未匹配错误。
7. IF 一次贡献各适用维度的占比之和不等于 100%，THEN THE Scoring_Engine SHALL 拒绝该评分请求、不铸造任何 MeriToken，并返回占比校验错误。

### Requirement 7: 维度膨胀指数

**User Story:** 作为协议设计者，我希望每个评分维度配置独立的膨胀指数，以便思想类贡献获得高于、训练类贡献约等于、技艺类贡献不高于基准的铸造倍率。

#### Acceptance Criteria

1. THE Scoring_Engine SHALL 为 Thought、Training、Technique 各维度分别保存一个取值范围在 0.01 至 10.00（含端点）、精确到两位小数的 Inflation_Index 数值。
2. THE Scoring_Engine SHALL 将 Thought 维度的 Inflation_Index 配置为大于 1.00 且不超过 10.00 的数值。
3. THE Scoring_Engine SHALL 将 Training 维度的 Inflation_Index 配置为落在闭区间 [0.95, 1.05]（即 1.00 ± 0.05）内的数值。
4. THE Scoring_Engine SHALL 将 Technique 维度的 Inflation_Index 配置为落在闭区间 [0.01, 1.00] 内的数值。
5. WHEN Scoring_Engine 计算某次贡献的铸造数量，THE Scoring_Engine SHALL 将该贡献的维度基础分乘以对应维度的 Inflation_Index。
6. WHEN 一次贡献跨多个维度，THE Scoring_Engine SHALL 按各维度贡献占比对各维度（基础分 × Inflation_Index）的结果求和，得到该次贡献的铸造数量。
7. WHEN Steward 或治理流程提交的某维度 Inflation_Index 修改经既定治理阈值通过，THE Governance_Module SHALL 使该修改生效，并将修改记录锚定到 L1_Settlement。
8. IF 一个 Inflation_Index 配置取值超出该维度规定的取值区间，THEN THE Scoring_Engine SHALL 拒绝该配置、保留该维度原有 Inflation_Index 不变，并返回取值越界校验错误。
9. IF 一项 Inflation_Index 修改未达到既定治理阈值，THEN THE Governance_Module SHALL 拒绝该修改、保留该维度当前 Inflation_Index 不变，并返回修改未通过提示。

### Requirement 8: 三维评分与 MeriToken 经济模型集成

**User Story:** 作为协议设计者，我希望三维评分的铸造结果接入既有 MeriToken 衰减与底部值模型，以便新评分体系与既有经济模型保持一致。

#### Acceptance Criteria

1. WHEN Minting_Service 依据 Scoring_Engine 的结果铸造 MeriToken，THE Minting_Service SHALL 为该次铸造创建一个独立的 Merit 批次，记录获取数量（等于 Scoring_Engine 计算的单次铸造数量且为大于零的数值）、影响期限（大于零的时长）、衰减系数与获取时间（该次铸造的链上时间）。
2. WHEN Minting_Service 完成一次 MeriToken 铸造，THE Minting_Service SHALL 按既有底部值更新规则更新对应 MeritPocket 的 minMerit，且该更新仅使 minMerit 增加或保持不变（惩罚场景除外）。
3. THE Scoring_Engine SHALL 保证经 Inflation_Index 计算后的单次铸造数量为大于零的数值。
4. THE Minting_Service SHALL 保持每个 Merit 批次按既有单批次指数衰减公式独立衰减。
5. WHILE 一个 MeritPocket 的所有批次持续衰减，THE Minting_Service SHALL 保证 curMerit 不低于 minMerit。
6. THE Minting_Service SHALL 在 L2_Rollup 上执行 MeriToken 的实时计算，并将状态根锚定到 L1_Settlement。
7. IF 经 Inflation_Index 计算后的单次铸造数量不大于零，THEN THE Minting_Service SHALL 拒绝该次铸造，不创建 Merit 批次，不修改对应 MeritPocket 的 curMerit 与 minMerit，并返回表明铸造数量无效的错误。

### Requirement 9: 登记 → 记录 → 授予标准流程

**User Story:** 作为贡献者，我希望先在链上登记功勋、再记录贡献行为、再授予 MeriToken，以便贡献的产生过程可追溯且抗作弊。

#### Acceptance Criteria

1. WHEN 一个贡献者提交功勋登记申请且申请包含贡献者标识、所属功勋链标识、不超过 2000 字的预期贡献描述与登记时间，THE Registration_Service SHALL 在链上创建一条登记记录（初始登记状态为"有效"）并将该登记状态根锚定到 L1_Settlement。
2. IF 一个功勋登记申请缺少必填字段或其预期贡献描述超过 2000 字，THEN THE Registration_Service SHALL 拒绝该申请、不创建任何登记记录，并返回字段校验错误。
3. WHEN 一次贡献行为被提交记录且存在一条与之匹配的有效登记记录（贡献者标识一致、所属功勋链标识一致且登记状态为"有效"），THE Recording_Service SHALL 创建一条贡献记录并关联到该登记记录。
4. IF 一次贡献记录提交时不存在与之匹配的有效登记记录（贡献者标识、所属功勋链标识匹配且状态为"有效"）且未走事后申报流程，THEN THE Recording_Service SHALL 拒绝该记录并返回"未登记"错误。
5. WHEN 一条贡献记录通过其所属功勋链的 Evaluation_Mechanism 认定，THE Minting_Service SHALL 铸造对应的 MeriToken 并写入贡献者的 MeritPocket。
6. IF 一条贡献记录未通过其所属功勋链的 Evaluation_Mechanism 认定，THEN THE Recording_Service SHALL 保留该贡献记录、将其标记为"认定未通过"，且 THE Minting_Service SHALL 不铸造任何 MeriToken。
7. THE Recording_Service SHALL 在 L2_Rollup 上处理贡献记录，并将批量记录的零知识证明提交到 L1_Settlement。
8. THE Registration_Service SHALL 保证授予动作仅在同时满足以下三个条件时触发：存在匹配的有效登记记录、存在关联的贡献记录、该贡献记录已通过 Evaluation_Mechanism 认定。

### Requirement 10: 事后申报与严格审核投票

**User Story:** 作为贡献者，我希望对未事先登记的已发生贡献提交事后申报，以便历史贡献也能被认定，同时接受比常规流程更严格的审核。

#### Acceptance Criteria

1. WHEN 一个贡献者提交 Retroactive_Declaration 且该申报包含贡献者标识、所属功勋链标识、已发生贡献描述、贡献发生时间与可复盘贡献证据引用，THE Retroactive_Review_Module SHALL 创建一条事后申报记录并将审核状态标记为"待审核"。
2. THE Retroactive_Review_Module SHALL 要求每条 Retroactive_Declaration 至少附带一条可复盘贡献证据引用，且该证据引用须指向审核者可独立访问并核验的链上记录或外部可验证记录。
3. WHEN 一条 Retroactive_Declaration 进入投票，THE Governance_Module SHALL 采用严格高于常规贡献认定通过阈值、且不低于参与投票总加权票数三分之二（约 66.7%）的干系人投票通过阈值。
4. THE Governance_Module SHALL 在事后申报投票中应用高亲密度者排除规则与 MeriToken 加权规则。
5. IF 一条 Retroactive_Declaration 的最终通过加权票数低于第 3 条规定的通过阈值，THEN THE Retroactive_Review_Module SHALL 将该申报审核状态标记为"驳回"、不触发任何 MeriToken 铸造，并向申报者返回驳回结果指示。
6. WHEN 一条 Retroactive_Declaration 通过审核与投票，THE Minting_Service SHALL 按三维评分模型为该历史贡献铸造 MeriToken。
7. THE Retroactive_Review_Module SHALL 将每条事后申报的审核状态与投票结果锚定到 L1_Settlement。
8. IF 一条 Retroactive_Declaration 的证据引用无法通过可复盘性校验或无法被审核者独立访问与核验，THEN THE Retroactive_Review_Module SHALL 拒绝该申报并返回证据无效错误，且不将该申报推入投票流程。

### Requirement 11: 反刷票与反欺诈策略

**User Story:** 作为协议设计者，我希望事后申报与贡献认定投票配套严格的反刷票策略，以便让作弊成本远高于收益。

#### Acceptance Criteria

1. WHEN AntiFraud_Engine 为某次认定选取投票者，THE AntiFraud_Engine SHALL 在归一化亲密度区间 [0, 1] 内排除与贡献者亲密度大于 0.9 的所有实体。
2. WHEN AntiFraud_Engine 完成高亲密度实体排除，THE AntiFraud_Engine SHALL 从剩余干系人中随机抽样产生一个规模不少于 7 名且不超过剩余干系人总数的投票者集合。
3. IF 排除高亲密度实体后剩余干系人数量少于 7 名，THEN THE AntiFraud_Engine SHALL 暂缓本次认定投票、返回干系人不足错误，且不铸造任何 MeriToken。
4. WHEN 某个投票者在最近 30 天评估窗口内对同一对象的赞成投票次数不少于 5 次且其赞成票占该投票者对该对象全部投票的比例超过 80%，THE AntiFraud_Engine SHALL 将相关投票行为标记为异常并记录为待审计条目。
5. THE Governance_Module SHALL 按投票者 curMerit 占当前投票者集合 curMerit 总和的比例对其投票进行加权。
6. IF AntiFraud_Engine 在认定通过后检测到串通刷票，THEN THE AntiFraud_Engine SHALL 依据既有惩罚机制对所有参与者发起事后追溯惩罚、撤销该次认定结果并回收因该次认定铸造的 MeriToken，并将处理结果锚定到 L1_Settlement。
7. THE AntiFraud_Engine SHALL 通过零知识证明保护投票者身份，仅公开投票结果。
8. IF 收到任何以货币注资兑换 MeriToken 或购买贡献认定的请求，THEN THE GMC_Base SHALL 拒绝该请求、不铸造任何 MeriToken 且不变更任何认定结果，并返回操作不被允许的错误指示。

### Requirement 12: 碳积分转 MeriToken 应用场景

**User Story:** 作为环境保护领域参与者，我希望将碳积分通过事后申报转化为 MeriToken，以便已发生的减碳贡献被功勋链承认。

#### Acceptance Criteria

1. WHERE 环境保护 Nested_Merit_Chain 已启用碳积分场景，THE Recording_Service SHALL 接受以可验证碳积分凭证引用为证据的贡献申报，并将其导入 Retroactive_Declaration 事后申报流程。
2. WHEN 一份碳积分申报经 Retroactive_Review_Module 审核与干系人投票通过，THE Minting_Service SHALL 按环境保护 Nested_Merit_Chain 的 Evaluation_Mechanism 与三维评分模型铸造 MeriToken。
3. THE Scoring_Engine SHALL 将碳积分减碳贡献按三维分类规则归入至少一个适用维度（Thought、Training 或 Technique），并对每个适用维度应用其对应的 Inflation_Index。
4. IF 一份碳积分证据的凭证引用无法通过可复盘性校验（凭证引用不可独立核验或不对应可追溯的减碳行为），THEN THE Retroactive_Review_Module SHALL 拒绝该申报、不铸造任何 MeriToken、不消耗任何 Quota，并返回证据无效错误。
5. WHEN 一份碳积分申报铸造 MeriToken，THE Minting_Service SHALL 将该次铸造计入环境保护 Nested_Merit_Chain 当前 Refresh_Period 的 Quota 消耗。
6. IF 一份碳积分申报所引用的凭证已被标记为已转化，THEN THE Retroactive_Review_Module SHALL 拒绝该申报、不铸造任何 MeriToken、不消耗任何 Quota，并返回重复转化错误。
7. WHEN 一份碳积分申报成功铸造 MeriToken，THE Minting_Service SHALL 将该申报所引用的碳积分凭证标记为已转化，以防止其再次转化。

### Requirement 13: 分层架构集成

**User Story:** 作为协议架构师，我希望嵌套功勋链、三维评分与登记授予流程统一接入 Substrate L1 + ZK Rollup L2 分层架构，以便兼顾免费高频记录与数学保证的结算安全。

#### Acceptance Criteria

1. THE L1_Settlement SHALL 存储功勋链注册记录、身份注册记录、治理投票结果、惩罚记录与状态根。
2. THE L2_Rollup SHALL 处理贡献记录创建、MeriToken 计算与亲密度更新，并在贡献记录提交后 5 秒内返回该记录对应的计算结果。
3. WHEN L2_Rollup 累计待结算贡献记录达到 1,000 条或距上一批次提交已满 60 秒（以先到者为准），THE L2_Rollup SHALL 向 L1_Settlement 提交该批次的零知识证明。
4. THE L1_Settlement SHALL 对每条上链交易不收取交易手续费。
5. WHERE 启用分片扩展能力，WHEN 全网贡献记录提交速率持续超过单个 Rollup 实例的额定吞吐上限（默认 1,000 条/秒）达 60 秒以上，THE L2_Rollup SHALL 新增并行 Rollup 实例（分片）以扩展处理能力，直至全网提交速率不再超过所有在用实例额定吞吐上限之和。
6. THE L1_Settlement SHALL 采用 Substrate 默认共识（GRANDPA / BABE）。
7. THE L2_Rollup SHALL 采用 BFT 类共识，并在不超过 3 秒内完成每个区块的最终确认。
8. IF L1_Settlement 对某批次提交的零知识证明验证失败，THEN THE L1_Settlement SHALL 拒绝该批次的状态更新、保留上一已确认状态根不变，并向 L2_Rollup 返回证明验证失败错误。
