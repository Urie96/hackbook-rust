// @generated automatically by Diesel CLI.

diesel::table! {
    article (id) {
        id -> Varchar,
        done -> Tinyint,
        publishDate -> Varchar,
        sectionId -> Varchar,
        title -> Varchar,
    }
}

diesel::table! {
    article_comment (id) {
        id -> Varchar,
        content -> Text,
        likeCount -> Integer,
        nickName -> Varchar,
        articleId -> Nullable<Varchar>,
        parentCommentId -> Nullable<Varchar>,
    }
}

diesel::table! {
    article_content (articleId) {
        articleId -> Varchar,
        content -> Mediumtext,
    }
}

diesel::table! {
    course (id) {
        id -> Varchar,
        brief -> Varchar,
        teacherName -> Varchar,
        teacherTitle -> Varchar,
        image -> Varchar,
        articleCount -> Integer,
        purchasedCount -> Varchar,
        done -> Tinyint,
        price -> Integer,
        title -> Varchar,
    }
}

diesel::table! {
    course_description (courseId) {
        courseId -> Varchar,
        content -> Text,
    }
}

diesel::table! {
    section (id) {
        id -> Varchar,
        courseId -> Varchar,
        title -> Varchar,
    }
}

diesel::table! {
    user_role (id) {
        user_id -> Varchar,
        role -> Unsigned<Integer>,
        created_at -> Unsigned<Bigint>,
        valid_before -> Unsigned<Bigint>,
        id -> Unsigned<Bigint>,
    }
}

diesel::table! {
    user_study_info (id) {
        id -> Unsigned<Integer>,
        course_id -> Varchar,
        article_id -> Varchar,
        last_study_at -> Unsigned<Bigint>,
        study_percent -> Unsigned<Smallint>,
        user_id -> Varchar,
    }
}

diesel::joinable!(article -> section (sectionId));
diesel::joinable!(article_comment -> article (articleId));
diesel::joinable!(article_content -> article (articleId));
diesel::joinable!(course_description -> course (courseId));
diesel::joinable!(section -> course (courseId));
diesel::joinable!(user_study_info -> article (article_id));
diesel::joinable!(user_study_info -> course (course_id));

diesel::allow_tables_to_appear_in_same_query!(
    article,
    article_comment,
    article_content,
    course,
    course_description,
    section,
    user_role,
    user_study_info,
);
