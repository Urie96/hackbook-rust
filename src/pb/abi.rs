#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Course {
    #[prost(uint32, tag="1")]
    pub article_count: u32,
    #[prost(string, tag="2")]
    pub brief: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub done: bool,
    #[prost(string, tag="4")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub image: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub price: u32,
    #[prost(string, tag="7")]
    pub purchased_count: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub teacher_name: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub teacher_title: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub title: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="11")]
    pub sections: ::prost::alloc::vec::Vec<Section>,
    #[prost(string, tag="12")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag="13")]
    pub study_info: ::core::option::Option<StudyInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StudyInfo {
    #[prost(float, tag="1")]
    pub percent: f32,
    #[prost(uint64, tag="2")]
    pub last_study_at: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCourseResponse {
    #[prost(message, repeated, tag="1")]
    pub courses: ::prost::alloc::vec::Vec<Course>,
    #[prost(bool, tag="2")]
    pub more: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Section {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub articles: ::prost::alloc::vec::Vec<Article>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SectionList {
    #[prost(message, repeated, tag="1")]
    pub sections: ::prost::alloc::vec::Vec<Section>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Article {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub publish_date: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub done: bool,
    #[prost(string, tag="5")]
    pub content: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub course: ::core::option::Option<Course>,
    #[prost(message, optional, tag="7")]
    pub section: ::core::option::Option<Section>,
    #[prost(message, optional, tag="8")]
    pub study_info: ::core::option::Option<StudyInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArticleList {
    #[prost(message, repeated, tag="1")]
    pub articles: ::prost::alloc::vec::Vec<Article>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    #[prost(string, tag="1")]
    pub content: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub like_count: u32,
    #[prost(string, tag="3")]
    pub nick_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub replies: ::prost::alloc::vec::Vec<Comment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentList {
    #[prost(message, repeated, tag="1")]
    pub comments: ::prost::alloc::vec::Vec<Comment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="UserRole", tag="2")]
    pub role: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveStudyInfoRequest {
    #[prost(string, tag="1")]
    pub article_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub course_id: ::prost::alloc::string::String,
    #[prost(float, tag="3")]
    pub percent: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserRole {
    Visitor = 0,
    Reader = 1,
}
impl UserRole {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserRole::Visitor => "Visitor",
            UserRole::Reader => "Reader",
        }
    }
}
