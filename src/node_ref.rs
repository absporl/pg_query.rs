use crate::*;

#[derive(Debug, Copy, Clone)]
pub enum NodeRef<'a> {
    Alias(&'a protobuf::Alias),
    RangeVar(&'a protobuf::RangeVar),
    TableFunc(&'a protobuf::TableFunc),
    Expr(&'a protobuf::Expr),
    Var(&'a protobuf::Var),
    Param(&'a protobuf::Param),
    Aggref(&'a protobuf::Aggref),
    GroupingFunc(&'a protobuf::GroupingFunc),
    WindowFunc(&'a protobuf::WindowFunc),
    SubscriptingRef(&'a protobuf::SubscriptingRef),
    FuncExpr(&'a protobuf::FuncExpr),
    NamedArgExpr(&'a protobuf::NamedArgExpr),
    OpExpr(&'a protobuf::OpExpr),
    DistinctExpr(&'a protobuf::DistinctExpr),
    NullIfExpr(&'a protobuf::NullIfExpr),
    ScalarArrayOpExpr(&'a protobuf::ScalarArrayOpExpr),
    BoolExpr(&'a protobuf::BoolExpr),
    SubLink(&'a protobuf::SubLink),
    SubPlan(&'a protobuf::SubPlan),
    AlternativeSubPlan(&'a protobuf::AlternativeSubPlan),
    FieldSelect(&'a protobuf::FieldSelect),
    FieldStore(&'a protobuf::FieldStore),
    RelabelType(&'a protobuf::RelabelType),
    CoerceViaIo(&'a protobuf::CoerceViaIo),
    ArrayCoerceExpr(&'a protobuf::ArrayCoerceExpr),
    ConvertRowtypeExpr(&'a protobuf::ConvertRowtypeExpr),
    CollateExpr(&'a protobuf::CollateExpr),
    CaseExpr(&'a protobuf::CaseExpr),
    CaseWhen(&'a protobuf::CaseWhen),
    CaseTestExpr(&'a protobuf::CaseTestExpr),
    ArrayExpr(&'a protobuf::ArrayExpr),
    RowExpr(&'a protobuf::RowExpr),
    RowCompareExpr(&'a protobuf::RowCompareExpr),
    CoalesceExpr(&'a protobuf::CoalesceExpr),
    MinMaxExpr(&'a protobuf::MinMaxExpr),
    SqlvalueFunction(&'a protobuf::SqlValueFunction),
    XmlExpr(&'a protobuf::XmlExpr),
    NullTest(&'a protobuf::NullTest),
    BooleanTest(&'a protobuf::BooleanTest),
    CoerceToDomain(&'a protobuf::CoerceToDomain),
    CoerceToDomainValue(&'a protobuf::CoerceToDomainValue),
    SetToDefault(&'a protobuf::SetToDefault),
    CurrentOfExpr(&'a protobuf::CurrentOfExpr),
    NextValueExpr(&'a protobuf::NextValueExpr),
    InferenceElem(&'a protobuf::InferenceElem),
    TargetEntry(&'a protobuf::TargetEntry),
    RangeTblRef(&'a protobuf::RangeTblRef),
    JoinExpr(&'a protobuf::JoinExpr),
    FromExpr(&'a protobuf::FromExpr),
    OnConflictExpr(&'a protobuf::OnConflictExpr),
    IntoClause(&'a protobuf::IntoClause),
    RawStmt(&'a protobuf::RawStmt),
    Query(&'a protobuf::Query),
    InsertStmt(&'a protobuf::InsertStmt),
    DeleteStmt(&'a protobuf::DeleteStmt),
    UpdateStmt(&'a protobuf::UpdateStmt),
    SelectStmt(&'a protobuf::SelectStmt),
    AlterTableStmt(&'a protobuf::AlterTableStmt),
    AlterTableCmd(&'a protobuf::AlterTableCmd),
    AlterDomainStmt(&'a protobuf::AlterDomainStmt),
    SetOperationStmt(&'a protobuf::SetOperationStmt),
    GrantStmt(&'a protobuf::GrantStmt),
    GrantRoleStmt(&'a protobuf::GrantRoleStmt),
    AlterDefaultPrivilegesStmt(&'a protobuf::AlterDefaultPrivilegesStmt),
    ClosePortalStmt(&'a protobuf::ClosePortalStmt),
    ClusterStmt(&'a protobuf::ClusterStmt),
    CopyStmt(&'a protobuf::CopyStmt),
    CreateStmt(&'a protobuf::CreateStmt),
    DefineStmt(&'a protobuf::DefineStmt),
    DropStmt(&'a protobuf::DropStmt),
    TruncateStmt(&'a protobuf::TruncateStmt),
    CommentStmt(&'a protobuf::CommentStmt),
    FetchStmt(&'a protobuf::FetchStmt),
    IndexStmt(&'a protobuf::IndexStmt),
    CreateFunctionStmt(&'a protobuf::CreateFunctionStmt),
    AlterFunctionStmt(&'a protobuf::AlterFunctionStmt),
    DoStmt(&'a protobuf::DoStmt),
    RenameStmt(&'a protobuf::RenameStmt),
    RuleStmt(&'a protobuf::RuleStmt),
    NotifyStmt(&'a protobuf::NotifyStmt),
    ListenStmt(&'a protobuf::ListenStmt),
    UnlistenStmt(&'a protobuf::UnlistenStmt),
    TransactionStmt(&'a protobuf::TransactionStmt),
    ViewStmt(&'a protobuf::ViewStmt),
    LoadStmt(&'a protobuf::LoadStmt),
    CreateDomainStmt(&'a protobuf::CreateDomainStmt),
    CreatedbStmt(&'a protobuf::CreatedbStmt),
    DropdbStmt(&'a protobuf::DropdbStmt),
    VacuumStmt(&'a protobuf::VacuumStmt),
    ExplainStmt(&'a protobuf::ExplainStmt),
    CreateTableAsStmt(&'a protobuf::CreateTableAsStmt),
    CreateSeqStmt(&'a protobuf::CreateSeqStmt),
    AlterSeqStmt(&'a protobuf::AlterSeqStmt),
    VariableSetStmt(&'a protobuf::VariableSetStmt),
    VariableShowStmt(&'a protobuf::VariableShowStmt),
    DiscardStmt(&'a protobuf::DiscardStmt),
    CreateTrigStmt(&'a protobuf::CreateTrigStmt),
    CreatePlangStmt(&'a protobuf::CreatePLangStmt),
    CreateRoleStmt(&'a protobuf::CreateRoleStmt),
    AlterRoleStmt(&'a protobuf::AlterRoleStmt),
    DropRoleStmt(&'a protobuf::DropRoleStmt),
    LockStmt(&'a protobuf::LockStmt),
    ConstraintsSetStmt(&'a protobuf::ConstraintsSetStmt),
    ReindexStmt(&'a protobuf::ReindexStmt),
    CheckPointStmt(&'a protobuf::CheckPointStmt),
    CreateSchemaStmt(&'a protobuf::CreateSchemaStmt),
    AlterDatabaseStmt(&'a protobuf::AlterDatabaseStmt),
    AlterDatabaseSetStmt(&'a protobuf::AlterDatabaseSetStmt),
    AlterRoleSetStmt(&'a protobuf::AlterRoleSetStmt),
    CreateConversionStmt(&'a protobuf::CreateConversionStmt),
    CreateCastStmt(&'a protobuf::CreateCastStmt),
    CreateOpClassStmt(&'a protobuf::CreateOpClassStmt),
    CreateOpFamilyStmt(&'a protobuf::CreateOpFamilyStmt),
    AlterOpFamilyStmt(&'a protobuf::AlterOpFamilyStmt),
    PrepareStmt(&'a protobuf::PrepareStmt),
    ExecuteStmt(&'a protobuf::ExecuteStmt),
    DeallocateStmt(&'a protobuf::DeallocateStmt),
    DeclareCursorStmt(&'a protobuf::DeclareCursorStmt),
    CreateTableSpaceStmt(&'a protobuf::CreateTableSpaceStmt),
    DropTableSpaceStmt(&'a protobuf::DropTableSpaceStmt),
    AlterObjectDependsStmt(&'a protobuf::AlterObjectDependsStmt),
    AlterObjectSchemaStmt(&'a protobuf::AlterObjectSchemaStmt),
    AlterOwnerStmt(&'a protobuf::AlterOwnerStmt),
    AlterOperatorStmt(&'a protobuf::AlterOperatorStmt),
    AlterTypeStmt(&'a protobuf::AlterTypeStmt),
    DropOwnedStmt(&'a protobuf::DropOwnedStmt),
    ReassignOwnedStmt(&'a protobuf::ReassignOwnedStmt),
    CompositeTypeStmt(&'a protobuf::CompositeTypeStmt),
    CreateEnumStmt(&'a protobuf::CreateEnumStmt),
    CreateRangeStmt(&'a protobuf::CreateRangeStmt),
    AlterEnumStmt(&'a protobuf::AlterEnumStmt),
    AlterTsdictionaryStmt(&'a protobuf::AlterTsDictionaryStmt),
    AlterTsconfigurationStmt(&'a protobuf::AlterTsConfigurationStmt),
    CreateFdwStmt(&'a protobuf::CreateFdwStmt),
    AlterFdwStmt(&'a protobuf::AlterFdwStmt),
    CreateForeignServerStmt(&'a protobuf::CreateForeignServerStmt),
    AlterForeignServerStmt(&'a protobuf::AlterForeignServerStmt),
    CreateUserMappingStmt(&'a protobuf::CreateUserMappingStmt),
    AlterUserMappingStmt(&'a protobuf::AlterUserMappingStmt),
    DropUserMappingStmt(&'a protobuf::DropUserMappingStmt),
    AlterTableSpaceOptionsStmt(&'a protobuf::AlterTableSpaceOptionsStmt),
    AlterTableMoveAllStmt(&'a protobuf::AlterTableMoveAllStmt),
    SecLabelStmt(&'a protobuf::SecLabelStmt),
    CreateForeignTableStmt(&'a protobuf::CreateForeignTableStmt),
    ImportForeignSchemaStmt(&'a protobuf::ImportForeignSchemaStmt),
    CreateExtensionStmt(&'a protobuf::CreateExtensionStmt),
    AlterExtensionStmt(&'a protobuf::AlterExtensionStmt),
    AlterExtensionContentsStmt(&'a protobuf::AlterExtensionContentsStmt),
    CreateEventTrigStmt(&'a protobuf::CreateEventTrigStmt),
    AlterEventTrigStmt(&'a protobuf::AlterEventTrigStmt),
    RefreshMatViewStmt(&'a protobuf::RefreshMatViewStmt),
    ReplicaIdentityStmt(&'a protobuf::ReplicaIdentityStmt),
    AlterSystemStmt(&'a protobuf::AlterSystemStmt),
    CreatePolicyStmt(&'a protobuf::CreatePolicyStmt),
    AlterPolicyStmt(&'a protobuf::AlterPolicyStmt),
    CreateTransformStmt(&'a protobuf::CreateTransformStmt),
    CreateAmStmt(&'a protobuf::CreateAmStmt),
    CreatePublicationStmt(&'a protobuf::CreatePublicationStmt),
    AlterPublicationStmt(&'a protobuf::AlterPublicationStmt),
    CreateSubscriptionStmt(&'a protobuf::CreateSubscriptionStmt),
    AlterSubscriptionStmt(&'a protobuf::AlterSubscriptionStmt),
    DropSubscriptionStmt(&'a protobuf::DropSubscriptionStmt),
    CreateStatsStmt(&'a protobuf::CreateStatsStmt),
    AlterCollationStmt(&'a protobuf::AlterCollationStmt),
    CallStmt(&'a protobuf::CallStmt),
    AlterStatsStmt(&'a protobuf::AlterStatsStmt),
    AExpr(&'a protobuf::AExpr),
    ColumnRef(&'a protobuf::ColumnRef),
    ParamRef(&'a protobuf::ParamRef),
    AConst(&'a protobuf::AConst),
    FuncCall(&'a protobuf::FuncCall),
    AStar(&'a protobuf::AStar),
    AIndices(&'a protobuf::AIndices),
    AIndirection(&'a protobuf::AIndirection),
    AArrayExpr(&'a protobuf::AArrayExpr),
    ResTarget(&'a protobuf::ResTarget),
    MultiAssignRef(&'a protobuf::MultiAssignRef),
    TypeCast(&'a protobuf::TypeCast),
    CollateClause(&'a protobuf::CollateClause),
    SortBy(&'a protobuf::SortBy),
    WindowDef(&'a protobuf::WindowDef),
    RangeSubselect(&'a protobuf::RangeSubselect),
    RangeFunction(&'a protobuf::RangeFunction),
    RangeTableSample(&'a protobuf::RangeTableSample),
    RangeTableFunc(&'a protobuf::RangeTableFunc),
    RangeTableFuncCol(&'a protobuf::RangeTableFuncCol),
    TypeName(&'a protobuf::TypeName),
    ColumnDef(&'a protobuf::ColumnDef),
    IndexElem(&'a protobuf::IndexElem),
    Constraint(&'a protobuf::Constraint),
    DefElem(&'a protobuf::DefElem),
    RangeTblEntry(&'a protobuf::RangeTblEntry),
    RangeTblFunction(&'a protobuf::RangeTblFunction),
    TableSampleClause(&'a protobuf::TableSampleClause),
    WithCheckOption(&'a protobuf::WithCheckOption),
    SortGroupClause(&'a protobuf::SortGroupClause),
    GroupingSet(&'a protobuf::GroupingSet),
    WindowClause(&'a protobuf::WindowClause),
    ObjectWithArgs(&'a protobuf::ObjectWithArgs),
    AccessPriv(&'a protobuf::AccessPriv),
    CreateOpClassItem(&'a protobuf::CreateOpClassItem),
    TableLikeClause(&'a protobuf::TableLikeClause),
    FunctionParameter(&'a protobuf::FunctionParameter),
    LockingClause(&'a protobuf::LockingClause),
    RowMarkClause(&'a protobuf::RowMarkClause),
    XmlSerialize(&'a protobuf::XmlSerialize),
    WithClause(&'a protobuf::WithClause),
    InferClause(&'a protobuf::InferClause),
    OnConflictClause(&'a protobuf::OnConflictClause),
    CommonTableExpr(&'a protobuf::CommonTableExpr),
    RoleSpec(&'a protobuf::RoleSpec),
    TriggerTransition(&'a protobuf::TriggerTransition),
    PartitionElem(&'a protobuf::PartitionElem),
    PartitionSpec(&'a protobuf::PartitionSpec),
    PartitionBoundSpec(&'a protobuf::PartitionBoundSpec),
    PartitionRangeDatum(&'a protobuf::PartitionRangeDatum),
    PartitionCmd(&'a protobuf::PartitionCmd),
    VacuumRelation(&'a protobuf::VacuumRelation),
    InlineCodeBlock(&'a protobuf::InlineCodeBlock),
    CallContext(&'a protobuf::CallContext),
    Integer(&'a protobuf::Integer),
    Float(&'a protobuf::Float),
    String(&'a protobuf::String),
    BitString(&'a protobuf::BitString),
    Null(&'a protobuf::Null),
    List(&'a protobuf::List),
    IntList(&'a protobuf::IntList),
    OidList(&'a protobuf::OidList),
}

impl<'a> NodeRef<'a> {
    // TODO: `deparseStmt` in pg_query_deparse.c panics on unexpected nodes. instead, return a Rust error
    pub fn deparse(&self) -> Result<String> {
        crate::deparse(&protobuf::ParseResult {
            version: crate::bindings::PG_VERSION_NUM as i32,
            stmts: vec![
                protobuf::RawStmt {
                    stmt: Some(Box::new(Node { node: Some(self.to_enum()) })),
                    stmt_location: 0,
                    stmt_len: 0,
                }
            ]
        })
    }

    pub fn to_enum(&self) -> NodeEnum {
        match self {
            NodeRef::Alias(n) => NodeEnum::Alias((*n).clone()),
            NodeRef::RangeVar(n) => NodeEnum::RangeVar((*n).clone()),
            NodeRef::TableFunc(n) => NodeEnum::TableFunc(Box::new((*n).clone())),
            NodeRef::Expr(n) => NodeEnum::Expr((*n).clone()),
            NodeRef::Var(n) => NodeEnum::Var(Box::new((*n).clone())),
            NodeRef::Param(n) => NodeEnum::Param(Box::new((*n).clone())),
            NodeRef::Aggref(n) => NodeEnum::Aggref(Box::new((*n).clone())),
            NodeRef::GroupingFunc(n) => NodeEnum::GroupingFunc(Box::new((*n).clone())),
            NodeRef::WindowFunc(n) => NodeEnum::WindowFunc(Box::new((*n).clone())),
            NodeRef::SubscriptingRef(n) => NodeEnum::SubscriptingRef(Box::new((*n).clone())),
            NodeRef::FuncExpr(n) => NodeEnum::FuncExpr(Box::new((*n).clone())),
            NodeRef::NamedArgExpr(n) => NodeEnum::NamedArgExpr(Box::new((*n).clone())),
            NodeRef::OpExpr(n) => NodeEnum::OpExpr(Box::new((*n).clone())),
            NodeRef::DistinctExpr(n) => NodeEnum::DistinctExpr(Box::new((*n).clone())),
            NodeRef::NullIfExpr(n) => NodeEnum::NullIfExpr(Box::new((*n).clone())),
            NodeRef::ScalarArrayOpExpr(n) => NodeEnum::ScalarArrayOpExpr(Box::new((*n).clone())),
            NodeRef::BoolExpr(n) => NodeEnum::BoolExpr(Box::new((*n).clone())),
            NodeRef::SubLink(n) => NodeEnum::SubLink(Box::new((*n).clone())),
            NodeRef::SubPlan(n) => NodeEnum::SubPlan(Box::new((*n).clone())),
            NodeRef::AlternativeSubPlan(n) => NodeEnum::AlternativeSubPlan(Box::new((*n).clone())),
            NodeRef::FieldSelect(n) => NodeEnum::FieldSelect(Box::new((*n).clone())),
            NodeRef::FieldStore(n) => NodeEnum::FieldStore(Box::new((*n).clone())),
            NodeRef::RelabelType(n) => NodeEnum::RelabelType(Box::new((*n).clone())),
            NodeRef::CoerceViaIo(n) => NodeEnum::CoerceViaIo(Box::new((*n).clone())),
            NodeRef::ArrayCoerceExpr(n) => NodeEnum::ArrayCoerceExpr(Box::new((*n).clone())),
            NodeRef::ConvertRowtypeExpr(n) => NodeEnum::ConvertRowtypeExpr(Box::new((*n).clone())),
            NodeRef::CollateExpr(n) => NodeEnum::CollateExpr(Box::new((*n).clone())),
            NodeRef::CaseExpr(n) => NodeEnum::CaseExpr(Box::new((*n).clone())),
            NodeRef::CaseWhen(n) => NodeEnum::CaseWhen(Box::new((*n).clone())),
            NodeRef::CaseTestExpr(n) => NodeEnum::CaseTestExpr(Box::new((*n).clone())),
            NodeRef::ArrayExpr(n) => NodeEnum::ArrayExpr(Box::new((*n).clone())),
            NodeRef::RowExpr(n) => NodeEnum::RowExpr(Box::new((*n).clone())),
            NodeRef::RowCompareExpr(n) => NodeEnum::RowCompareExpr(Box::new((*n).clone())),
            NodeRef::CoalesceExpr(n) => NodeEnum::CoalesceExpr(Box::new((*n).clone())),
            NodeRef::MinMaxExpr(n) => NodeEnum::MinMaxExpr(Box::new((*n).clone())),
            NodeRef::SqlvalueFunction(n) => NodeEnum::SqlvalueFunction(Box::new((*n).clone())),
            NodeRef::XmlExpr(n) => NodeEnum::XmlExpr(Box::new((*n).clone())),
            NodeRef::NullTest(n) => NodeEnum::NullTest(Box::new((*n).clone())),
            NodeRef::BooleanTest(n) => NodeEnum::BooleanTest(Box::new((*n).clone())),
            NodeRef::CoerceToDomain(n) => NodeEnum::CoerceToDomain(Box::new((*n).clone())),
            NodeRef::CoerceToDomainValue(n) => NodeEnum::CoerceToDomainValue(Box::new((*n).clone())),
            NodeRef::SetToDefault(n) => NodeEnum::SetToDefault(Box::new((*n).clone())),
            NodeRef::CurrentOfExpr(n) => NodeEnum::CurrentOfExpr(Box::new((*n).clone())),
            NodeRef::NextValueExpr(n) => NodeEnum::NextValueExpr(Box::new((*n).clone())),
            NodeRef::InferenceElem(n) => NodeEnum::InferenceElem(Box::new((*n).clone())),
            NodeRef::TargetEntry(n) => NodeEnum::TargetEntry(Box::new((*n).clone())),
            NodeRef::RangeTblRef(n) => NodeEnum::RangeTblRef((*n).clone()),
            NodeRef::JoinExpr(n) => NodeEnum::JoinExpr(Box::new((*n).clone())),
            NodeRef::FromExpr(n) => NodeEnum::FromExpr(Box::new((*n).clone())),
            NodeRef::OnConflictExpr(n) => NodeEnum::OnConflictExpr(Box::new((*n).clone())),
            NodeRef::IntoClause(n) => NodeEnum::IntoClause(Box::new((*n).clone())),
            NodeRef::RawStmt(n) => NodeEnum::RawStmt(Box::new((*n).clone())),
            NodeRef::Query(n) => NodeEnum::Query(Box::new((*n).clone())),
            NodeRef::InsertStmt(n) => NodeEnum::InsertStmt(Box::new((*n).clone())),
            NodeRef::DeleteStmt(n) => NodeEnum::DeleteStmt(Box::new((*n).clone())),
            NodeRef::UpdateStmt(n) => NodeEnum::UpdateStmt(Box::new((*n).clone())),
            NodeRef::SelectStmt(n) => NodeEnum::SelectStmt(Box::new((*n).clone())),
            NodeRef::AlterTableStmt(n) => NodeEnum::AlterTableStmt((*n).clone()),
            NodeRef::AlterTableCmd(n) => NodeEnum::AlterTableCmd(Box::new((*n).clone())),
            NodeRef::AlterDomainStmt(n) => NodeEnum::AlterDomainStmt(Box::new((*n).clone())),
            NodeRef::SetOperationStmt(n) => NodeEnum::SetOperationStmt(Box::new((*n).clone())),
            NodeRef::GrantStmt(n) => NodeEnum::GrantStmt((*n).clone()),
            NodeRef::GrantRoleStmt(n) => NodeEnum::GrantRoleStmt((*n).clone()),
            NodeRef::AlterDefaultPrivilegesStmt(n) => NodeEnum::AlterDefaultPrivilegesStmt((*n).clone()),
            NodeRef::ClosePortalStmt(n) => NodeEnum::ClosePortalStmt((*n).clone()),
            NodeRef::ClusterStmt(n) => NodeEnum::ClusterStmt((*n).clone()),
            NodeRef::CopyStmt(n) => NodeEnum::CopyStmt(Box::new((*n).clone())),
            NodeRef::CreateStmt(n) => NodeEnum::CreateStmt((*n).clone()),
            NodeRef::DefineStmt(n) => NodeEnum::DefineStmt((*n).clone()),
            NodeRef::DropStmt(n) => NodeEnum::DropStmt((*n).clone()),
            NodeRef::TruncateStmt(n) => NodeEnum::TruncateStmt((*n).clone()),
            NodeRef::CommentStmt(n) => NodeEnum::CommentStmt(Box::new((*n).clone())),
            NodeRef::FetchStmt(n) => NodeEnum::FetchStmt((*n).clone()),
            NodeRef::IndexStmt(n) => NodeEnum::IndexStmt(Box::new((*n).clone())),
            NodeRef::CreateFunctionStmt(n) => NodeEnum::CreateFunctionStmt((*n).clone()),
            NodeRef::AlterFunctionStmt(n) => NodeEnum::AlterFunctionStmt((*n).clone()),
            NodeRef::DoStmt(n) => NodeEnum::DoStmt((*n).clone()),
            NodeRef::RenameStmt(n) => NodeEnum::RenameStmt(Box::new((*n).clone())),
            NodeRef::RuleStmt(n) => NodeEnum::RuleStmt(Box::new((*n).clone())),
            NodeRef::NotifyStmt(n) => NodeEnum::NotifyStmt((*n).clone()),
            NodeRef::ListenStmt(n) => NodeEnum::ListenStmt((*n).clone()),
            NodeRef::UnlistenStmt(n) => NodeEnum::UnlistenStmt((*n).clone()),
            NodeRef::TransactionStmt(n) => NodeEnum::TransactionStmt((*n).clone()),
            NodeRef::ViewStmt(n) => NodeEnum::ViewStmt(Box::new((*n).clone())),
            NodeRef::LoadStmt(n) => NodeEnum::LoadStmt((*n).clone()),
            NodeRef::CreateDomainStmt(n) => NodeEnum::CreateDomainStmt(Box::new((*n).clone())),
            NodeRef::CreatedbStmt(n) => NodeEnum::CreatedbStmt((*n).clone()),
            NodeRef::DropdbStmt(n) => NodeEnum::DropdbStmt((*n).clone()),
            NodeRef::VacuumStmt(n) => NodeEnum::VacuumStmt((*n).clone()),
            NodeRef::ExplainStmt(n) => NodeEnum::ExplainStmt(Box::new((*n).clone())),
            NodeRef::CreateTableAsStmt(n) => NodeEnum::CreateTableAsStmt(Box::new((*n).clone())),
            NodeRef::CreateSeqStmt(n) => NodeEnum::CreateSeqStmt((*n).clone()),
            NodeRef::AlterSeqStmt(n) => NodeEnum::AlterSeqStmt((*n).clone()),
            NodeRef::VariableSetStmt(n) => NodeEnum::VariableSetStmt((*n).clone()),
            NodeRef::VariableShowStmt(n) => NodeEnum::VariableShowStmt((*n).clone()),
            NodeRef::DiscardStmt(n) => NodeEnum::DiscardStmt((*n).clone()),
            NodeRef::CreateTrigStmt(n) => NodeEnum::CreateTrigStmt(Box::new((*n).clone())),
            NodeRef::CreatePlangStmt(n) => NodeEnum::CreatePlangStmt((*n).clone()),
            NodeRef::CreateRoleStmt(n) => NodeEnum::CreateRoleStmt((*n).clone()),
            NodeRef::AlterRoleStmt(n) => NodeEnum::AlterRoleStmt((*n).clone()),
            NodeRef::DropRoleStmt(n) => NodeEnum::DropRoleStmt((*n).clone()),
            NodeRef::LockStmt(n) => NodeEnum::LockStmt((*n).clone()),
            NodeRef::ConstraintsSetStmt(n) => NodeEnum::ConstraintsSetStmt((*n).clone()),
            NodeRef::ReindexStmt(n) => NodeEnum::ReindexStmt((*n).clone()),
            NodeRef::CheckPointStmt(n) => NodeEnum::CheckPointStmt((*n).clone()),
            NodeRef::CreateSchemaStmt(n) => NodeEnum::CreateSchemaStmt((*n).clone()),
            NodeRef::AlterDatabaseStmt(n) => NodeEnum::AlterDatabaseStmt((*n).clone()),
            NodeRef::AlterDatabaseSetStmt(n) => NodeEnum::AlterDatabaseSetStmt((*n).clone()),
            NodeRef::AlterRoleSetStmt(n) => NodeEnum::AlterRoleSetStmt((*n).clone()),
            NodeRef::CreateConversionStmt(n) => NodeEnum::CreateConversionStmt((*n).clone()),
            NodeRef::CreateCastStmt(n) => NodeEnum::CreateCastStmt((*n).clone()),
            NodeRef::CreateOpClassStmt(n) => NodeEnum::CreateOpClassStmt((*n).clone()),
            NodeRef::CreateOpFamilyStmt(n) => NodeEnum::CreateOpFamilyStmt((*n).clone()),
            NodeRef::AlterOpFamilyStmt(n) => NodeEnum::AlterOpFamilyStmt((*n).clone()),
            NodeRef::PrepareStmt(n) => NodeEnum::PrepareStmt(Box::new((*n).clone())),
            NodeRef::ExecuteStmt(n) => NodeEnum::ExecuteStmt((*n).clone()),
            NodeRef::DeallocateStmt(n) => NodeEnum::DeallocateStmt((*n).clone()),
            NodeRef::DeclareCursorStmt(n) => NodeEnum::DeclareCursorStmt(Box::new((*n).clone())),
            NodeRef::CreateTableSpaceStmt(n) => NodeEnum::CreateTableSpaceStmt((*n).clone()),
            NodeRef::DropTableSpaceStmt(n) => NodeEnum::DropTableSpaceStmt((*n).clone()),
            NodeRef::AlterObjectDependsStmt(n) => NodeEnum::AlterObjectDependsStmt(Box::new((*n).clone())),
            NodeRef::AlterObjectSchemaStmt(n) => NodeEnum::AlterObjectSchemaStmt(Box::new((*n).clone())),
            NodeRef::AlterOwnerStmt(n) => NodeEnum::AlterOwnerStmt(Box::new((*n).clone())),
            NodeRef::AlterOperatorStmt(n) => NodeEnum::AlterOperatorStmt((*n).clone()),
            NodeRef::AlterTypeStmt(n) => NodeEnum::AlterTypeStmt((*n).clone()),
            NodeRef::DropOwnedStmt(n) => NodeEnum::DropOwnedStmt((*n).clone()),
            NodeRef::ReassignOwnedStmt(n) => NodeEnum::ReassignOwnedStmt((*n).clone()),
            NodeRef::CompositeTypeStmt(n) => NodeEnum::CompositeTypeStmt((*n).clone()),
            NodeRef::CreateEnumStmt(n) => NodeEnum::CreateEnumStmt((*n).clone()),
            NodeRef::CreateRangeStmt(n) => NodeEnum::CreateRangeStmt((*n).clone()),
            NodeRef::AlterEnumStmt(n) => NodeEnum::AlterEnumStmt((*n).clone()),
            NodeRef::AlterTsdictionaryStmt(n) => NodeEnum::AlterTsdictionaryStmt((*n).clone()),
            NodeRef::AlterTsconfigurationStmt(n) => NodeEnum::AlterTsconfigurationStmt((*n).clone()),
            NodeRef::CreateFdwStmt(n) => NodeEnum::CreateFdwStmt((*n).clone()),
            NodeRef::AlterFdwStmt(n) => NodeEnum::AlterFdwStmt((*n).clone()),
            NodeRef::CreateForeignServerStmt(n) => NodeEnum::CreateForeignServerStmt((*n).clone()),
            NodeRef::AlterForeignServerStmt(n) => NodeEnum::AlterForeignServerStmt((*n).clone()),
            NodeRef::CreateUserMappingStmt(n) => NodeEnum::CreateUserMappingStmt((*n).clone()),
            NodeRef::AlterUserMappingStmt(n) => NodeEnum::AlterUserMappingStmt((*n).clone()),
            NodeRef::DropUserMappingStmt(n) => NodeEnum::DropUserMappingStmt((*n).clone()),
            NodeRef::AlterTableSpaceOptionsStmt(n) => NodeEnum::AlterTableSpaceOptionsStmt((*n).clone()),
            NodeRef::AlterTableMoveAllStmt(n) => NodeEnum::AlterTableMoveAllStmt((*n).clone()),
            NodeRef::SecLabelStmt(n) => NodeEnum::SecLabelStmt(Box::new((*n).clone())),
            NodeRef::CreateForeignTableStmt(n) => NodeEnum::CreateForeignTableStmt((*n).clone()),
            NodeRef::ImportForeignSchemaStmt(n) => NodeEnum::ImportForeignSchemaStmt((*n).clone()),
            NodeRef::CreateExtensionStmt(n) => NodeEnum::CreateExtensionStmt((*n).clone()),
            NodeRef::AlterExtensionStmt(n) => NodeEnum::AlterExtensionStmt((*n).clone()),
            NodeRef::AlterExtensionContentsStmt(n) => NodeEnum::AlterExtensionContentsStmt(Box::new((*n).clone())),
            NodeRef::CreateEventTrigStmt(n) => NodeEnum::CreateEventTrigStmt((*n).clone()),
            NodeRef::AlterEventTrigStmt(n) => NodeEnum::AlterEventTrigStmt((*n).clone()),
            NodeRef::RefreshMatViewStmt(n) => NodeEnum::RefreshMatViewStmt((*n).clone()),
            NodeRef::ReplicaIdentityStmt(n) => NodeEnum::ReplicaIdentityStmt((*n).clone()),
            NodeRef::AlterSystemStmt(n) => NodeEnum::AlterSystemStmt((*n).clone()),
            NodeRef::CreatePolicyStmt(n) => NodeEnum::CreatePolicyStmt(Box::new((*n).clone())),
            NodeRef::AlterPolicyStmt(n) => NodeEnum::AlterPolicyStmt(Box::new((*n).clone())),
            NodeRef::CreateTransformStmt(n) => NodeEnum::CreateTransformStmt((*n).clone()),
            NodeRef::CreateAmStmt(n) => NodeEnum::CreateAmStmt((*n).clone()),
            NodeRef::CreatePublicationStmt(n) => NodeEnum::CreatePublicationStmt((*n).clone()),
            NodeRef::AlterPublicationStmt(n) => NodeEnum::AlterPublicationStmt((*n).clone()),
            NodeRef::CreateSubscriptionStmt(n) => NodeEnum::CreateSubscriptionStmt((*n).clone()),
            NodeRef::AlterSubscriptionStmt(n) => NodeEnum::AlterSubscriptionStmt((*n).clone()),
            NodeRef::DropSubscriptionStmt(n) => NodeEnum::DropSubscriptionStmt((*n).clone()),
            NodeRef::CreateStatsStmt(n) => NodeEnum::CreateStatsStmt((*n).clone()),
            NodeRef::AlterCollationStmt(n) => NodeEnum::AlterCollationStmt((*n).clone()),
            NodeRef::CallStmt(n) => NodeEnum::CallStmt(Box::new((*n).clone())),
            NodeRef::AlterStatsStmt(n) => NodeEnum::AlterStatsStmt((*n).clone()),
            NodeRef::AExpr(n) => NodeEnum::AExpr(Box::new((*n).clone())),
            NodeRef::ColumnRef(n) => NodeEnum::ColumnRef((*n).clone()),
            NodeRef::ParamRef(n) => NodeEnum::ParamRef((*n).clone()),
            NodeRef::AConst(n) => NodeEnum::AConst(Box::new((*n).clone())),
            NodeRef::FuncCall(n) => NodeEnum::FuncCall(Box::new((*n).clone())),
            NodeRef::AStar(n) => NodeEnum::AStar((*n).clone()),
            NodeRef::AIndices(n) => NodeEnum::AIndices(Box::new((*n).clone())),
            NodeRef::AIndirection(n) => NodeEnum::AIndirection(Box::new((*n).clone())),
            NodeRef::AArrayExpr(n) => NodeEnum::AArrayExpr((*n).clone()),
            NodeRef::ResTarget(n) => NodeEnum::ResTarget(Box::new((*n).clone())),
            NodeRef::MultiAssignRef(n) => NodeEnum::MultiAssignRef(Box::new((*n).clone())),
            NodeRef::TypeCast(n) => NodeEnum::TypeCast(Box::new((*n).clone())),
            NodeRef::CollateClause(n) => NodeEnum::CollateClause(Box::new((*n).clone())),
            NodeRef::SortBy(n) => NodeEnum::SortBy(Box::new((*n).clone())),
            NodeRef::WindowDef(n) => NodeEnum::WindowDef(Box::new((*n).clone())),
            NodeRef::RangeSubselect(n) => NodeEnum::RangeSubselect(Box::new((*n).clone())),
            NodeRef::RangeFunction(n) => NodeEnum::RangeFunction((*n).clone()),
            NodeRef::RangeTableSample(n) => NodeEnum::RangeTableSample(Box::new((*n).clone())),
            NodeRef::RangeTableFunc(n) => NodeEnum::RangeTableFunc(Box::new((*n).clone())),
            NodeRef::RangeTableFuncCol(n) => NodeEnum::RangeTableFuncCol(Box::new((*n).clone())),
            NodeRef::TypeName(n) => NodeEnum::TypeName((*n).clone()),
            NodeRef::ColumnDef(n) => NodeEnum::ColumnDef(Box::new((*n).clone())),
            NodeRef::IndexElem(n) => NodeEnum::IndexElem(Box::new((*n).clone())),
            NodeRef::Constraint(n) => NodeEnum::Constraint(Box::new((*n).clone())),
            NodeRef::DefElem(n) => NodeEnum::DefElem(Box::new((*n).clone())),
            NodeRef::RangeTblEntry(n) => NodeEnum::RangeTblEntry(Box::new((*n).clone())),
            NodeRef::RangeTblFunction(n) => NodeEnum::RangeTblFunction(Box::new((*n).clone())),
            NodeRef::TableSampleClause(n) => NodeEnum::TableSampleClause(Box::new((*n).clone())),
            NodeRef::WithCheckOption(n) => NodeEnum::WithCheckOption(Box::new((*n).clone())),
            NodeRef::SortGroupClause(n) => NodeEnum::SortGroupClause((*n).clone()),
            NodeRef::GroupingSet(n) => NodeEnum::GroupingSet((*n).clone()),
            NodeRef::WindowClause(n) => NodeEnum::WindowClause(Box::new((*n).clone())),
            NodeRef::ObjectWithArgs(n) => NodeEnum::ObjectWithArgs((*n).clone()),
            NodeRef::AccessPriv(n) => NodeEnum::AccessPriv((*n).clone()),
            NodeRef::CreateOpClassItem(n) => NodeEnum::CreateOpClassItem((*n).clone()),
            NodeRef::TableLikeClause(n) => NodeEnum::TableLikeClause((*n).clone()),
            NodeRef::FunctionParameter(n) => NodeEnum::FunctionParameter(Box::new((*n).clone())),
            NodeRef::LockingClause(n) => NodeEnum::LockingClause((*n).clone()),
            NodeRef::RowMarkClause(n) => NodeEnum::RowMarkClause((*n).clone()),
            NodeRef::XmlSerialize(n) => NodeEnum::XmlSerialize(Box::new((*n).clone())),
            NodeRef::WithClause(n) => NodeEnum::WithClause((*n).clone()),
            NodeRef::InferClause(n) => NodeEnum::InferClause(Box::new((*n).clone())),
            NodeRef::OnConflictClause(n) => NodeEnum::OnConflictClause(Box::new((*n).clone())),
            NodeRef::CommonTableExpr(n) => NodeEnum::CommonTableExpr(Box::new((*n).clone())),
            NodeRef::RoleSpec(n) => NodeEnum::RoleSpec((*n).clone()),
            NodeRef::TriggerTransition(n) => NodeEnum::TriggerTransition((*n).clone()),
            NodeRef::PartitionElem(n) => NodeEnum::PartitionElem(Box::new((*n).clone())),
            NodeRef::PartitionSpec(n) => NodeEnum::PartitionSpec((*n).clone()),
            NodeRef::PartitionBoundSpec(n) => NodeEnum::PartitionBoundSpec((*n).clone()),
            NodeRef::PartitionRangeDatum(n) => NodeEnum::PartitionRangeDatum(Box::new((*n).clone())),
            NodeRef::PartitionCmd(n) => NodeEnum::PartitionCmd((*n).clone()),
            NodeRef::VacuumRelation(n) => NodeEnum::VacuumRelation((*n).clone()),
            NodeRef::InlineCodeBlock(n) => NodeEnum::InlineCodeBlock((*n).clone()),
            NodeRef::CallContext(n) => NodeEnum::CallContext((*n).clone()),
            NodeRef::Integer(n) => NodeEnum::Integer((*n).clone()),
            NodeRef::Float(n) => NodeEnum::Float((*n).clone()),
            NodeRef::String(n) => NodeEnum::String((*n).clone()),
            NodeRef::BitString(n) => NodeEnum::BitString((*n).clone()),
            NodeRef::Null(n) => NodeEnum::Null((*n).clone()),
            NodeRef::List(n) => NodeEnum::List((*n).clone()),
            NodeRef::IntList(n) => NodeEnum::IntList((*n).clone()),
            NodeRef::OidList(n) => NodeEnum::OidList((*n).clone()),
        }
    }
}