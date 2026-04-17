use super::{
    EditAccountOperationInput, MonitorExternalCanisterOperationDTO,
    MonitorExternalCanisterOperationInput, TimestampRfc3339, TransferOperationDTO,
    TransferOperationInput,
};
use crate::{
    AddAccountOperationDTO, AddAccountOperationInput, AddAddressBookEntryOperationDTO,
    AddAddressBookEntryOperationInput, AddAssetOperationDTO, AddAssetOperationInput,
    AddNamedRuleOperationDTO, AddNamedRuleOperationInput, AddUserGroupOperationDTO,
    AddUserGroupOperationInput, AddUserOperationDTO, AddUserOperationInput,
    CallExternalCanisterOperationDTO, CallExternalCanisterOperationInput,
    ChangeExternalCanisterOperationDTO, ChangeExternalCanisterOperationInput,
    ConfigureExternalCanisterOperationDTO, ConfigureExternalCanisterOperationInput,
    CreateExternalCanisterOperationDTO, CreateExternalCanisterOperationInput, DisplayUserDTO,
    EditAccountOperationDTO, EditAddressBookEntryOperationDTO, EditAddressBookEntryOperationInput,
    EditAssetOperationDTO, EditAssetOperationInput, EditNamedRuleOperationDTO,
    EditNamedRuleOperationInput, EditPermissionOperationDTO, EditPermissionOperationInput,
    EditUserGroupOperationDTO, EditUserGroupOperationInput, EditUserOperationDTO,
    EditUserOperationInput, FundExternalCanisterOperationDTO, FundExternalCanisterOperationInput,
    ManageSystemInfoOperationDTO, ManageSystemInfoOperationInput, PaginationInput,
    PruneExternalCanisterOperationDTO, PruneExternalCanisterOperationInput,
    RemoveAddressBookEntryOperationDTO, RemoveAddressBookEntryOperationInput,
    RemoveAssetOperationDTO, RemoveAssetOperationInput, RemoveNamedRuleOperationDTO,
    RemoveNamedRuleOperationInput, RemoveUserGroupOperationDTO, RemoveUserGroupOperationInput,
    RequestEvaluationResultDTO, RequestPolicyRuleDTO, RequestSpecifierDTO,
    RestoreExternalCanisterOperationDTO, RestoreExternalCanisterOperationInput,
    SetDisasterRecoveryOperationDTO, SetDisasterRecoveryOperationInput,
    SnapshotExternalCanisterOperationDTO, SnapshotExternalCanisterOperationInput, SortDirection,
    SystemRestoreOperationDTO, SystemRestoreOperationInput, SystemUpgradeOperationDTO,
    SystemUpgradeOperationInput, UuidDTO,
};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestStatusDTO {
    Created,
    Approved,
    Rejected,
    Cancelled { reason: Option<String> },
    Scheduled { scheduled_at: TimestampRfc3339 },
    Processing { started_at: TimestampRfc3339 },
    Completed { completed_at: TimestampRfc3339 },
    Failed { reason: Option<String> },
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RequestStatusCodeDTO {
    Created = 0,
    Approved = 1,
    Rejected = 2,
    Cancelled = 3,
    Scheduled = 4,
    Processing = 5,
    Completed = 6,
    Failed = 7,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RequestApprovalStatusDTO {
    Approved,
    Rejected,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestExecutionScheduleDTO {
    Immediate,
    Scheduled { execution_time: TimestampRfc3339 },
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestOperationDTO {
    Transfer(Box<TransferOperationDTO>),
    AddAccount(Box<AddAccountOperationDTO>),
    EditAccount(Box<EditAccountOperationDTO>),
    AddAddressBookEntry(Box<AddAddressBookEntryOperationDTO>),
    EditAddressBookEntry(Box<EditAddressBookEntryOperationDTO>),
    RemoveAddressBookEntry(Box<RemoveAddressBookEntryOperationDTO>),
    AddUser(Box<AddUserOperationDTO>),
    EditUser(Box<EditUserOperationDTO>),
    AddUserGroup(Box<AddUserGroupOperationDTO>),
    EditUserGroup(Box<EditUserGroupOperationDTO>),
    RemoveUserGroup(Box<RemoveUserGroupOperationDTO>),
    SystemUpgrade(Box<SystemUpgradeOperationDTO>),
    SystemRestore(Box<SystemRestoreOperationDTO>),
    SetDisasterRecovery(Box<SetDisasterRecoveryOperationDTO>),
    ChangeExternalCanister(Box<ChangeExternalCanisterOperationDTO>),
    CreateExternalCanister(Box<CreateExternalCanisterOperationDTO>),
    ConfigureExternalCanister(Box<ConfigureExternalCanisterOperationDTO>),
    CallExternalCanister(Box<CallExternalCanisterOperationDTO>),
    FundExternalCanister(Box<FundExternalCanisterOperationDTO>),
    MonitorExternalCanister(Box<MonitorExternalCanisterOperationDTO>),
    SnapshotExternalCanister(Box<SnapshotExternalCanisterOperationDTO>),
    RestoreExternalCanister(Box<RestoreExternalCanisterOperationDTO>),
    PruneExternalCanister(Box<PruneExternalCanisterOperationDTO>),
    EditPermission(Box<EditPermissionOperationDTO>),
    AddRequestPolicy(Box<AddRequestPolicyOperationDTO>),
    EditRequestPolicy(Box<EditRequestPolicyOperationDTO>),
    RemoveRequestPolicy(Box<RemoveRequestPolicyOperationDTO>),
    ManageSystemInfo(Box<ManageSystemInfoOperationDTO>),
    AddAsset(Box<AddAssetOperationDTO>),
    EditAsset(Box<EditAssetOperationDTO>),
    RemoveAsset(Box<RemoveAssetOperationDTO>),
    AddNamedRule(Box<AddNamedRuleOperationDTO>),
    EditNamedRule(Box<EditNamedRuleOperationDTO>),
    RemoveNamedRule(Box<RemoveNamedRuleOperationDTO>),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestOperationInput {
    Transfer(TransferOperationInput),
    AddAccount(AddAccountOperationInput),
    EditAccount(EditAccountOperationInput),
    AddAddressBookEntry(AddAddressBookEntryOperationInput),
    EditAddressBookEntry(EditAddressBookEntryOperationInput),
    RemoveAddressBookEntry(RemoveAddressBookEntryOperationInput),
    AddUser(AddUserOperationInput),
    EditUser(EditUserOperationInput),
    AddUserGroup(AddUserGroupOperationInput),
    EditUserGroup(EditUserGroupOperationInput),
    RemoveUserGroup(RemoveUserGroupOperationInput),
    SystemUpgrade(SystemUpgradeOperationInput),
    SystemRestore(SystemRestoreOperationInput),
    SetDisasterRecovery(SetDisasterRecoveryOperationInput),
    ChangeExternalCanister(ChangeExternalCanisterOperationInput),
    CreateExternalCanister(CreateExternalCanisterOperationInput),
    ConfigureExternalCanister(ConfigureExternalCanisterOperationInput),
    CallExternalCanister(CallExternalCanisterOperationInput),
    FundExternalCanister(FundExternalCanisterOperationInput),
    MonitorExternalCanister(MonitorExternalCanisterOperationInput),
    SnapshotExternalCanister(SnapshotExternalCanisterOperationInput),
    RestoreExternalCanister(RestoreExternalCanisterOperationInput),
    PruneExternalCanister(PruneExternalCanisterOperationInput),
    EditPermission(EditPermissionOperationInput),
    AddRequestPolicy(AddRequestPolicyOperationInput),
    EditRequestPolicy(EditRequestPolicyOperationInput),
    RemoveRequestPolicy(RemoveRequestPolicyOperationInput),
    ManageSystemInfo(ManageSystemInfoOperationInput),
    AddAsset(AddAssetOperationInput),
    EditAsset(EditAssetOperationInput),
    RemoveAsset(RemoveAssetOperationInput),
    AddNamedRule(AddNamedRuleOperationInput),
    EditNamedRule(EditNamedRuleOperationInput),
    RemoveNamedRule(RemoveNamedRuleOperationInput),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum RequestOperationTypeDTO {
    Transfer,
    AddAccount,
    EditAccount,
    AddAddressBookEntry,
    EditAddressBookEntry,
    RemoveAddressBookEntry,
    AddUser,
    EditUser,
    AddUserGroup,
    EditUserGroup,
    RemoveUserGroup,
    SystemUpgrade,
    SystemRestore,
    SetDisasterRecovery,
    ChangeExternalCanister,
    CreateExternalCanister,
    CallExternalCanister,
    FundExternalCanister,
    MonitorExternalCanister,
    SnapshotExternalCanister,
    RestoreExternalCanister,
    PruneExternalCanister,
    EditPermission,
    AddRequestPolicy,
    EditRequestPolicy,
    RemoveRequestPolicy,
    ManageSystemInfo,
    ConfigureExternalCanister,
    AddAsset,
    EditAsset,
    RemoveAsset,
    AddNamedRule,
    EditNamedRule,
    RemoveNamedRule,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ListRequestsOperationTypeDTO {
    Transfer(Option<UuidDTO>),
    AddAccount,
    EditAccount,
    AddAddressBookEntry,
    EditAddressBookEntry,
    RemoveAddressBookEntry,
    AddUser,
    EditUser,
    AddUserGroup,
    EditUserGroup,
    RemoveUserGroup,
    SystemUpgrade,
    SystemRestore,
    ChangeExternalCanister(Option<Principal>),
    CreateExternalCanister,
    CallExternalCanister(Option<Principal>),
    FundExternalCanister(Option<Principal>),
    MonitorExternalCanister(Option<Principal>),
    SnapshotExternalCanister(Option<Principal>),
    RestoreExternalCanister(Option<Principal>),
    PruneExternalCanister(Option<Principal>),
    EditPermission,
    AddRequestPolicy,
    EditRequestPolicy,
    RemoveRequestPolicy,
    ManageSystemInfo,
    SetDisasterRecovery,
    ConfigureExternalCanister(Option<Principal>),
    AddAsset,
    EditAsset,
    RemoveAsset,
    AddNamedRule,
    EditNamedRule,
    RemoveNamedRule,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestApprovalDTO {
    pub approver_id: UuidDTO,
    pub status: RequestApprovalStatusDTO,
    pub status_reason: Option<String>,
    pub decided_at: TimestampRfc3339,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestDTO {
    pub id: UuidDTO,
    pub title: String,
    pub summary: Option<String>,
    pub operation: RequestOperationDTO,
    pub requested_by: UuidDTO,
    pub approvals: Vec<RequestApprovalDTO>,
    pub created_at: TimestampRfc3339,
    pub status: RequestStatusDTO,
    pub expiration_dt: TimestampRfc3339,
    pub execution_plan: RequestExecutionScheduleDTO,
    pub deduplication_key: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestCallerPrivilegesDTO {
    pub id: UuidDTO,
    pub can_approve: bool,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RequestAdditionalInfoDTO {
    pub id: UuidDTO,
    pub requester_name: String,
    pub approvers: Vec<DisplayUserDTO>,
    pub evaluation_result: Option<RequestEvaluationResultDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CreateRequestInput {
    pub operation: RequestOperationInput,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub execution_plan: Option<RequestExecutionScheduleDTO>,
    pub expiration_dt: Option<TimestampRfc3339>,
    pub deduplication_key: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CancelRequestInput {
    pub request_id: UuidDTO,
    pub reason: Option<String>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CancelRequestResponse {
    pub request: RequestDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct SubmitRequestApprovalInput {
    pub decision: RequestApprovalStatusDTO,
    pub request_id: UuidDTO,
    pub reason: Option<String>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct SubmitRequestApprovalResponse {
    pub request: RequestDTO,
    pub privileges: RequestCallerPrivilegesDTO,
    pub additional_info: RequestAdditionalInfoDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetRequestInput {
    pub request_id: UuidDTO,
    pub with_full_info: Option<bool>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetRequestResponse {
    pub request: RequestDTO,
    pub privileges: RequestCallerPrivilegesDTO,
    pub additional_info: RequestAdditionalInfoDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub enum ListRequestsSortBy {
    CreatedAt(SortDirection),
    ExpirationDt(SortDirection),
    LastModificationDt(SortDirection),
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListRequestsInput {
    pub requester_ids: Option<Vec<UuidDTO>>,
    pub approver_ids: Option<Vec<UuidDTO>>,
    pub statuses: Option<Vec<RequestStatusCodeDTO>>,
    pub operation_types: Option<Vec<ListRequestsOperationTypeDTO>>,
    pub expiration_from_dt: Option<TimestampRfc3339>,
    pub expiration_to_dt: Option<TimestampRfc3339>,
    pub created_from_dt: Option<TimestampRfc3339>,
    pub created_to_dt: Option<TimestampRfc3339>,
    pub paginate: Option<PaginationInput>,
    pub sort_by: Option<ListRequestsSortBy>,
    pub only_approvable: bool,
    pub with_evaluation_results: bool,
    pub deduplication_keys: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListRequestsResponse {
    pub requests: Vec<RequestDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<RequestCallerPrivilegesDTO>,
    pub additional_info: Vec<RequestAdditionalInfoDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetNextApprovableRequestInput {
    pub excluded_request_ids: Vec<UuidDTO>,
    pub operation_types: Option<Vec<ListRequestsOperationTypeDTO>>,
    pub sort_by: Option<ListRequestsSortBy>,
}

pub type GetNextApprovableRequestResponse = Option<GetRequestResponse>;

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct CreateRequestResponse {
    pub request: RequestDTO,
    pub privileges: RequestCallerPrivilegesDTO,
    pub additional_info: RequestAdditionalInfoDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AddRequestPolicyOperationInput {
    pub specifier: RequestSpecifierDTO,
    pub rule: RequestPolicyRuleDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct AddRequestPolicyOperationDTO {
    pub policy_id: Option<UuidDTO>,
    pub input: AddRequestPolicyOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct EditRequestPolicyOperationInput {
    pub policy_id: UuidDTO,
    pub specifier: Option<RequestSpecifierDTO>,
    pub rule: Option<RequestPolicyRuleDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct EditRequestPolicyOperationDTO {
    pub input: EditRequestPolicyOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RemoveRequestPolicyOperationInput {
    pub policy_id: UuidDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RemoveRequestPolicyOperationDTO {
    pub input: RemoveRequestPolicyOperationInput,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiErrorDTO;

    // Regression test: station 0.6.0 returns `tags: null` in CreateRequestResponse.
    // Verifies that RequestDTO with `tags: Option<Vec<String>>` can decode such responses.
    #[test]
    fn test_decode_create_request_response_with_null_tags() {
        let hex = "4449444ce1016b02bc8a0101c5fed201dd016c03e2acedf30102cfbe93a4040386ec9ee30ed1016c02dbb70171dee5e28e0e7e6c0adbb70171b2ceef2f0498abec810171b0aefca5030ae0f3bfbe0371aaacd9d00671c8b6fa930771a696a4870806a78882820a0c90b0858b0fce016b08ddf3cce401059795c8e4067f9ef7d6a9087f8dc494da0a07f1cc9da00c05f3bb99ef0c08c8bbeff50d7feb82ae880f096c01c49ff4e40f066e716c018586a8bf0a716c01f1808f960f716c01e7a4acda0c716b02d1e78f8e077f8dc494da0a0b6c01d4ea86bb03716b22ecbdff440db39db7590fd987d285011396b8f0fa0128bcadb9ba032acbb3e7dc042df88ad6e40436c2caddf804e200acecae8605e400ecf9efb905ed00e69681df05f300eaeaecf105f500b894d89606f70097b899c806fa009dcbb7b3078001af98a1ca078201cdc29fd6078601b9f7fc88088901ffa79a92088b01cea594b6088e01ff9ae2ea089c0197e2f8ab099e0195a3d8bb0aa101e2abc9bf0aa401cbd6fda00ba601a3cfd2bc0bb001d6bef6c10bb801e09be7ef0cbc01d1d0c2960dbe01f087c8ab0ec001f0c085810fc20194e6b1c40fc601e095bee70fca01acd9d4fc0fcc016c018a88f7f00b0e6c018abce7cd0c716c028baeb3fe04108a88f7f00b126e116c02dbb70171cbe4fdc704716c01cbe4fdc704716c018a88f7f00b146c04ceb98195011588a6bd9b0824bdac84c70826c8dfc9d10a246b0caf8ed88e01168b86f5a002188f96dcbb0219ebfc9ac4031ab2ffc0c6031b8dc9acf50621ddc7fed20722d0b39bcb0b22b5cae3970d22d481e9ac0e22afeafde30f2381c981e60f226b02fe90b194037ff6ee81b403176b02fb7f71cce5c6017f6b02fe90b194037fa9f4e7dc0b176b04fcb88b84037fe2abc9bf0a7f9dc6bed70e7fd6b496dd0f7f6b04fe90b194037ff6ee81b403179cb68f850b7fa9f4e7dc0b176b06be89cde4021ce5aef8f40220fe90b194037ff6ee81b403209cb68f850b7f908b9ce00e206c02e8ff872e1de7cae7b8071f6b02cce5c6017ff9c0ed830d1e6c02b3c4b1f20468a98f98ec08716b02e188017f9a88bfeb021e6b02cce5c6017fc7bc99b407686b05fe90b194037ff6ee81b403179cb68f850b7fcbd6fda00b17a9f4e7dc0b176b05fe90b194037ff6ee81b40317cbe8b4fb09179cb68f850b7fa9f4e7dc0b176b02f6ee81b4037fa9f4e7dc0b7f6e256d716e276b038fd597b5027fc9e99fdc097fbbdd99b00a7f6c028a88f7f00b29b6b897890f066c03ebe3d2247eefc1e4cf0106b3c4b1f204686c018a88f7f00b2b6c02b3c4b1f20468aae6d1c40c2c6b03c4b0d08d027191ecada0087fcfe2f08d0d7f6c018a88f7f00b2e6c04cbe4fdc70406fcf1cddd042ffc91f4f80535b891eae70f716e306b09f3a1ee0130bfdefe5331868b90dd017fb9d6a9a10631a6fafae1097ff8f083a60b338386f5b00b338cab86f20b34b5cae3970d716c0284e4c5a1057acecc918b0f326b03fb7f25cce5c6017f9fc688e902256d306c029f93c60271f1fee18d03716e066c02d4c2a7b80437b3c4b1f204686b04d5c1c780017f83f2bca70638cbe8b4fb097fba8bdea90adc006c0784f9d17639cbe4fdc70406ffcff4cb0424fc91f4f8050680cedad907c80091ecada008d700debede830ad9006e3a6c0395e4ed88043bd6969ddd04c400f0a2cabb0bc4006e3c6b03f5d7a4a60c3d82eb86bc0dc200a4e9d2e90dc6006d3e6c02e8ff872e7184f9d1763f6dc0006c0289c9ce8c02c100e7cae7b8071f6c0388a6bd9b0825bdac84c70827c8dfc9d10a256dc3006c0289c9ce8c02c40098dfbcdd0ac5006ec1006c02e8ff872e71e7cae7b8071f6dc7006c03e8ff872e7189c9ce8c02c100e7cae7b8071f6ec9006c0295e4ed8804ca00f0a2cabb0bd6006ecb006b048ba98d9f0225f5d7a4a60ccc0082eb86bc0dd000a4e9d2e90dd4006dcd006c02e8ff872e71d0e1f9990fce006dcf006c03fcf1cddd0430e7cae7b8071fc8b2ccef0e066dd1006c0298dfbcdd0ac500d0e1f9990fd2006dd3006c02fcf1cddd0430c8b2ccef0e066dd5006c04e8ff872e71fcf1cddd0430e7cae7b8071fc8b2ccef0e066ed2006ed8006b02e6ebead6047f82e580b90c7f6eda006b039bf883b709db00f8c0dcd60b25a4e9d2e90ddb006d346c07c0cff271dd00d7e09b9002de0080ad988a04dd00edd9c8c907e000f8e287cc0cdd00deebb5a90edd00a882acc60fdd006e7d6edf006d686ee1006b03d7e09b90027fa981ceb7067fcaa989aa08df006c04e3a683c304e300b3c4b1f20468f6959e9e05718cf88cf909066b03c8bb8a707f9ce9c699067f9baaebec087f6c02d4c2a7b804e500b3c4b1f204686b02c2c8e3b901e600a2b8d4b9037f6c02b4e2c99009e700f5ea9edb09ea006ee8006b039cfbd7ad057ff6e7c5860ae900ea90daa90ce9006c01edbeb705716b03ef85aedb077ddabda0df08eb00adec86fe0dec006c0287bc86b9027dda96c2cb0e7d6c05c3b8d2b3017897efbbf5037dd693a38d0678aace87c4067df5aafdf5097d6c02cba4b6ed04ee008a88f7f00bf2006eef006c06dbb70171b2ceef2ff000d4c8a0e101f100cbe4fdc70471fccdfec40c719c8ab7bb0ddf006b028ba7879f047fe6ebead6047f6d116c04b2ceef2ff000d4c8a0e10125cbe4fdc704719c8ab7bb0ddf006c018a88f7f00bf4006c0696bde0f50324cbe4fdc704069491a7cf0806debede830ad9008abce7cd0c71d8def6f60e066c018a88f7f00bf6006c02cbe4fdc70471cfa785e10b716c01bd84f4de06f8006ef9006c02cfa785e10b719ff6acaf0d7a6c018a88f7f00bfb006c03fcf1cddd042fbebafdf407fc00c8b2ccef0e716efd006b1cecbdff44fe00b39db7597fd987d28501ff00cbb3e7dc04fe00c2caddf80420ecf9efb9057fe69681df05fe00eaeaecf105fe00b894d896067f97b899c806fe009dcbb7b307fe00af98a1ca077fcdc29fd6077fb9f7fc8808fe00cea594b6087fff9ae2ea08fe0097e2f8ab092095a3d8bb0afe00e2abc9bf0a7fcbd6fda00bfe00a3cfd2bc0bfe00d6bef6c10b7fe09be7ef0c7fd1d0c2960dfe00f087c8ab0efe00f0c085810f1c94e6b1c40f7facd9d4fc0f7f6b02cce5c6017f98f8de01256b02cce5c6017feeb1c8e20e156c018a88f7f00b81016c01c8b2ccef0e716c02f0d3e5b10283018a88f7f00b85016e84016c07dbb70171c295a993017996bde0f50325efcee78004db00cbe4fdc704719491a7cf0871d8def6f60e716c06c295a993017996bde0f50325efcee78004db00cbe4fdc704719491a7cf0871d8def6f60e716c04e9aff7a2048701f6959e9e0571d1e6b3b70888018cf88cf909066e7e6b02f291afbd067fd8b2d9af0d7f6c018a88f7f00b8a016c01d3cfefe20c716c018a88f7f00b8c016c02d1e6b3b7088d01b6b897890f716b02a4fb9da00a7fe68195950b7f6c02b3c4b1f2048f018a88f7f00b90016e686c0784f9d1769101efcee780049201d4c2a7b8049301cbe4fdc70471ffcff4cb0424fc91f4f8050680cedad9079b016c0395e4ed8804c600d6969ddd04c100f0a2cabb0bc1006edb006b02accbbfa00a9401a48dd2dd0e95016c01b3c4b1f204686c02a8ddc0639601aac2fafd0497016e786e98016b02d8a7a6ce0b9901fd95eef40c9a016c019ced879c0e066c01ddad9cd009686c0295e4ed8804d400f0a2cabb0bd2006c018a88f7f00b9d016c04ffcff4cb0424debede830ad900d3cfefe20c71a8a0ae910e066c02d4c2a7b8049f01b3c4b1f204686b01e8f2a6b903a0016c018daacd9408786c018a88f7f00ba2016c06dbb70171b2ceef2fa301d4c8a0e1012491b4bfe8038701cbe4fdc704069c8ab7bb0dde006ef0006c018a88f7f00ba5016c04cbe4fdc70406b4e2c99009e700e099bea90e9601d8c3a6a40f96016c06c6fcb602dd009b84c186038401ae89f3cd0511af9ff985070698ebdd8b0aa7018a88f7f00baf016ea8016c08dbb70171f0c4e5452fefcee78004db00cbe4fdc7047183918de004a901a2becae008ad01f68fe6b20a2ffccdfec40c716daa016c029cbab69c02ab018abce7cd0c716eac016c06edbeb70571c295a99301799cbab69c027da9fe83be0471dafc8cec09718abce7cd0c716dae016c02b4e3ade80971b79ebaec0f716c08fbca0171c6fcb602dd00d68fd7c80171e29682ed0171efcee78004db00ae89f3cd0510d8a38ca80d7dbfe5989b0e716c018a88f7f00bb1016c08edbeb70571f0c4e545b20198cdf6db01c400dddcc39102c400cbe4fdc70406b2adf7ef04b401f68fe6b20ab201e3b189910dc4006eb3016b0282a7fd0130e4a0f4c8037f6eb5016b02bad89abe04b601908b9ce00eb7016c0183918de004256c02a1bccda50525de9c8eaf0c256c0287b1bd9a07b9018a88f7f00bbb016eba016c08dbb70171efcee78004db00ffcff4cb04259491a7cf0871b4e3ade80971fccdfec40c71a29bca910d71a8a0ae910e716c06efcee78004db00ffcff4cb04259491a7cf0871b4e3ade80971a29bca910d71a8a0ae910e716c028a88f7f00bbd01c8b2ccef0e066c02fcf1cddd0430bebafdf407fd006c018a88f7f00bbf016c01b891eae70f716c018a88f7f00bc1016c01cfa785e10b716c07d6fca702c301e8ff872e1ee7cae7b807c5018cf88cf90906c4e7b6d60a9601e3b987be0b06d3dcb0ff0bc3016ec4016d7b6e1e6c0282cdfdd204c7018a88f7f00bc9016ec8016c04dbb70171cbe4fdc70471fcf1cddd0430fc91f4f805066c03cbe4fdc70471fcf1cddd0430fc91f4f805066c018a88f7f00bcb016c02b3c4b1f20468b6b897890f716c02adf9e78a0aa7018a88f7f00bcd016c08f0c4e5452f98cdf6db01c100dddcc39102c100efcee78004db00cbe4fdc7047183918de00425f68fe6b20a2fe3b189910dc1006dcf016c04b2ceef2fd0019589d9ff0571f1c8b5da0906d689bec50e716b029795c8e4067f9ef7d6a9087f6c04dbb70171c0ee98f009d201ae86b7a50a71cecc918b0ff1006ed3016c048b9cc30871b2ceef2fd4018dc7f4800ad501e9a0f7c10bd8016b039795c8e4067f9ef7d6a9087fb780f7c90f7f6ed6016dd7016b04e78ae9a3057ff6cfd0c5067fa6fafae1097fa2e580b20f7f6dd9016c02b2ceef2fd40190a68dca05da016b08f3a1ee01d901bfdefe53db01868b90dd017fb9d6a9a106db01a6fafae1097ff8f083a60bd8018386f5b00bd8018cab86f20bdc016c039bad8a96027884e4c5a10578cecc918b0f256c01efcee78004346c03ade2928e0471c7ebc4d00906c2b9dbda0ade016edf016de0016c02007101710100002461353432336431642d363532352d343338312d623464632d346231663362663935383332002461353432336431642d363532352d343338312d623464632d346231663362663935383332060d43616c6c2063616e6973746572001e323032362d30352d31365430383a33363a33322e3530333531313134335a1e323032362d30342d31365430383a33363a33322e3530333531313134345a2462633038373039322d666136662d346332322d613762342d626564343263386532643731001e00010a0000000001801b2b010115636f6d6d69745f70726f706f7365645f626174636800014036343461326463333232383439633134613965353861303166633462313533633631653636613030363066656639633134383335666638343666326236313336000000002461353432336431642d363532352d343338312d623464632d346231663362663935383332012461353432336431642d363532352d343338312d623464632d34623166336266393538333202010103010201020000000000000001000000000000000002434900";
        let bytes = hex::decode(hex).expect("invalid hex");
        let result: Result<Result<CreateRequestResponse, ApiErrorDTO>, _> =
            candid::decode_one(&bytes);
        assert!(
            result.is_ok(),
            "failed to decode response with null tags: {:?}",
            result.err()
        );
        let response = result.unwrap().unwrap();
        // tags should be None (null from older canister)
        assert!(
            response.request.tags.is_none(),
            "expected tags to be None, got {:?}",
            response.request.tags
        );
    }
}
