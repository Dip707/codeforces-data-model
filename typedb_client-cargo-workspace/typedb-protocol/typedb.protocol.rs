#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseManager {
}
/// Nested message and enum types in `DatabaseManager`.
pub mod database_manager {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Get {
    }
    /// Nested message and enum types in `Get`.
    pub mod get {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub database: ::core::option::Option<super::super::DatabaseReplicas>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct All {
    }
    /// Nested message and enum types in `All`.
    pub mod all {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, repeated, tag="1")]
            pub databases: ::prost::alloc::vec::Vec<super::super::DatabaseReplicas>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contains {
    }
    /// Nested message and enum types in `Contains`.
    pub mod contains {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(bool, tag="1")]
            pub contains: bool,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Create {
    }
    /// Nested message and enum types in `Create`.
    pub mod create {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseReplicas {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub replicas: ::prost::alloc::vec::Vec<database_replicas::Replica>,
}
/// Nested message and enum types in `DatabaseReplicas`.
pub mod database_replicas {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Replica {
        #[prost(string, tag="1")]
        pub address: ::prost::alloc::string::String,
        #[prost(bool, tag="2")]
        pub primary: bool,
        #[prost(bool, tag="3")]
        pub preferred: bool,
        #[prost(int64, tag="4")]
        pub term: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
}
/// Nested message and enum types in `Database`.
pub mod database {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Schema {
    }
    /// Nested message and enum types in `Schema`.
    pub mod schema {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(string, tag="1")]
            pub schema: ::prost::alloc::string::String,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TypeSchema {
    }
    /// Nested message and enum types in `TypeSchema`.
    pub mod type_schema {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(string, tag="1")]
            pub schema: ::prost::alloc::string::String,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RuleSchema {
    }
    /// Nested message and enum types in `RuleSchema`.
    pub mod rule_schema {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(string, tag="1")]
            pub schema: ::prost::alloc::string::String,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delete {
    }
    /// Nested message and enum types in `Delete`.
    pub mod delete {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerManager {
}
/// Nested message and enum types in `ServerManager`.
pub mod server_manager {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct All {
    }
    /// Nested message and enum types in `All`.
    pub mod all {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, repeated, tag="1")]
            pub servers: ::prost::alloc::vec::Vec<super::super::Server>,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Server {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserManager {
}
/// Nested message and enum types in `UserManager`.
pub mod user_manager {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contains {
    }
    /// Nested message and enum types in `Contains`.
    pub mod contains {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub username: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(bool, tag="1")]
            pub contains: bool,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Create {
    }
    /// Nested message and enum types in `Create`.
    pub mod create {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub username: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub password: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delete {
    }
    /// Nested message and enum types in `Delete`.
    pub mod delete {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub username: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct All {
    }
    /// Nested message and enum types in `All`.
    pub mod all {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, repeated, tag="1")]
            pub users: ::prost::alloc::vec::Vec<super::super::User>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PasswordSet {
    }
    /// Nested message and enum types in `PasswordSet`.
    pub mod password_set {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub username: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub password: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Get {
    }
    /// Nested message and enum types in `Get`.
    pub mod get {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub username: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub user: ::core::option::Option<super::super::User>,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub username: ::prost::alloc::string::String,
    #[prost(int64, optional, tag="2")]
    pub password_expiry_seconds: ::core::option::Option<i64>,
}
/// Nested message and enum types in `User`.
pub mod user {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PasswordUpdate {
    }
    /// Nested message and enum types in `PasswordUpdate`.
    pub mod password_update {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub username: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub password_old: ::prost::alloc::string::String,
            #[prost(string, tag="3")]
            pub password_new: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Token {
    }
    /// Nested message and enum types in `Token`.
    pub mod token {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub username: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(string, tag="1")]
            pub token: ::prost::alloc::string::String,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Options {
    #[prost(bool, optional, tag="1")]
    pub infer: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub trace_inference: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3")]
    pub explain: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub parallel: ::core::option::Option<bool>,
    #[prost(int32, optional, tag="5")]
    pub prefetch_size: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="6")]
    pub prefetch: ::core::option::Option<bool>,
    #[prost(int32, optional, tag="7")]
    pub session_idle_timeout_millis: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub transaction_timeout_millis: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub schema_lock_acquire_timeout_millis: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="10")]
    pub read_any_replica: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
}
/// Nested message and enum types in `Session`.
pub mod session {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Open {
    }
    /// Nested message and enum types in `Open`.
    pub mod open {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub database: ::prost::alloc::string::String,
            #[prost(enumeration="super::Type", tag="2")]
            pub r#type: i32,
            #[prost(message, optional, tag="3")]
            pub options: ::core::option::Option<super::super::Options>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(bytes="vec", tag="1")]
            pub session_id: ::prost::alloc::vec::Vec<u8>,
            #[prost(int32, tag="2")]
            pub server_duration_millis: i32,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Close {
    }
    /// Nested message and enum types in `Close`.
    pub mod close {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(bytes="vec", tag="1")]
            pub session_id: ::prost::alloc::vec::Vec<u8>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Pulse {
    }
    /// Nested message and enum types in `Pulse`.
    pub mod pulse {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(bytes="vec", tag="1")]
            pub session_id: ::prost::alloc::vec::Vec<u8>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(bool, tag="1")]
            pub alive: bool,
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Data = 0,
        Schema = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Data => "DATA",
                Type::Schema => "SCHEMA",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConceptManager {
}
/// Nested message and enum types in `ConceptManager`.
pub mod concept_manager {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Req {
        #[prost(oneof="req::Req", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
        pub req: ::core::option::Option<req::Req>,
    }
    /// Nested message and enum types in `Req`.
    pub mod req {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Req {
            #[prost(message, tag="1")]
            GetEntityTypeReq(super::get_entity_type::Req),
            #[prost(message, tag="2")]
            GetRelationTypeReq(super::get_relation_type::Req),
            #[prost(message, tag="3")]
            GetAttributeTypeReq(super::get_attribute_type::Req),
            #[prost(message, tag="4")]
            PutEntityTypeReq(super::put_entity_type::Req),
            #[prost(message, tag="5")]
            PutRelationTypeReq(super::put_relation_type::Req),
            #[prost(message, tag="6")]
            PutAttributeTypeReq(super::put_attribute_type::Req),
            #[prost(message, tag="7")]
            GetEntityReq(super::get_entity::Req),
            #[prost(message, tag="8")]
            GetRelationReq(super::get_relation::Req),
            #[prost(message, tag="9")]
            GetAttributeReq(super::get_attribute::Req),
            #[prost(message, tag="10")]
            GetSchemaExceptionsReq(super::get_schema_exceptions::Req),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Res {
        #[prost(oneof="res::Res", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
        pub res: ::core::option::Option<res::Res>,
    }
    /// Nested message and enum types in `Res`.
    pub mod res {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="1")]
            GetEntityTypeRes(super::get_entity_type::Res),
            #[prost(message, tag="2")]
            GetRelationTypeRes(super::get_relation_type::Res),
            #[prost(message, tag="3")]
            GetAttributeTypeRes(super::get_attribute_type::Res),
            #[prost(message, tag="4")]
            PutEntityTypeRes(super::put_entity_type::Res),
            #[prost(message, tag="5")]
            PutRelationTypeRes(super::put_relation_type::Res),
            #[prost(message, tag="6")]
            PutAttributeTypeRes(super::put_attribute_type::Res),
            #[prost(message, tag="7")]
            GetEntityRes(super::get_entity::Res),
            #[prost(message, tag="8")]
            GetRelationRes(super::get_relation::Res),
            #[prost(message, tag="9")]
            GetAttributeRes(super::get_attribute::Res),
            #[prost(message, tag="10")]
            GetSchemaExceptionsRes(super::get_schema_exceptions::Res),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetEntityType {
    }
    /// Nested message and enum types in `GetEntityType`.
    pub mod get_entity_type {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub entity_type: ::core::option::Option<super::super::EntityType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRelationType {
    }
    /// Nested message and enum types in `GetRelationType`.
    pub mod get_relation_type {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub relation_type: ::core::option::Option<super::super::RelationType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetAttributeType {
    }
    /// Nested message and enum types in `GetAttributeType`.
    pub mod get_attribute_type {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub attribute_type: ::core::option::Option<super::super::AttributeType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PutEntityType {
    }
    /// Nested message and enum types in `PutEntityType`.
    pub mod put_entity_type {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub entity_type: ::core::option::Option<super::super::EntityType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PutRelationType {
    }
    /// Nested message and enum types in `PutRelationType`.
    pub mod put_relation_type {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub relation_type: ::core::option::Option<super::super::RelationType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PutAttributeType {
    }
    /// Nested message and enum types in `PutAttributeType`.
    pub mod put_attribute_type {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
            #[prost(enumeration="super::super::attribute_type::ValueType", tag="2")]
            pub value_type: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub attribute_type: ::core::option::Option<super::super::AttributeType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetEntity {
    }
    /// Nested message and enum types in `GetEntity`.
    pub mod get_entity {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(bytes="vec", tag="1")]
            pub iid: ::prost::alloc::vec::Vec<u8>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub entity: ::core::option::Option<super::super::Entity>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRelation {
    }
    /// Nested message and enum types in `GetRelation`.
    pub mod get_relation {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(bytes="vec", tag="1")]
            pub iid: ::prost::alloc::vec::Vec<u8>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub relation: ::core::option::Option<super::super::Relation>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetAttribute {
    }
    /// Nested message and enum types in `GetAttribute`.
    pub mod get_attribute {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(bytes="vec", tag="1")]
            pub iid: ::prost::alloc::vec::Vec<u8>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub attribute: ::core::option::Option<super::super::Attribute>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSchemaExceptions {
    }
    /// Nested message and enum types in `GetSchemaExceptions`.
    pub mod get_schema_exceptions {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, repeated, tag="1")]
            pub exceptions: ::prost::alloc::vec::Vec<super::super::Exception>,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exception {
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Concept {
    #[prost(oneof="concept::Concept", tags="1, 2, 3, 4, 5, 6, 7, 10")]
    pub concept: ::core::option::Option<concept::Concept>,
}
/// Nested message and enum types in `Concept`.
pub mod concept {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Concept {
        #[prost(message, tag="1")]
        EntityType(super::EntityType),
        #[prost(message, tag="2")]
        RelationType(super::RelationType),
        #[prost(message, tag="3")]
        AttributeType(super::AttributeType),
        #[prost(message, tag="4")]
        RoleType(super::RoleType),
        #[prost(message, tag="5")]
        Entity(super::Entity),
        #[prost(message, tag="6")]
        Relation(super::Relation),
        #[prost(message, tag="7")]
        Attribute(super::Attribute),
        #[prost(message, tag="10")]
        ThingTypeRoot(super::thing_type::Root),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Thing {
    #[prost(oneof="thing::Thing", tags="1, 2, 3")]
    pub thing: ::core::option::Option<thing::Thing>,
}
/// Nested message and enum types in `Thing`.
pub mod thing {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Req {
        #[prost(bytes="vec", tag="1")]
        pub iid: ::prost::alloc::vec::Vec<u8>,
        #[prost(oneof="req::Req", tags="100, 101, 102, 103, 104, 105, 200, 201, 202, 203, 204, 300")]
        pub req: ::core::option::Option<req::Req>,
    }
    /// Nested message and enum types in `Req`.
    pub mod req {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Req {
            #[prost(message, tag="100")]
            ThingDeleteReq(super::delete::Req),
            #[prost(message, tag="101")]
            ThingGetHasReq(super::get_has::Req),
            #[prost(message, tag="102")]
            ThingSetHasReq(super::set_has::Req),
            #[prost(message, tag="103")]
            ThingUnsetHasReq(super::unset_has::Req),
            #[prost(message, tag="104")]
            ThingGetRelationsReq(super::get_relations::Req),
            #[prost(message, tag="105")]
            ThingGetPlayingReq(super::get_playing::Req),
            #[prost(message, tag="200")]
            RelationAddRolePlayerReq(super::super::relation::add_role_player::Req),
            #[prost(message, tag="201")]
            RelationRemoveRolePlayerReq(super::super::relation::remove_role_player::Req),
            #[prost(message, tag="202")]
            RelationGetPlayersByRoleTypeReq(super::super::relation::get_players_by_role_type::Req),
            #[prost(message, tag="203")]
            RelationGetRolePlayersReq(super::super::relation::get_role_players::Req),
            #[prost(message, tag="204")]
            RelationGetRelatingReq(super::super::relation::get_relating::Req),
            #[prost(message, tag="300")]
            AttributeGetOwnersReq(super::super::attribute::get_owners::Req),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Res {
        #[prost(oneof="res::Res", tags="100, 101, 102, 200, 201")]
        pub res: ::core::option::Option<res::Res>,
    }
    /// Nested message and enum types in `Res`.
    pub mod res {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="100")]
            ThingDeleteRes(super::delete::Res),
            #[prost(message, tag="101")]
            ThingSetHasRes(super::set_has::Res),
            #[prost(message, tag="102")]
            ThingUnsetHasRes(super::unset_has::Res),
            #[prost(message, tag="200")]
            RelationAddRolePlayerRes(super::super::relation::add_role_player::Res),
            #[prost(message, tag="201")]
            RelationRemoveRolePlayerRes(super::super::relation::remove_role_player::Res),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResPart {
        #[prost(oneof="res_part::Res", tags="100, 101, 102, 200, 201, 202, 300")]
        pub res: ::core::option::Option<res_part::Res>,
    }
    /// Nested message and enum types in `ResPart`.
    pub mod res_part {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="100")]
            ThingGetHasResPart(super::get_has::ResPart),
            #[prost(message, tag="101")]
            ThingGetRelationsResPart(super::get_relations::ResPart),
            #[prost(message, tag="102")]
            ThingGetPlayingResPart(super::get_playing::ResPart),
            #[prost(message, tag="200")]
            RelationGetPlayersByRoleTypeResPart(super::super::relation::get_players_by_role_type::ResPart),
            #[prost(message, tag="201")]
            RelationGetRolePlayersResPart(super::super::relation::get_role_players::ResPart),
            #[prost(message, tag="202")]
            RelationGetRelatingResPart(super::super::relation::get_relating::ResPart),
            #[prost(message, tag="300")]
            AttributeGetOwnersResPart(super::super::attribute::get_owners::ResPart),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delete {
    }
    /// Nested message and enum types in `Delete`.
    pub mod delete {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetHas {
    }
    /// Nested message and enum types in `GetHas`.
    pub mod get_has {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, repeated, tag="1")]
            pub attribute_types: ::prost::alloc::vec::Vec<super::super::AttributeType>,
            #[prost(message, repeated, tag="2")]
            pub annotations: ::prost::alloc::vec::Vec<super::super::r#type::Annotation>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub attributes: ::prost::alloc::vec::Vec<super::super::Attribute>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetHas {
    }
    /// Nested message and enum types in `SetHas`.
    pub mod set_has {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub attribute: ::core::option::Option<super::super::Attribute>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnsetHas {
    }
    /// Nested message and enum types in `UnsetHas`.
    pub mod unset_has {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub attribute: ::core::option::Option<super::super::Attribute>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRelations {
    }
    /// Nested message and enum types in `GetRelations`.
    pub mod get_relations {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, repeated, tag="1")]
            pub role_types: ::prost::alloc::vec::Vec<super::super::RoleType>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub relations: ::prost::alloc::vec::Vec<super::super::Relation>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetPlaying {
    }
    /// Nested message and enum types in `GetPlaying`.
    pub mod get_playing {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub role_types: ::prost::alloc::vec::Vec<super::super::RoleType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Thing {
        #[prost(message, tag="1")]
        Entity(super::Entity),
        #[prost(message, tag="2")]
        Relation(super::Relation),
        #[prost(message, tag="3")]
        Attribute(super::Attribute),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(bytes="vec", tag="1")]
    pub iid: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub entity_type: ::core::option::Option<EntityType>,
    #[prost(bool, tag="3")]
    pub inferred: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    #[prost(bytes="vec", tag="1")]
    pub iid: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub relation_type: ::core::option::Option<RelationType>,
    #[prost(bool, tag="3")]
    pub inferred: bool,
}
/// Nested message and enum types in `Relation`.
pub mod relation {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddRolePlayer {
    }
    /// Nested message and enum types in `AddRolePlayer`.
    pub mod add_role_player {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub role_player: ::core::option::Option<super::RolePlayer>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RemoveRolePlayer {
    }
    /// Nested message and enum types in `RemoveRolePlayer`.
    pub mod remove_role_player {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub role_player: ::core::option::Option<super::RolePlayer>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetPlayersByRoleType {
    }
    /// Nested message and enum types in `GetPlayersByRoleType`.
    pub mod get_players_by_role_type {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, repeated, tag="1")]
            pub role_types: ::prost::alloc::vec::Vec<super::super::RoleType>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub things: ::prost::alloc::vec::Vec<super::super::Thing>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRolePlayers {
    }
    /// Nested message and enum types in `GetRolePlayers`.
    pub mod get_role_players {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub role_players: ::prost::alloc::vec::Vec<super::RolePlayer>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRelating {
    }
    /// Nested message and enum types in `GetRelating`.
    pub mod get_relating {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub role_types: ::prost::alloc::vec::Vec<super::super::RoleType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RolePlayer {
        #[prost(message, optional, tag="1")]
        pub role_type: ::core::option::Option<super::RoleType>,
        #[prost(message, optional, tag="2")]
        pub player: ::core::option::Option<super::Thing>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribute {
    #[prost(bytes="vec", tag="1")]
    pub iid: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub attribute_type: ::core::option::Option<AttributeType>,
    #[prost(message, optional, tag="3")]
    pub value: ::core::option::Option<attribute::Value>,
    #[prost(bool, tag="4")]
    pub inferred: bool,
}
/// Nested message and enum types in `Attribute`.
pub mod attribute {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Value {
        #[prost(oneof="value::Value", tags="1, 2, 3, 4, 5")]
        pub value: ::core::option::Option<value::Value>,
    }
    /// Nested message and enum types in `Value`.
    pub mod value {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            #[prost(string, tag="1")]
            String(::prost::alloc::string::String),
            #[prost(bool, tag="2")]
            Boolean(bool),
            #[prost(sint64, tag="3")]
            Long(i64),
            #[prost(double, tag="4")]
            Double(f64),
            ///  time since epoch in milliseconds
            #[prost(sint64, tag="5")]
            DateTime(i64),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetOwners {
    }
    /// Nested message and enum types in `GetOwners`.
    pub mod get_owners {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub thing_type: ::core::option::Option<super::super::ThingType>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub things: ::prost::alloc::vec::Vec<super::super::Thing>,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Type {
}
/// Nested message and enum types in `Type`.
pub mod r#type {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Req {
        #[prost(oneof="req::Req", tags="1, 2")]
        pub req: ::core::option::Option<req::Req>,
    }
    /// Nested message and enum types in `Req`.
    pub mod req {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Req {
            #[prost(message, tag="1")]
            ThingTypeReq(super::super::thing_type::Req),
            #[prost(message, tag="2")]
            RoleTypeReq(super::super::role_type::Req),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Res {
        #[prost(oneof="res::Res", tags="1, 2")]
        pub res: ::core::option::Option<res::Res>,
    }
    /// Nested message and enum types in `Res`.
    pub mod res {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="1")]
            ThingTypeRes(super::super::thing_type::Res),
            #[prost(message, tag="2")]
            RoleTypeRes(super::super::role_type::Res),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResPart {
        #[prost(oneof="res_part::Res", tags="1, 2")]
        pub res: ::core::option::Option<res_part::Res>,
    }
    /// Nested message and enum types in `ResPart`.
    pub mod res_part {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="1")]
            ThingTypeResPart(super::super::thing_type::ResPart),
            #[prost(message, tag="2")]
            RoleTypeResPart(super::super::role_type::ResPart),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Annotation {
        #[prost(oneof="annotation::Annotation", tags="1, 2")]
        pub annotation: ::core::option::Option<annotation::Annotation>,
    }
    /// Nested message and enum types in `Annotation`.
    pub mod annotation {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Key {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Unique {
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Annotation {
            #[prost(message, tag="1")]
            Key(Key),
            #[prost(message, tag="2")]
            Unique(Unique),
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Transitivity {
        Transitive = 0,
        Explicit = 1,
    }
    impl Transitivity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Transitivity::Transitive => "TRANSITIVE",
                Transitivity::Explicit => "EXPLICIT",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThingType {
    #[prost(oneof="thing_type::Type", tags="1, 2, 3, 10")]
    pub r#type: ::core::option::Option<thing_type::Type>,
}
/// Nested message and enum types in `ThingType`.
pub mod thing_type {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Root {
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Req {
        #[prost(string, tag="1")]
        pub label: ::prost::alloc::string::String,
        #[prost(oneof="req::Req", tags="100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 200, 201, 202, 203, 204, 205, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 400, 401, 402, 403, 404, 405, 406, 407, 408, 409")]
        pub req: ::core::option::Option<req::Req>,
    }
    /// Nested message and enum types in `Req`.
    pub mod req {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Req {
            #[prost(message, tag="100")]
            ThingTypeDeleteReq(super::delete::Req),
            #[prost(message, tag="101")]
            ThingTypeSetLabelReq(super::set_label::Req),
            #[prost(message, tag="102")]
            ThingTypeSetAbstractReq(super::set_abstract::Req),
            #[prost(message, tag="103")]
            ThingTypeUnsetAbstractReq(super::unset_abstract::Req),
            #[prost(message, tag="104")]
            ThingTypeGetOwnsReq(super::get_owns::Req),
            #[prost(message, tag="105")]
            ThingTypeGetOwnsOverriddenReq(super::get_owns_overridden::Req),
            #[prost(message, tag="106")]
            ThingTypeSetOwnsReq(super::set_owns::Req),
            #[prost(message, tag="107")]
            ThingTypeUnsetOwnsReq(super::unset_owns::Req),
            #[prost(message, tag="108")]
            ThingTypeGetPlaysReq(super::get_plays::Req),
            #[prost(message, tag="109")]
            ThingTypeGetPlaysOverriddenReq(super::get_plays_overridden::Req),
            #[prost(message, tag="110")]
            ThingTypeSetPlaysReq(super::set_plays::Req),
            #[prost(message, tag="111")]
            ThingTypeUnsetPlaysReq(super::unset_plays::Req),
            #[prost(message, tag="112")]
            ThingTypeGetSyntaxReq(super::get_syntax::Req),
            #[prost(message, tag="200")]
            EntityTypeCreateReq(super::super::entity_type::create::Req),
            #[prost(message, tag="201")]
            EntityTypeGetSupertypeReq(super::super::entity_type::get_supertype::Req),
            #[prost(message, tag="202")]
            EntityTypeSetSupertypeReq(super::super::entity_type::set_supertype::Req),
            #[prost(message, tag="203")]
            EntityTypeGetSupertypesReq(super::super::entity_type::get_supertypes::Req),
            #[prost(message, tag="204")]
            EntityTypeGetSubtypesReq(super::super::entity_type::get_subtypes::Req),
            #[prost(message, tag="205")]
            EntityTypeGetInstancesReq(super::super::entity_type::get_instances::Req),
            #[prost(message, tag="300")]
            RelationTypeCreateReq(super::super::relation_type::create::Req),
            #[prost(message, tag="301")]
            RelationTypeGetSupertypeReq(super::super::relation_type::get_supertype::Req),
            #[prost(message, tag="302")]
            RelationTypeSetSupertypeReq(super::super::relation_type::set_supertype::Req),
            #[prost(message, tag="303")]
            RelationTypeGetSupertypesReq(super::super::relation_type::get_supertypes::Req),
            #[prost(message, tag="304")]
            RelationTypeGetSubtypesReq(super::super::relation_type::get_subtypes::Req),
            #[prost(message, tag="305")]
            RelationTypeGetInstancesReq(super::super::relation_type::get_instances::Req),
            #[prost(message, tag="306")]
            RelationTypeGetRelatesReq(super::super::relation_type::get_relates::Req),
            #[prost(message, tag="307")]
            RelationTypeGetRelatesForRoleLabelReq(super::super::relation_type::get_relates_for_role_label::Req),
            #[prost(message, tag="308")]
            RelationTypeGetRelatesOverriddenReq(super::super::relation_type::get_relates_overridden::Req),
            #[prost(message, tag="309")]
            RelationTypeSetRelatesReq(super::super::relation_type::set_relates::Req),
            #[prost(message, tag="310")]
            RelationTypeUnsetRelatesReq(super::super::relation_type::unset_relates::Req),
            #[prost(message, tag="400")]
            AttributeTypePutReq(super::super::attribute_type::put::Req),
            #[prost(message, tag="401")]
            AttributeTypeGetReq(super::super::attribute_type::get::Req),
            #[prost(message, tag="402")]
            AttributeTypeGetSupertypeReq(super::super::attribute_type::get_supertype::Req),
            #[prost(message, tag="403")]
            AttributeTypeSetSupertypeReq(super::super::attribute_type::set_supertype::Req),
            #[prost(message, tag="404")]
            AttributeTypeGetSupertypesReq(super::super::attribute_type::get_supertypes::Req),
            #[prost(message, tag="405")]
            AttributeTypeGetSubtypesReq(super::super::attribute_type::get_subtypes::Req),
            #[prost(message, tag="406")]
            AttributeTypeGetInstancesReq(super::super::attribute_type::get_instances::Req),
            #[prost(message, tag="407")]
            AttributeTypeGetRegexReq(super::super::attribute_type::get_regex::Req),
            #[prost(message, tag="408")]
            AttributeTypeSetRegexReq(super::super::attribute_type::set_regex::Req),
            #[prost(message, tag="409")]
            AttributeTypeGetOwnersReq(super::super::attribute_type::get_owners::Req),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Res {
        #[prost(oneof="res::Res", tags="100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 200, 201, 202, 300, 301, 302, 303, 304, 305, 306, 400, 401, 402, 403, 404, 405")]
        pub res: ::core::option::Option<res::Res>,
    }
    /// Nested message and enum types in `Res`.
    pub mod res {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="100")]
            ThingTypeDeleteRes(super::delete::Res),
            #[prost(message, tag="101")]
            ThingTypeSetLabelRes(super::set_label::Res),
            #[prost(message, tag="102")]
            ThingTypeSetAbstractRes(super::set_abstract::Res),
            #[prost(message, tag="103")]
            ThingTypeUnsetAbstractRes(super::unset_abstract::Res),
            #[prost(message, tag="104")]
            ThingTypeGetOwnsOverriddenRes(super::get_owns_overridden::Res),
            #[prost(message, tag="105")]
            ThingTypeSetOwnsRes(super::set_owns::Res),
            #[prost(message, tag="106")]
            ThingTypeUnsetOwnsRes(super::unset_owns::Res),
            #[prost(message, tag="107")]
            ThingTypeGetPlaysOverriddenRes(super::get_plays_overridden::Res),
            #[prost(message, tag="108")]
            ThingTypeSetPlaysRes(super::set_plays::Res),
            #[prost(message, tag="109")]
            ThingTypeUnsetPlaysRes(super::unset_plays::Res),
            #[prost(message, tag="110")]
            ThingTypeGetSyntaxRes(super::get_syntax::Res),
            #[prost(message, tag="200")]
            EntityTypeCreateRes(super::super::entity_type::create::Res),
            #[prost(message, tag="201")]
            EntityTypeGetSupertypeRes(super::super::entity_type::get_supertype::Res),
            #[prost(message, tag="202")]
            EntityTypeSetSupertypeRes(super::super::entity_type::set_supertype::Res),
            #[prost(message, tag="300")]
            RelationTypeCreateRes(super::super::relation_type::create::Res),
            #[prost(message, tag="301")]
            RelationTypeGetSupertypeRes(super::super::relation_type::get_supertype::Res),
            #[prost(message, tag="302")]
            RelationTypeSetSupertypeRes(super::super::relation_type::set_supertype::Res),
            #[prost(message, tag="303")]
            RelationTypeGetRelatesForRoleLabelRes(super::super::relation_type::get_relates_for_role_label::Res),
            #[prost(message, tag="304")]
            RelationTypeGetRelatesOverriddenRes(super::super::relation_type::get_relates_overridden::Res),
            #[prost(message, tag="305")]
            RelationTypeSetRelatesRes(super::super::relation_type::set_relates::Res),
            #[prost(message, tag="306")]
            RelationTypeUnsetRelatesRes(super::super::relation_type::unset_relates::Res),
            #[prost(message, tag="400")]
            AttributeTypePutRes(super::super::attribute_type::put::Res),
            #[prost(message, tag="401")]
            AttributeTypeGetRes(super::super::attribute_type::get::Res),
            #[prost(message, tag="402")]
            AttributeTypeGetSupertypeRes(super::super::attribute_type::get_supertype::Res),
            #[prost(message, tag="403")]
            AttributeTypeSetSupertypeRes(super::super::attribute_type::set_supertype::Res),
            #[prost(message, tag="404")]
            AttributeTypeGetRegexRes(super::super::attribute_type::get_regex::Res),
            #[prost(message, tag="405")]
            AttributeTypeSetRegexRes(super::super::attribute_type::set_regex::Res),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResPart {
        #[prost(oneof="res_part::Res", tags="100, 101, 200, 201, 202, 300, 301, 302, 303, 400, 401, 402, 403")]
        pub res: ::core::option::Option<res_part::Res>,
    }
    /// Nested message and enum types in `ResPart`.
    pub mod res_part {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="100")]
            ThingTypeGetOwnsResPart(super::get_owns::ResPart),
            #[prost(message, tag="101")]
            ThingTypeGetPlaysResPart(super::get_plays::ResPart),
            #[prost(message, tag="200")]
            EntityTypeGetSupertypesResPart(super::super::entity_type::get_supertypes::ResPart),
            #[prost(message, tag="201")]
            EntityTypeGetSubtypesResPart(super::super::entity_type::get_subtypes::ResPart),
            #[prost(message, tag="202")]
            EntityTypeGetInstancesResPart(super::super::entity_type::get_instances::ResPart),
            #[prost(message, tag="300")]
            RelationTypeGetSupertypesResPart(super::super::relation_type::get_supertypes::ResPart),
            #[prost(message, tag="301")]
            RelationTypeGetSubtypesResPart(super::super::relation_type::get_subtypes::ResPart),
            #[prost(message, tag="302")]
            RelationTypeGetInstancesResPart(super::super::relation_type::get_instances::ResPart),
            #[prost(message, tag="303")]
            RelationTypeGetRelatesResPart(super::super::relation_type::get_relates::ResPart),
            #[prost(message, tag="400")]
            AttributeTypeGetSupertypesResPart(super::super::attribute_type::get_supertypes::ResPart),
            #[prost(message, tag="401")]
            AttributeTypeGetSubtypesResPart(super::super::attribute_type::get_subtypes::ResPart),
            #[prost(message, tag="402")]
            AttributeTypeGetInstancesResPart(super::super::attribute_type::get_instances::ResPart),
            #[prost(message, tag="403")]
            AttributeTypeGetOwnersResPart(super::super::attribute_type::get_owners::ResPart),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delete {
    }
    /// Nested message and enum types in `Delete`.
    pub mod delete {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetLabel {
    }
    /// Nested message and enum types in `SetLabel`.
    pub mod set_label {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetAbstract {
    }
    /// Nested message and enum types in `SetAbstract`.
    pub mod set_abstract {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnsetAbstract {
    }
    /// Nested message and enum types in `UnsetAbstract`.
    pub mod unset_abstract {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetOwns {
    }
    /// Nested message and enum types in `GetOwns`.
    pub mod get_owns {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::attribute_type::ValueType", optional, tag="1")]
            pub value_type: ::core::option::Option<i32>,
            #[prost(enumeration="super::super::r#type::Transitivity", tag="2")]
            pub transitivity: i32,
            #[prost(message, repeated, tag="3")]
            pub annotations: ::prost::alloc::vec::Vec<super::super::r#type::Annotation>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub attribute_types: ::prost::alloc::vec::Vec<super::super::AttributeType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetOwnsOverridden {
    }
    /// Nested message and enum types in `GetOwnsOverridden`.
    pub mod get_owns_overridden {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub attribute_type: ::core::option::Option<super::super::AttributeType>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub attribute_type: ::core::option::Option<super::super::AttributeType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetOwns {
    }
    /// Nested message and enum types in `SetOwns`.
    pub mod set_owns {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub attribute_type: ::core::option::Option<super::super::AttributeType>,
            #[prost(message, optional, tag="2")]
            pub overridden_type: ::core::option::Option<super::super::AttributeType>,
            #[prost(message, repeated, tag="3")]
            pub annotations: ::prost::alloc::vec::Vec<super::super::r#type::Annotation>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnsetOwns {
    }
    /// Nested message and enum types in `UnsetOwns`.
    pub mod unset_owns {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub attribute_type: ::core::option::Option<super::super::AttributeType>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetPlays {
    }
    /// Nested message and enum types in `GetPlays`.
    pub mod get_plays {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub role_types: ::prost::alloc::vec::Vec<super::super::RoleType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetPlaysOverridden {
    }
    /// Nested message and enum types in `GetPlaysOverridden`.
    pub mod get_plays_overridden {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub role_type: ::core::option::Option<super::super::RoleType>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub role_type: ::core::option::Option<super::super::RoleType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetPlays {
    }
    /// Nested message and enum types in `SetPlays`.
    pub mod set_plays {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub role_type: ::core::option::Option<super::super::RoleType>,
            #[prost(message, optional, tag="2")]
            pub overridden_role_type: ::core::option::Option<super::super::RoleType>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnsetPlays {
    }
    /// Nested message and enum types in `UnsetPlays`.
    pub mod unset_plays {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub role_type: ::core::option::Option<super::super::RoleType>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSyntax {
    }
    /// Nested message and enum types in `GetSyntax`.
    pub mod get_syntax {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(string, tag="1")]
            pub syntax: ::prost::alloc::string::String,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        EntityType(super::EntityType),
        #[prost(message, tag="2")]
        RelationType(super::RelationType),
        #[prost(message, tag="3")]
        AttributeType(super::AttributeType),
        #[prost(message, tag="10")]
        ThingTypeRoot(Root),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleType {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub scope: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_root: bool,
    #[prost(bool, tag="4")]
    pub is_abstract: bool,
}
/// Nested message and enum types in `RoleType`.
pub mod role_type {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Req {
        #[prost(string, tag="1")]
        pub label: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub scope: ::prost::alloc::string::String,
        #[prost(oneof="req::Req", tags="100, 101, 102, 103, 104, 105, 106, 107, 108")]
        pub req: ::core::option::Option<req::Req>,
    }
    /// Nested message and enum types in `Req`.
    pub mod req {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Req {
            #[prost(message, tag="100")]
            RoleTypeDeleteReq(super::delete::Req),
            #[prost(message, tag="101")]
            RoleTypeSetLabelReq(super::set_label::Req),
            #[prost(message, tag="102")]
            RoleTypeGetSupertypeReq(super::get_supertype::Req),
            #[prost(message, tag="103")]
            RoleTypeGetSupertypesReq(super::get_supertypes::Req),
            #[prost(message, tag="104")]
            RoleTypeGetSubtypesReq(super::get_subtypes::Req),
            #[prost(message, tag="105")]
            RoleTypeGetRelationTypesReq(super::get_relation_types::Req),
            #[prost(message, tag="106")]
            RoleTypeGetPlayerTypesReq(super::get_player_types::Req),
            #[prost(message, tag="107")]
            RoleTypeGetRelationInstancesReq(super::get_relation_instances::Req),
            #[prost(message, tag="108")]
            RoleTypeGetPlayerInstancesReq(super::get_player_instances::Req),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Res {
        #[prost(oneof="res::Res", tags="100, 101, 102")]
        pub res: ::core::option::Option<res::Res>,
    }
    /// Nested message and enum types in `Res`.
    pub mod res {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="100")]
            RoleTypeDeleteRes(super::delete::Res),
            #[prost(message, tag="101")]
            RoleTypeSetLabelRes(super::set_label::Res),
            #[prost(message, tag="102")]
            RoleTypeGetSupertypeRes(super::get_supertype::Res),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResPart {
        #[prost(oneof="res_part::Res", tags="100, 101, 102, 103, 104, 105")]
        pub res: ::core::option::Option<res_part::Res>,
    }
    /// Nested message and enum types in `ResPart`.
    pub mod res_part {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="100")]
            RoleTypeGetSupertypesResPart(super::get_supertypes::ResPart),
            #[prost(message, tag="101")]
            RoleTypeGetSubtypesResPart(super::get_subtypes::ResPart),
            #[prost(message, tag="102")]
            RoleTypeGetRelationTypesResPart(super::get_relation_types::ResPart),
            #[prost(message, tag="103")]
            RoleTypeGetPlayerTypesResPart(super::get_player_types::ResPart),
            #[prost(message, tag="104")]
            RoleTypeGetRelationInstancesResPart(super::get_relation_instances::ResPart),
            #[prost(message, tag="105")]
            RoleTypeGetPlayerInstancesResPart(super::get_player_instances::ResPart),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delete {
    }
    /// Nested message and enum types in `Delete`.
    pub mod delete {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetLabel {
    }
    /// Nested message and enum types in `SetLabel`.
    pub mod set_label {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSupertype {
    }
    /// Nested message and enum types in `GetSupertype`.
    pub mod get_supertype {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub role_type: ::core::option::Option<super::super::RoleType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSupertypes {
    }
    /// Nested message and enum types in `GetSupertypes`.
    pub mod get_supertypes {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub role_types: ::prost::alloc::vec::Vec<super::super::RoleType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSubtypes {
    }
    /// Nested message and enum types in `GetSubtypes`.
    pub mod get_subtypes {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub role_types: ::prost::alloc::vec::Vec<super::super::RoleType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRelationTypes {
    }
    /// Nested message and enum types in `GetRelationTypes`.
    pub mod get_relation_types {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub relation_types: ::prost::alloc::vec::Vec<super::super::RelationType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetPlayerTypes {
    }
    /// Nested message and enum types in `GetPlayerTypes`.
    pub mod get_player_types {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub thing_types: ::prost::alloc::vec::Vec<super::super::ThingType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRelationInstances {
    }
    /// Nested message and enum types in `GetRelationInstances`.
    pub mod get_relation_instances {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub relations: ::prost::alloc::vec::Vec<super::super::Relation>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetPlayerInstances {
    }
    /// Nested message and enum types in `GetPlayerInstances`.
    pub mod get_player_instances {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub things: ::prost::alloc::vec::Vec<super::super::Thing>,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityType {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_root: bool,
    #[prost(bool, tag="3")]
    pub is_abstract: bool,
}
/// Nested message and enum types in `EntityType`.
pub mod entity_type {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Create {
    }
    /// Nested message and enum types in `Create`.
    pub mod create {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub entity: ::core::option::Option<super::super::Entity>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSupertype {
    }
    /// Nested message and enum types in `GetSupertype`.
    pub mod get_supertype {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub entity_type: ::core::option::Option<super::super::EntityType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetSupertype {
    }
    /// Nested message and enum types in `SetSupertype`.
    pub mod set_supertype {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub entity_type: ::core::option::Option<super::super::EntityType>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSupertypes {
    }
    /// Nested message and enum types in `GetSupertypes`.
    pub mod get_supertypes {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub entity_types: ::prost::alloc::vec::Vec<super::super::EntityType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSubtypes {
    }
    /// Nested message and enum types in `GetSubtypes`.
    pub mod get_subtypes {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub entity_types: ::prost::alloc::vec::Vec<super::super::EntityType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetInstances {
    }
    /// Nested message and enum types in `GetInstances`.
    pub mod get_instances {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub entities: ::prost::alloc::vec::Vec<super::super::Entity>,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationType {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_root: bool,
    #[prost(bool, tag="3")]
    pub is_abstract: bool,
}
/// Nested message and enum types in `RelationType`.
pub mod relation_type {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Create {
    }
    /// Nested message and enum types in `Create`.
    pub mod create {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub relation: ::core::option::Option<super::super::Relation>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSupertype {
    }
    /// Nested message and enum types in `GetSupertype`.
    pub mod get_supertype {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub relation_type: ::core::option::Option<super::super::RelationType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetSupertype {
    }
    /// Nested message and enum types in `SetSupertype`.
    pub mod set_supertype {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub relation_type: ::core::option::Option<super::super::RelationType>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSupertypes {
    }
    /// Nested message and enum types in `GetSupertypes`.
    pub mod get_supertypes {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub relation_types: ::prost::alloc::vec::Vec<super::super::RelationType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSubtypes {
    }
    /// Nested message and enum types in `GetSubtypes`.
    pub mod get_subtypes {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub relation_types: ::prost::alloc::vec::Vec<super::super::RelationType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetInstances {
    }
    /// Nested message and enum types in `GetInstances`.
    pub mod get_instances {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub relations: ::prost::alloc::vec::Vec<super::super::Relation>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRelates {
    }
    /// Nested message and enum types in `GetRelates`.
    pub mod get_relates {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub role_types: ::prost::alloc::vec::Vec<super::super::RoleType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRelatesForRoleLabel {
    }
    /// Nested message and enum types in `GetRelatesForRoleLabel`.
    pub mod get_relates_for_role_label {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub role_type: ::core::option::Option<super::super::RoleType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRelatesOverridden {
    }
    /// Nested message and enum types in `GetRelatesOverridden`.
    pub mod get_relates_overridden {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub role_type: ::core::option::Option<super::super::RoleType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetRelates {
    }
    /// Nested message and enum types in `SetRelates`.
    pub mod set_relates {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
            #[prost(string, optional, tag="2")]
            pub overridden_label: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnsetRelates {
    }
    /// Nested message and enum types in `UnsetRelates`.
    pub mod unset_relates {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributeType {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(enumeration="attribute_type::ValueType", tag="2")]
    pub value_type: i32,
    #[prost(bool, tag="3")]
    pub is_root: bool,
    #[prost(bool, tag="4")]
    pub is_abstract: bool,
}
/// Nested message and enum types in `AttributeType`.
pub mod attribute_type {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Put {
    }
    /// Nested message and enum types in `Put`.
    pub mod put {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub value: ::core::option::Option<super::super::attribute::Value>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub attribute: ::core::option::Option<super::super::Attribute>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Get {
    }
    /// Nested message and enum types in `Get`.
    pub mod get {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub value: ::core::option::Option<super::super::attribute::Value>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub attribute: ::core::option::Option<super::super::Attribute>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSupertype {
    }
    /// Nested message and enum types in `GetSupertype`.
    pub mod get_supertype {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub attribute_type: ::core::option::Option<super::super::AttributeType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetSupertype {
    }
    /// Nested message and enum types in `SetSupertype`.
    pub mod set_supertype {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(message, optional, tag="1")]
            pub attribute_type: ::core::option::Option<super::super::AttributeType>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSupertypes {
    }
    /// Nested message and enum types in `GetSupertypes`.
    pub mod get_supertypes {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub attribute_types: ::prost::alloc::vec::Vec<super::super::AttributeType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetSubtypes {
    }
    /// Nested message and enum types in `GetSubtypes`.
    pub mod get_subtypes {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
            #[prost(enumeration="super::ValueType", optional, tag="2")]
            pub value_type: ::core::option::Option<i32>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub attribute_types: ::prost::alloc::vec::Vec<super::super::AttributeType>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetInstances {
    }
    /// Nested message and enum types in `GetInstances`.
    pub mod get_instances {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
            #[prost(enumeration="super::ValueType", optional, tag="2")]
            pub value_type: ::core::option::Option<i32>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub attributes: ::prost::alloc::vec::Vec<super::super::Attribute>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRegex {
    }
    /// Nested message and enum types in `GetRegex`.
    pub mod get_regex {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(string, tag="1")]
            pub regex: ::prost::alloc::string::String,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetRegex {
    }
    /// Nested message and enum types in `SetRegex`.
    pub mod set_regex {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub regex: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetOwners {
    }
    /// Nested message and enum types in `GetOwners`.
    pub mod get_owners {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(enumeration="super::super::r#type::Transitivity", tag="1")]
            pub transitivity: i32,
            #[prost(message, repeated, tag="2")]
            pub annotations: ::prost::alloc::vec::Vec<super::super::r#type::Annotation>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub thing_types: ::prost::alloc::vec::Vec<super::super::ThingType>,
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ValueType {
        Object = 0,
        Boolean = 1,
        Long = 2,
        Double = 3,
        String = 4,
        Datetime = 5,
    }
    impl ValueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueType::Object => "OBJECT",
                ValueType::Boolean => "BOOLEAN",
                ValueType::Long => "LONG",
                ValueType::Double => "DOUBLE",
                ValueType::String => "STRING",
                ValueType::Datetime => "DATETIME",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConceptMap {
    #[prost(map="string, message", tag="1")]
    pub map: ::std::collections::HashMap<::prost::alloc::string::String, Concept>,
    #[prost(message, optional, tag="2")]
    pub explainables: ::core::option::Option<Explainables>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Explainables {
    #[prost(map="string, message", tag="1")]
    pub relations: ::std::collections::HashMap<::prost::alloc::string::String, Explainable>,
    #[prost(map="string, message", tag="2")]
    pub attributes: ::std::collections::HashMap<::prost::alloc::string::String, Explainable>,
    #[prost(map="string, message", tag="3")]
    pub ownerships: ::std::collections::HashMap<::prost::alloc::string::String, explainables::Owned>,
}
/// Nested message and enum types in `Explainables`.
pub mod explainables {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Owned {
        #[prost(map="string, message", tag="1")]
        pub owned: ::std::collections::HashMap<::prost::alloc::string::String, super::Explainable>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Explainable {
    #[prost(string, tag="1")]
    pub conjunction: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConceptMapGroup {
    #[prost(message, optional, tag="1")]
    pub owner: ::core::option::Option<Concept>,
    #[prost(message, repeated, tag="2")]
    pub concept_maps: ::prost::alloc::vec::Vec<ConceptMap>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Numeric {
    #[prost(oneof="numeric::Value", tags="1, 2, 3")]
    pub value: ::core::option::Option<numeric::Value>,
}
/// Nested message and enum types in `Numeric`.
pub mod numeric {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(sint64, tag="1")]
        LongValue(i64),
        #[prost(double, tag="2")]
        DoubleValue(f64),
        #[prost(bool, tag="3")]
        Nan(bool),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumericGroup {
    #[prost(message, optional, tag="1")]
    pub owner: ::core::option::Option<Concept>,
    #[prost(message, optional, tag="2")]
    pub number: ::core::option::Option<Numeric>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicManager {
}
/// Nested message and enum types in `LogicManager`.
pub mod logic_manager {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Req {
        #[prost(oneof="req::Req", tags="1, 2, 3")]
        pub req: ::core::option::Option<req::Req>,
    }
    /// Nested message and enum types in `Req`.
    pub mod req {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Req {
            #[prost(message, tag="1")]
            GetRuleReq(super::get_rule::Req),
            #[prost(message, tag="2")]
            PutRuleReq(super::put_rule::Req),
            #[prost(message, tag="3")]
            GetRulesReq(super::get_rules::Req),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Res {
        #[prost(oneof="res::Res", tags="1, 2")]
        pub res: ::core::option::Option<res::Res>,
    }
    /// Nested message and enum types in `Res`.
    pub mod res {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="1")]
            GetRuleRes(super::get_rule::Res),
            #[prost(message, tag="2")]
            PutRuleRes(super::put_rule::Res),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResPart {
        #[prost(message, optional, tag="1")]
        pub get_rules_res_part: ::core::option::Option<get_rules::ResPart>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRule {
    }
    /// Nested message and enum types in `GetRule`.
    pub mod get_rule {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub rule: ::core::option::Option<super::super::Rule>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PutRule {
    }
    /// Nested message and enum types in `PutRule`.
    pub mod put_rule {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub when: ::prost::alloc::string::String,
            #[prost(string, tag="3")]
            pub then: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub rule: ::core::option::Option<super::super::Rule>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRules {
    }
    /// Nested message and enum types in `GetRules`.
    pub mod get_rules {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub rules: ::prost::alloc::vec::Vec<super::super::Rule>,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rule {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub when: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub then: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Rule`.
pub mod rule {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Req {
        #[prost(string, tag="1")]
        pub label: ::prost::alloc::string::String,
        #[prost(oneof="req::Req", tags="100, 101")]
        pub req: ::core::option::Option<req::Req>,
    }
    /// Nested message and enum types in `Req`.
    pub mod req {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Req {
            #[prost(message, tag="100")]
            RuleDeleteReq(super::delete::Req),
            #[prost(message, tag="101")]
            RuleSetLabelReq(super::set_label::Req),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Res {
        #[prost(oneof="res::Res", tags="100, 101")]
        pub res: ::core::option::Option<res::Res>,
    }
    /// Nested message and enum types in `Res`.
    pub mod res {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="100")]
            RuleDeleteRes(super::delete::Res),
            #[prost(message, tag="101")]
            RuleSetLabelRes(super::set_label::Res),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delete {
    }
    /// Nested message and enum types in `Delete`.
    pub mod delete {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetLabel {
    }
    /// Nested message and enum types in `SetLabel`.
    pub mod set_label {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub label: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Explanation {
    #[prost(message, optional, tag="1")]
    pub rule: ::core::option::Option<Rule>,
    #[prost(map="string, message", tag="2")]
    pub var_mapping: ::std::collections::HashMap<::prost::alloc::string::String, explanation::VarList>,
    #[prost(message, optional, tag="3")]
    pub condition: ::core::option::Option<ConceptMap>,
    #[prost(message, optional, tag="4")]
    pub conclusion: ::core::option::Option<ConceptMap>,
}
/// Nested message and enum types in `Explanation`.
pub mod explanation {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VarList {
        #[prost(string, repeated, tag="1")]
        pub vars: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryManager {
}
/// Nested message and enum types in `QueryManager`.
pub mod query_manager {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Req {
        #[prost(message, optional, tag="1")]
        pub options: ::core::option::Option<super::Options>,
        #[prost(oneof="req::Req", tags="100, 101, 102, 103, 104, 105, 106, 107, 108, 109")]
        pub req: ::core::option::Option<req::Req>,
    }
    /// Nested message and enum types in `Req`.
    pub mod req {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Req {
            #[prost(message, tag="100")]
            DefineReq(super::define::Req),
            #[prost(message, tag="101")]
            UndefineReq(super::undefine::Req),
            #[prost(message, tag="102")]
            MatchReq(super::r#match::Req),
            #[prost(message, tag="103")]
            MatchAggregateReq(super::match_aggregate::Req),
            #[prost(message, tag="104")]
            MatchGroupReq(super::match_group::Req),
            #[prost(message, tag="105")]
            MatchGroupAggregateReq(super::match_group_aggregate::Req),
            #[prost(message, tag="106")]
            InsertReq(super::insert::Req),
            #[prost(message, tag="107")]
            DeleteReq(super::delete::Req),
            #[prost(message, tag="108")]
            UpdateReq(super::update::Req),
            #[prost(message, tag="109")]
            ExplainReq(super::explain::Req),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Res {
        #[prost(oneof="res::Res", tags="100, 101, 102, 104")]
        pub res: ::core::option::Option<res::Res>,
    }
    /// Nested message and enum types in `Res`.
    pub mod res {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="100")]
            DefineRes(super::define::Res),
            #[prost(message, tag="101")]
            UndefineRes(super::undefine::Res),
            #[prost(message, tag="102")]
            MatchAggregateRes(super::match_aggregate::Res),
            #[prost(message, tag="104")]
            DeleteRes(super::delete::Res),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResPart {
        #[prost(oneof="res_part::Res", tags="100, 101, 102, 103, 104, 105")]
        pub res: ::core::option::Option<res_part::Res>,
    }
    /// Nested message and enum types in `ResPart`.
    pub mod res_part {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="100")]
            MatchResPart(super::r#match::ResPart),
            #[prost(message, tag="101")]
            MatchGroupResPart(super::match_group::ResPart),
            #[prost(message, tag="102")]
            MatchGroupAggregateResPart(super::match_group_aggregate::ResPart),
            #[prost(message, tag="103")]
            InsertResPart(super::insert::ResPart),
            #[prost(message, tag="104")]
            UpdateResPart(super::update::ResPart),
            #[prost(message, tag="105")]
            ExplainResPart(super::explain::ResPart),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Match {
    }
    /// Nested message and enum types in `Match`.
    pub mod r#match {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub query: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub answers: ::prost::alloc::vec::Vec<super::super::ConceptMap>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchAggregate {
    }
    /// Nested message and enum types in `MatchAggregate`.
    pub mod match_aggregate {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub query: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
            #[prost(message, optional, tag="1")]
            pub answer: ::core::option::Option<super::super::Numeric>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchGroup {
    }
    /// Nested message and enum types in `MatchGroup`.
    pub mod match_group {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub query: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub answers: ::prost::alloc::vec::Vec<super::super::ConceptMapGroup>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchGroupAggregate {
    }
    /// Nested message and enum types in `MatchGroupAggregate`.
    pub mod match_group_aggregate {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub query: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub answers: ::prost::alloc::vec::Vec<super::super::NumericGroup>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Explain {
    }
    /// Nested message and enum types in `Explain`.
    pub mod explain {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(int64, tag="1")]
            pub explainable_id: i64,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub explanations: ::prost::alloc::vec::Vec<super::super::Explanation>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Insert {
    }
    /// Nested message and enum types in `Insert`.
    pub mod insert {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub query: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub answers: ::prost::alloc::vec::Vec<super::super::ConceptMap>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delete {
    }
    /// Nested message and enum types in `Delete`.
    pub mod delete {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub query: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Update {
    }
    /// Nested message and enum types in `Update`.
    pub mod update {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub query: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(message, repeated, tag="1")]
            pub answers: ::prost::alloc::vec::Vec<super::super::ConceptMap>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Define {
    }
    /// Nested message and enum types in `Define`.
    pub mod define {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub query: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Undefine {
    }
    /// Nested message and enum types in `Undefine`.
    pub mod undefine {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(string, tag="1")]
            pub query: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
}
/// Nested message and enum types in `Transaction`.
pub mod transaction {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Client {
        #[prost(message, repeated, tag="1")]
        pub reqs: ::prost::alloc::vec::Vec<Req>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Server {
        #[prost(oneof="server::Server", tags="2, 3")]
        pub server: ::core::option::Option<server::Server>,
    }
    /// Nested message and enum types in `Server`.
    pub mod server {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Server {
            #[prost(message, tag="2")]
            Res(super::Res),
            #[prost(message, tag="3")]
            ResPart(super::ResPart),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Req {
        #[prost(bytes="vec", tag="1")]
        pub req_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(map="string, string", tag="2")]
        pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
        #[prost(oneof="req::Req", tags="3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
        pub req: ::core::option::Option<req::Req>,
    }
    /// Nested message and enum types in `Req`.
    pub mod req {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Req {
            #[prost(message, tag="3")]
            OpenReq(super::open::Req),
            #[prost(message, tag="4")]
            StreamReq(super::stream::Req),
            #[prost(message, tag="5")]
            CommitReq(super::commit::Req),
            #[prost(message, tag="6")]
            RollbackReq(super::rollback::Req),
            #[prost(message, tag="7")]
            QueryManagerReq(super::super::query_manager::Req),
            #[prost(message, tag="8")]
            ConceptManagerReq(super::super::concept_manager::Req),
            #[prost(message, tag="9")]
            LogicManagerReq(super::super::logic_manager::Req),
            #[prost(message, tag="10")]
            RuleReq(super::super::rule::Req),
            #[prost(message, tag="11")]
            TypeReq(super::super::r#type::Req),
            #[prost(message, tag="12")]
            ThingReq(super::super::thing::Req),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Res {
        #[prost(bytes="vec", tag="1")]
        pub req_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(oneof="res::Res", tags="2, 3, 4, 5, 6, 7, 8, 9, 10")]
        pub res: ::core::option::Option<res::Res>,
    }
    /// Nested message and enum types in `Res`.
    pub mod res {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="2")]
            OpenRes(super::open::Res),
            #[prost(message, tag="3")]
            CommitRes(super::commit::Res),
            #[prost(message, tag="4")]
            RollbackRes(super::rollback::Res),
            #[prost(message, tag="5")]
            QueryManagerRes(super::super::query_manager::Res),
            #[prost(message, tag="6")]
            ConceptManagerRes(super::super::concept_manager::Res),
            #[prost(message, tag="7")]
            LogicManagerRes(super::super::logic_manager::Res),
            #[prost(message, tag="8")]
            RuleRes(super::super::rule::Res),
            #[prost(message, tag="9")]
            TypeRes(super::super::r#type::Res),
            #[prost(message, tag="10")]
            ThingRes(super::super::thing::Res),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResPart {
        #[prost(bytes="vec", tag="1")]
        pub req_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(oneof="res_part::Res", tags="2, 3, 4, 5, 6")]
        pub res: ::core::option::Option<res_part::Res>,
    }
    /// Nested message and enum types in `ResPart`.
    pub mod res_part {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Res {
            #[prost(message, tag="2")]
            StreamResPart(super::stream::ResPart),
            #[prost(message, tag="3")]
            QueryManagerResPart(super::super::query_manager::ResPart),
            #[prost(message, tag="4")]
            LogicManagerResPart(super::super::logic_manager::ResPart),
            #[prost(message, tag="5")]
            TypeResPart(super::super::r#type::ResPart),
            #[prost(message, tag="6")]
            ThingResPart(super::super::thing::ResPart),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Open {
    }
    /// Nested message and enum types in `Open`.
    pub mod open {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
            #[prost(bytes="vec", tag="1")]
            pub session_id: ::prost::alloc::vec::Vec<u8>,
            #[prost(enumeration="super::Type", tag="2")]
            pub r#type: i32,
            #[prost(message, optional, tag="3")]
            pub options: ::core::option::Option<super::super::Options>,
            #[prost(int32, tag="4")]
            pub network_latency_millis: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Stream {
    }
    /// Nested message and enum types in `Stream`.
    pub mod stream {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResPart {
            #[prost(enumeration="State", tag="1")]
            pub state: i32,
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum State {
            Continue = 0,
            Done = 1,
        }
        impl State {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    State::Continue => "CONTINUE",
                    State::Done => "DONE",
                }
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Commit {
    }
    /// Nested message and enum types in `Commit`.
    pub mod commit {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rollback {
    }
    /// Nested message and enum types in `Rollback`.
    pub mod rollback {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Req {
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Res {
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Read = 0,
        Write = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Read => "READ",
                Type::Write => "WRITE",
            }
        }
    }
}
/// Generated client implementations.
pub mod type_db_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TypeDbClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TypeDbClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TypeDbClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TypeDbClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            TypeDbClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Server Manager API
        pub async fn servers_all(
            &mut self,
            request: impl tonic::IntoRequest<super::server_manager::all::Req>,
        ) -> Result<tonic::Response<super::server_manager::all::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/servers_all",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// User Manager API
        pub async fn users_contains(
            &mut self,
            request: impl tonic::IntoRequest<super::user_manager::contains::Req>,
        ) -> Result<tonic::Response<super::user_manager::contains::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/users_contains",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn users_create(
            &mut self,
            request: impl tonic::IntoRequest<super::user_manager::create::Req>,
        ) -> Result<tonic::Response<super::user_manager::create::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/users_create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn users_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::user_manager::delete::Req>,
        ) -> Result<tonic::Response<super::user_manager::delete::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/users_delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn users_all(
            &mut self,
            request: impl tonic::IntoRequest<super::user_manager::all::Req>,
        ) -> Result<tonic::Response<super::user_manager::all::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/users_all",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn users_password_set(
            &mut self,
            request: impl tonic::IntoRequest<super::user_manager::password_set::Req>,
        ) -> Result<
            tonic::Response<super::user_manager::password_set::Res>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/users_password_set",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn users_get(
            &mut self,
            request: impl tonic::IntoRequest<super::user_manager::get::Req>,
        ) -> Result<tonic::Response<super::user_manager::get::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/users_get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// User API
        pub async fn user_password_update(
            &mut self,
            request: impl tonic::IntoRequest<super::user::password_update::Req>,
        ) -> Result<tonic::Response<super::user::password_update::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/user_password_update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn user_token(
            &mut self,
            request: impl tonic::IntoRequest<super::user::token::Req>,
        ) -> Result<tonic::Response<super::user::token::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/user_token",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Database Manager API
        pub async fn databases_get(
            &mut self,
            request: impl tonic::IntoRequest<super::database_manager::get::Req>,
        ) -> Result<tonic::Response<super::database_manager::get::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/databases_get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn databases_all(
            &mut self,
            request: impl tonic::IntoRequest<super::database_manager::all::Req>,
        ) -> Result<tonic::Response<super::database_manager::all::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/databases_all",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn databases_contains(
            &mut self,
            request: impl tonic::IntoRequest<super::database_manager::contains::Req>,
        ) -> Result<
            tonic::Response<super::database_manager::contains::Res>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/databases_contains",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn databases_create(
            &mut self,
            request: impl tonic::IntoRequest<super::database_manager::create::Req>,
        ) -> Result<
            tonic::Response<super::database_manager::create::Res>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/databases_create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Database API
        pub async fn database_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::database::schema::Req>,
        ) -> Result<tonic::Response<super::database::schema::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/database_schema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn database_type_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::database::type_schema::Req>,
        ) -> Result<tonic::Response<super::database::type_schema::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/database_type_schema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn database_rule_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::database::rule_schema::Req>,
        ) -> Result<tonic::Response<super::database::rule_schema::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/database_rule_schema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn database_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::database::delete::Req>,
        ) -> Result<tonic::Response<super::database::delete::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/database_delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Session API
        pub async fn session_open(
            &mut self,
            request: impl tonic::IntoRequest<super::session::open::Req>,
        ) -> Result<tonic::Response<super::session::open::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/session_open",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn session_close(
            &mut self,
            request: impl tonic::IntoRequest<super::session::close::Req>,
        ) -> Result<tonic::Response<super::session::close::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/session_close",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Checks with the server that the session is still alive, and informs it that it should be kept alive.
        pub async fn session_pulse(
            &mut self,
            request: impl tonic::IntoRequest<super::session::pulse::Req>,
        ) -> Result<tonic::Response<super::session::pulse::Res>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/session_pulse",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Transaction Streaming API
        /// Opens a bi-directional stream representing a stateful transaction, streaming
        /// requests and responses back-and-forth. The first transaction client message must
        /// be {Transaction.Open.Req}. Closing the stream closes the transaction.
        pub async fn transaction(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::transaction::Client,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::transaction::Server>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/typedb.protocol.TypeDB/transaction",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod type_db_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with TypeDbServer.
    #[async_trait]
    pub trait TypeDb: Send + Sync + 'static {
        /// Server Manager API
        async fn servers_all(
            &self,
            request: tonic::Request<super::server_manager::all::Req>,
        ) -> Result<tonic::Response<super::server_manager::all::Res>, tonic::Status>;
        /// User Manager API
        async fn users_contains(
            &self,
            request: tonic::Request<super::user_manager::contains::Req>,
        ) -> Result<tonic::Response<super::user_manager::contains::Res>, tonic::Status>;
        async fn users_create(
            &self,
            request: tonic::Request<super::user_manager::create::Req>,
        ) -> Result<tonic::Response<super::user_manager::create::Res>, tonic::Status>;
        async fn users_delete(
            &self,
            request: tonic::Request<super::user_manager::delete::Req>,
        ) -> Result<tonic::Response<super::user_manager::delete::Res>, tonic::Status>;
        async fn users_all(
            &self,
            request: tonic::Request<super::user_manager::all::Req>,
        ) -> Result<tonic::Response<super::user_manager::all::Res>, tonic::Status>;
        async fn users_password_set(
            &self,
            request: tonic::Request<super::user_manager::password_set::Req>,
        ) -> Result<
            tonic::Response<super::user_manager::password_set::Res>,
            tonic::Status,
        >;
        async fn users_get(
            &self,
            request: tonic::Request<super::user_manager::get::Req>,
        ) -> Result<tonic::Response<super::user_manager::get::Res>, tonic::Status>;
        /// User API
        async fn user_password_update(
            &self,
            request: tonic::Request<super::user::password_update::Req>,
        ) -> Result<tonic::Response<super::user::password_update::Res>, tonic::Status>;
        async fn user_token(
            &self,
            request: tonic::Request<super::user::token::Req>,
        ) -> Result<tonic::Response<super::user::token::Res>, tonic::Status>;
        /// Database Manager API
        async fn databases_get(
            &self,
            request: tonic::Request<super::database_manager::get::Req>,
        ) -> Result<tonic::Response<super::database_manager::get::Res>, tonic::Status>;
        async fn databases_all(
            &self,
            request: tonic::Request<super::database_manager::all::Req>,
        ) -> Result<tonic::Response<super::database_manager::all::Res>, tonic::Status>;
        async fn databases_contains(
            &self,
            request: tonic::Request<super::database_manager::contains::Req>,
        ) -> Result<
            tonic::Response<super::database_manager::contains::Res>,
            tonic::Status,
        >;
        async fn databases_create(
            &self,
            request: tonic::Request<super::database_manager::create::Req>,
        ) -> Result<
            tonic::Response<super::database_manager::create::Res>,
            tonic::Status,
        >;
        /// Database API
        async fn database_schema(
            &self,
            request: tonic::Request<super::database::schema::Req>,
        ) -> Result<tonic::Response<super::database::schema::Res>, tonic::Status>;
        async fn database_type_schema(
            &self,
            request: tonic::Request<super::database::type_schema::Req>,
        ) -> Result<tonic::Response<super::database::type_schema::Res>, tonic::Status>;
        async fn database_rule_schema(
            &self,
            request: tonic::Request<super::database::rule_schema::Req>,
        ) -> Result<tonic::Response<super::database::rule_schema::Res>, tonic::Status>;
        async fn database_delete(
            &self,
            request: tonic::Request<super::database::delete::Req>,
        ) -> Result<tonic::Response<super::database::delete::Res>, tonic::Status>;
        /// Session API
        async fn session_open(
            &self,
            request: tonic::Request<super::session::open::Req>,
        ) -> Result<tonic::Response<super::session::open::Res>, tonic::Status>;
        async fn session_close(
            &self,
            request: tonic::Request<super::session::close::Req>,
        ) -> Result<tonic::Response<super::session::close::Res>, tonic::Status>;
        /// Checks with the server that the session is still alive, and informs it that it should be kept alive.
        async fn session_pulse(
            &self,
            request: tonic::Request<super::session::pulse::Req>,
        ) -> Result<tonic::Response<super::session::pulse::Res>, tonic::Status>;
        ///Server streaming response type for the transaction method.
        type transactionStream: futures_core::Stream<
                Item = Result<super::transaction::Server, tonic::Status>,
            >
            + Send
            + 'static;
        /// Transaction Streaming API
        /// Opens a bi-directional stream representing a stateful transaction, streaming
        /// requests and responses back-and-forth. The first transaction client message must
        /// be {Transaction.Open.Req}. Closing the stream closes the transaction.
        async fn transaction(
            &self,
            request: tonic::Request<tonic::Streaming<super::transaction::Client>>,
        ) -> Result<tonic::Response<Self::transactionStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TypeDbServer<T: TypeDb> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TypeDb> TypeDbServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TypeDbServer<T>
    where
        T: TypeDb,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/typedb.protocol.TypeDB/servers_all" => {
                    #[allow(non_camel_case_types)]
                    struct servers_allSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::server_manager::all::Req>
                    for servers_allSvc<T> {
                        type Response = super::server_manager::all::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::server_manager::all::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).servers_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = servers_allSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/users_contains" => {
                    #[allow(non_camel_case_types)]
                    struct users_containsSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::user_manager::contains::Req>
                    for users_containsSvc<T> {
                        type Response = super::user_manager::contains::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::user_manager::contains::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).users_contains(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = users_containsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/users_create" => {
                    #[allow(non_camel_case_types)]
                    struct users_createSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::user_manager::create::Req>
                    for users_createSvc<T> {
                        type Response = super::user_manager::create::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::user_manager::create::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).users_create(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = users_createSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/users_delete" => {
                    #[allow(non_camel_case_types)]
                    struct users_deleteSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::user_manager::delete::Req>
                    for users_deleteSvc<T> {
                        type Response = super::user_manager::delete::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::user_manager::delete::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).users_delete(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = users_deleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/users_all" => {
                    #[allow(non_camel_case_types)]
                    struct users_allSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::user_manager::all::Req>
                    for users_allSvc<T> {
                        type Response = super::user_manager::all::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::user_manager::all::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).users_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = users_allSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/users_password_set" => {
                    #[allow(non_camel_case_types)]
                    struct users_password_setSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::user_manager::password_set::Req>
                    for users_password_setSvc<T> {
                        type Response = super::user_manager::password_set::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::user_manager::password_set::Req,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).users_password_set(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = users_password_setSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/users_get" => {
                    #[allow(non_camel_case_types)]
                    struct users_getSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::user_manager::get::Req>
                    for users_getSvc<T> {
                        type Response = super::user_manager::get::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::user_manager::get::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).users_get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = users_getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/user_password_update" => {
                    #[allow(non_camel_case_types)]
                    struct user_password_updateSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::user::password_update::Req>
                    for user_password_updateSvc<T> {
                        type Response = super::user::password_update::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::user::password_update::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).user_password_update(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = user_password_updateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/user_token" => {
                    #[allow(non_camel_case_types)]
                    struct user_tokenSvc<T: TypeDb>(pub Arc<T>);
                    impl<T: TypeDb> tonic::server::UnaryService<super::user::token::Req>
                    for user_tokenSvc<T> {
                        type Response = super::user::token::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::user::token::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).user_token(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = user_tokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/databases_get" => {
                    #[allow(non_camel_case_types)]
                    struct databases_getSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::database_manager::get::Req>
                    for databases_getSvc<T> {
                        type Response = super::database_manager::get::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::database_manager::get::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).databases_get(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = databases_getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/databases_all" => {
                    #[allow(non_camel_case_types)]
                    struct databases_allSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::database_manager::all::Req>
                    for databases_allSvc<T> {
                        type Response = super::database_manager::all::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::database_manager::all::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).databases_all(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = databases_allSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/databases_contains" => {
                    #[allow(non_camel_case_types)]
                    struct databases_containsSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::database_manager::contains::Req>
                    for databases_containsSvc<T> {
                        type Response = super::database_manager::contains::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::database_manager::contains::Req,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).databases_contains(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = databases_containsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/databases_create" => {
                    #[allow(non_camel_case_types)]
                    struct databases_createSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::database_manager::create::Req>
                    for databases_createSvc<T> {
                        type Response = super::database_manager::create::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::database_manager::create::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).databases_create(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = databases_createSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/database_schema" => {
                    #[allow(non_camel_case_types)]
                    struct database_schemaSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::database::schema::Req>
                    for database_schemaSvc<T> {
                        type Response = super::database::schema::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::database::schema::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).database_schema(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = database_schemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/database_type_schema" => {
                    #[allow(non_camel_case_types)]
                    struct database_type_schemaSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::database::type_schema::Req>
                    for database_type_schemaSvc<T> {
                        type Response = super::database::type_schema::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::database::type_schema::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).database_type_schema(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = database_type_schemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/database_rule_schema" => {
                    #[allow(non_camel_case_types)]
                    struct database_rule_schemaSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::database::rule_schema::Req>
                    for database_rule_schemaSvc<T> {
                        type Response = super::database::rule_schema::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::database::rule_schema::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).database_rule_schema(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = database_rule_schemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/database_delete" => {
                    #[allow(non_camel_case_types)]
                    struct database_deleteSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::database::delete::Req>
                    for database_deleteSvc<T> {
                        type Response = super::database::delete::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::database::delete::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).database_delete(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = database_deleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/session_open" => {
                    #[allow(non_camel_case_types)]
                    struct session_openSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::session::open::Req>
                    for session_openSvc<T> {
                        type Response = super::session::open::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::session::open::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).session_open(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = session_openSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/session_close" => {
                    #[allow(non_camel_case_types)]
                    struct session_closeSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::session::close::Req>
                    for session_closeSvc<T> {
                        type Response = super::session::close::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::session::close::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).session_close(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = session_closeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/session_pulse" => {
                    #[allow(non_camel_case_types)]
                    struct session_pulseSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::UnaryService<super::session::pulse::Req>
                    for session_pulseSvc<T> {
                        type Response = super::session::pulse::Res;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::session::pulse::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).session_pulse(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = session_pulseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/typedb.protocol.TypeDB/transaction" => {
                    #[allow(non_camel_case_types)]
                    struct transactionSvc<T: TypeDb>(pub Arc<T>);
                    impl<
                        T: TypeDb,
                    > tonic::server::StreamingService<super::transaction::Client>
                    for transactionSvc<T> {
                        type Response = super::transaction::Server;
                        type ResponseStream = T::transactionStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::transaction::Client>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = transactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: TypeDb> Clone for TypeDbServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TypeDb> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TypeDb> tonic::server::NamedService for TypeDbServer<T> {
        const NAME: &'static str = "typedb.protocol.TypeDB";
    }
}
